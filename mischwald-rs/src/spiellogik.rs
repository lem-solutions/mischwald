use smallvec::SmallVec;

use crate::datentypen::*;
use crate::spieler::*;
use crate::spielstand::*;
use crate::Spieler;

/// `None` bedeutet das Spiel ist zuende.
type OptionSpielende<T> = Option<T>;

/// Typalias um die Bedeutung von bestimmten Rückgabewerten zu verdeutlichen.
type EffektGenutzt = bool;

/// Manche Bedingungen hängen von der Position der Karte auf der die Bedingung
/// steht ab. `Bedingungskontext` beschreibt diese Position
#[derive(Copy, Clone)]
struct Bedingungskontext {
	pub(crate) hauptpflanze_idx: usize,
	pub(crate) kartenpos: Option<Kartenposition>,
}

struct Zugstatus {
	extrazug : bool,
	// Dauereffekte treten, mit der Ausnahme von Streuchern, wenn deren Bonus eingesetzt wird,
	// erst im nächsten Spielzug in Kraft. Dauereffekte die im aktuellen Spielzug
	// noch nicht in Kraft sind werden hier zuwischengespeichert.
	zurückgehaltene_dauereffekte: SmallVec<[&'static Dauereffekt;1]>
}
impl Default for Zugstatus {
	fn default() -> Self {
		Zugstatus { extrazug: false, zurückgehaltene_dauereffekte: SmallVec::new()}
	}
}

impl Spielstand {
	/// Führt einen Zug für den aktuellen Spieler durch. Der Aufrufer muss
	/// mittels `Spielstand::nächster_spieler` ermitteln welcher Spieler dran ist falls
	/// verschidene Spielerinstanzen/Implementierungen benutzt werden.
	pub fn zug<S: Spieler>(&mut self, spieler: &mut S) {
		// wir ignorieren alle `OptionSpielende`, da wir in keinem Fall hier etwas 
		// tun müssen.
		assert!(!self.ist_spiel_beendet());
		if self.erste_runde {
			self.erste_runde_logik(spieler);
			self.zug_beendet_nächster_spieler();
			return;
		}
		
		let mut zugstatus = Zugstatus::default();
		
		match spieler.zugkation(self) {
			Zugaktion::KarteAusspielen(ausspiel_info) => {
				let _ = self.karte_bezahlen_und_ausspielen(
					spieler, &mut zugstatus, ausspiel_info);
			},
			Zugaktion::KartenZiehen(ziehen_von1, ziehen_von2_opt) => {
				for ziehen_von_opt in [Some(ziehen_von1), ziehen_von2_opt] {
					if ziehen_von_opt.is_none() { continue; }
					let ziehen_von = ziehen_von_opt.unwrap();
					let _ = self.karte_ziehen_auswahl(spieler, ziehen_von);
				}
			},
			Zugaktion::SetzlingAusspielen { karten_idx } => {
				let _ = self.setzling_auspielen(karten_idx);
			}
		}
		
		self.nächster_spielerstand_mut().dauereffekte.extend_from_slice(
			zugstatus.zurückgehaltene_dauereffekte.as_slice());
		
		if !self.ist_spiel_beendet() && zugstatus.extrazug {
			self.zug(spieler);
		} else {
			self.zug_beendet_nächster_spieler();
		}
	}
	
	#[must_use]
	fn setzling_auspielen(&mut self, karten_idx: usize) -> OptionSpielende<()> {
		self.nächster_spielerstand_mut()
			.handkarten
			.remove(karten_idx)
			.unwrap();
		
		self.nächster_spielerstand_mut()
			.wald
			.push(Hauptpflanze::neu(&crate::karten::SETZLING));
		
		self.karte_stapel_auf_lichtung()
	}
	
	#[must_use]
	fn karte_stapel_auf_lichtung(&mut self) -> OptionSpielende<()> {
		let karte = self.karte_ziehen()?;
		self.lichtung.push(karte);
		Some(())
	}
	
	#[must_use]
	fn karte_bezahlen_und_ausspielen<S: Spieler>(
		&mut self,
		spieler: &mut S,
		zugstatus: &mut Zugstatus,
		ausspiel_info: Ausspielen,
	) -> OptionSpielende<()> {
		assert!(ausspiel_info.karten_idx < self.nächster_spielerstand().handkarten.len());
		let karte = self.nächster_spielerstand_mut().handkarten.remove(ausspiel_info.karten_idx).unwrap();
		match ausspiel_info.ausspielziel {
			Ausspielziel::NeueHauptpflanze => {
				assert!(matches!(karte, GanzeKarte::Hauptpflanze(_)));
				if let GanzeKarte::Hauptpflanze(k) = karte {
					let passend = self.bezahlen(spieler, k.kosten, k.baumsymbol);
					self.hauptpflanze_ausspielen(spieler, zugstatus, k, passend, false)?;
				} else {
					panic!("Ausspielziel::NeueHauptfplanze, aber Karte ist keine Hauptpflanze");
				}
			},
			Ausspielziel::AnHauptpflanzeAnlegen{pos, ..} => {
				let karte = match (pos, karte) {
					(Kartenposition::Oben, GanzeKarte::ZweigeteiltV { oben: karte_oben, unten : _ }) => karte_oben,
					(Kartenposition::Unten, GanzeKarte::ZweigeteiltV { oben: _, unten : karte_unten }) => karte_unten,
					(Kartenposition::Links, GanzeKarte::ZweigeteiltH { links: karte_links, rechts: _ }) => karte_links,
					(Kartenposition::Rechts, GanzeKarte::ZweigeteiltH { links: _, rechts: karte_rechts }) => karte_rechts,
					_ => panic!("Ungültige Kombination aus Kartenposition und Karte"),
				};
				let passend = self.bezahlen(spieler, karte.kosten, karte.baumsymbol);
				self.anlegekarte_ausspielen(spieler, zugstatus, karte, ausspiel_info.ausspielziel, passend, false)?;
			},
		}
		
		Some(())
	}
	
	#[must_use]
	fn hauptpflanze_ausspielen<S: Spieler>(
		&mut self,
		spieler: &mut S,
		zugstatus: &mut Zugstatus,
		karte: &'static Karte,
		passend: bool,
		// nur bein kostenlosen ausspielen ohne Boni und Soforteffekte `true`
		boni_und_soforteffekte_deaktiviert: bool,
	) -> OptionSpielende<()> {
		let bedingungskontext = Bedingungskontext {
			hauptpflanze_idx: self.nächster_spielerstand().wald.len(),
			kartenpos: None,
		};
		self.nächster_spielerstand_mut().wald.push(Hauptpflanze::neu(karte));
		
		self.nach_ausspielen_allgemein(spieler, zugstatus, karte, Ausspielziel::NeueHauptpflanze, passend, boni_und_soforteffekte_deaktiviert, bedingungskontext)
	}
	
	#[must_use]
	fn anlegekarte_ausspielen<S: Spieler>(
		&mut self,
		spieler: &mut S,
		zugstatus: &mut Zugstatus,
		karte: &'static Karte,
		ziel: Ausspielziel,
		passend: bool,
		// nur bein kostenlosen ausspielen ohne Boni und Soforteffekte `true`
		boni_und_soforteffekte_deaktiviert: bool,
	) -> OptionSpielende<()> {
		let (hauptpflanze_idx, pos) = match ziel {
			Ausspielziel::NeueHauptpflanze => panic!(
				"ungültiges Ausspielziel `Ausspielziel::NeueHauptpflanze` für Anlegekarte."),
			Ausspielziel::AnHauptpflanzeAnlegen { hauptpflanze_idx, pos } =>
				(hauptpflanze_idx, pos),
		};
		let bedingungskontext = Bedingungskontext {
			hauptpflanze_idx,
			kartenpos: Some(pos),
		};
		
		assert!(self.anlegekarte_prüfen(karte, hauptpflanze_idx, pos));
		
		self.nächster_spielerstand_mut()
			.wald[hauptpflanze_idx]
			.seite_mut(pos)
			.push(karte);
		
		self.nach_ausspielen_allgemein(spieler, zugstatus, karte, ziel, passend, boni_und_soforteffekte_deaktiviert, bedingungskontext)
	}
	
	// TODO zu viele Einrückungsstufen und generell zu Komplex; in kleinere
	//      Funktionen unterteilen.
	#[must_use]
	fn nach_ausspielen_allgemein<S: Spieler>(
		&mut self,
		spieler: &mut S,
		zugstatus: &mut Zugstatus,
		karte: &'static Karte,
		ziel: Ausspielziel,
		passend: bool,
		// nur bein kostenlosen ausspielen ohne Boni und Soforteffekte `true`
		boni_und_soforteffekte_deaktiviert: bool,
		bedingungskontext: Bedingungskontext,
	) -> OptionSpielende<()> {
		
		if karte.typen.contains(&Typsymbol::Baum) {
			self.karte_stapel_auf_lichtung()?;
		}
		
		// REGELFRAGE sollte der Spieler sich aussuchen können in welcher
		//            Reihenfolge die Dauereffekte ausgelöst werden? Wahrscheinlich
		//            ja, aber dieses Detail ist nicht unbedingt den
		//            Implementierungsaufwand wert.
		for dauereffekt in self.nächster_spielerstand().dauereffekte.clone() {
			match dauereffekt {
				Dauereffekt::Keiner => unreachable!("`Dauereffekt::Keiner` sollte nie in `Spielerstand::dauereffekte` auftauchen"),
				Dauereffekt::BeiKarteAusspielenTyp(typsymbol, effekt) => {
					if karte.typen.contains(typsymbol) {
						self.effekt_ausüben(spieler, zugstatus, effekt, bedingungskontext)?;
					}
				},
				Dauereffekt::BeiKarteAusspielenPositionTyp(
					pos_bedingung, hp_typ, effekt
				) => match ziel {
					Ausspielziel::NeueHauptpflanze => {},
					Ausspielziel::AnHauptpflanzeAnlegen { hauptpflanze_idx, ref pos } => {
						let hauptfplanze = 
							&self.nächster_spielerstand().wald[hauptpflanze_idx].hauptpflanze;
						if pos == pos_bedingung && hauptfplanze.typen.contains(hp_typ) {
							self.effekt_ausüben(
								spieler,
								zugstatus,
								effekt,
								bedingungskontext)?;
						}
					}
				}
			}
		}
		
		let mut dauereffekt_zurückgehalten_einfügen = true;
		if !boni_und_soforteffekte_deaktiviert {
			self.effekt_ausüben(
				spieler, zugstatus, &karte.soforteffekt, bedingungskontext)?;
			if passend {
				// Sträucher haben eine Sonderregel die besagt, das deren Dauereffekt
				// bereits in Kraft tritt, wenn der Bonus genutzt wird.
				// Wir implementieren das so, das wir vor dem Evaluieren des Bonus den
				// Dauereffekt aktivieren und, sollte der Bonus nicht genutzt worden
				// sein wieder deaktivieren und via 
				// `Zugstatus::zurückgehaltene_dauereffekte` im nächsten Zug zu
				// aktivieren.
				if karte.typen.contains(&Typsymbol::Strauch) &&
					karte.dauereffekt.ist_einer()
				{
					// Siehe Kommentar für `Self::effekt_ausüben`.
					assert!(matches!(
						karte.dauereffekt.endeffekt(),
						Effekt::EinsKostenlosAblegen(_)
					));
					let dauereffekt_idx = self.nächster_spielerstand_mut()
						.dauereffekte.len();
					self.nächster_spielerstand_mut().dauereffekte.push(
						&karte.dauereffekt);
				
					let genutzt = self.effekt_ausüben(
						spieler, zugstatus, &karte.bonus, bedingungskontext)?;
					
					if genutzt {
						dauereffekt_zurückgehalten_einfügen = false;
					} else {
						// Wir können nicht einfach `.pop()`en, da durch das ausüben des
						// Bonus theoretisch weitere Dauereffekte dazukommen könnten.
						self.nächster_spielerstand_mut().dauereffekte
							.swap_remove(dauereffekt_idx);
					}
				}
			}
		}
		
		if dauereffekt_zurückgehalten_einfügen && karte.dauereffekt.ist_einer() {
			zugstatus.zurückgehaltene_dauereffekte.push(&karte.dauereffekt);
		}
		
		Some(())
	}
	
	/// Prüft ob die Anlegekarte an das gegebene Ziel ausgespielt werden kann.
	fn anlegekarte_prüfen(
		&self,
		karte: &'static Karte,
		hauptpflanze_idx: usize,
		pos: Kartenposition,
	) -> bool
	{
		
		let hauptpflanze = &self.nächster_spielerstand().wald[hauptpflanze_idx];
		let seite = hauptpflanze.seite(pos);
		
		// Normalerweise ist nur eine Karte pro Seite erlaubt, es gibt allerdings
		// einpaar Ausnahmen.
		
		if seite.is_empty() { return true; }
		
		if karte.typen.contains(&Typsymbol::Schmetterling) && 
			hauptpflanze.iter_anlegekarten().any(|k| k.bezeichnung == "Brennessel")
		{
			return true;
		}
		
		match karte.bezeichnung {
			"Feldhase" => seite.iter()
				.all(|&k| k.bezeichnung == "Feldhase"),
			"Erdkröte" => seite.len() == 1 &&
				seite[0].bezeichnung == "Erdkröte",
			_ => false
		}
	}
	
	/// Der Spieler *darf* einen Effekt ausführen wenn er möchte.
	///
	/// Gibt zurück ob der Effekt vom Spieler benutzt wurde. Dies kann je nach
	/// Effekt selbst dann negativ sein wenn `Spieler::effekt_ausüben` `true`
	/// zurück gibt. ACHTUNG: dies ist nur für `Effekt::EinsKostenlosAblegen`
	/// implementiert, da diese Unterscheidung nur von Sträucher-Boni
	/// (welche nur diesen Effekt haben) gebraucht wird. TODO: entweder die API so
	/// spezialisieren das nicht die Erwartung entstehen kann, dass dies für
	/// andere Effekte funktioniert oder dies für alle Effekt implementieren.
	#[must_use]
	fn effekt_ausüben<S: Spieler>(
		&mut self,
		spieler: &mut S,
		zugstatus: &mut Zugstatus,
		effekt: &'static Effekt,
		bedingungskontext: Bedingungskontext,
	) -> OptionSpielende<EffektGenutzt> {
		if matches!(effekt, Effekt::Keiner) { return Some(false); }
		if !spieler.effekt_ausüben(self, effekt) { return Some(false); }
		
		let mut effekt_genutzt = false;
		match *effekt {
			Effekt::Keiner => unreachable!(),
			Effekt::Extrazug => zugstatus.extrazug = true,
			// REGELFRAGE: Die Anleitung besagt „Ziehe so viele Karten vom Stapel wie angegeben.“
			//             Diese implementierung ermöglicht es nicht eine geringere
			//             Anzahl Karten zu ziehen außer das Handkartenlimit wird
			//             erreicht.
			Effekt::KartenZiehen(n) => {
				self.karten_ziehen_n(n)?;
			},
			Effekt::KartenZiehenLichtung(mut n) => {
				while self.nächster_spielerstand().handkarten.len() < 10 && n > 0 {
					if let Some(idx) = spieler.aus_lichtung_ziehen(self) {
						let karte = self.lichtung.swap_remove(idx);
						self.nächster_spielerstand_mut().handkarten.push(karte).map_err(|_| ()).unwrap();
					} else {
						break;
					}
					n -= 1;
				}
			},
			Effekt::KartenZiehenAnzTyp(typ) => {
				let anz = self.nächster_spielerstand().iter_wald().filter(|k| k.typen.contains(&typ)).count();
				self.karten_ziehen_n(anz.try_into().unwrap())?;
			},
			Effekt::KartenZiehenAnzName(name) => {
				let anz = self.nächster_spielerstand().iter_wald().filter(|k| k.bezeichnung == name).count();
				self.karten_ziehen_n(anz.try_into().unwrap())?;
			},
			Effekt::BeliebigKartenKostenpflichtigSpielen => {
				while let Some(ausspielen_info) = spieler.bonus_ausspielen_kostenpflichtig(self) {
					self.karte_bezahlen_und_ausspielen(spieler, zugstatus, ausspielen_info)?;
				}
			},
			Effekt::EinsKostenlosAblegen(typsymbol) => {
				effekt_genutzt = self.eins_kostenlos_ablegen_typ(
					spieler, zugstatus, typsymbol)?;
			},
			Effekt::EinsKostenlosAblegen2x(typ1, typ2) => {
				self.eins_kostenlos_ablegen_typ(spieler, zugstatus, typ1);
				self.eins_kostenlos_ablegen_typ(spieler, zugstatus, typ2);
			},
			Effekt::EinsKostenlosAblegenName(bezeichnung) => {
				if let Some(ausspielen_info) = spieler
					.bonus_kostenlos_ablegen_name(self, bezeichnung)
				{
				self.eins_kostenlos_ablegen(
					spieler, zugstatus, ausspielen_info,
					|k| k.bezeichnung == bezeichnung)?;
				}
			},
			Effekt::BeliebigKostenlosAblegen(typsymbol) => {
				while self.eins_kostenlos_ablegen_typ(spieler, zugstatus, typsymbol)? {}
			},
			Effekt::BeliebigSprösslingeAusspielen => {
				while let Some(idx) = 
					spieler.setzling_ausspielen(self)
				{
					self.setzling_auspielen(idx)?;
				}
			},
			Effekt::HandkartenHöle => {
				let ablegen_bools = spieler.handkarten_höhle(self);
				let spielerstand = self.nächster_spielerstand_mut();
				let anz_abgelegte_karten = ablegen_bools.iter().filter(|&&b| b).count() as u32;
				spielerstand.höhle += anz_abgelegte_karten;
				spielerstand.handkarten = mit_bitmap_filtern(spielerstand.handkarten.iter().copied(), ablegen_bools);
				self.karten_ziehen_n(anz_abgelegte_karten)?;
			},
			Effekt::LichtungHöhle(anz_karten) => {
				let in_höhle_bools = spieler.lichtung_höhle(self, anz_karten);
				
				assert!(in_höhle_bools.iter().filter(|&&b| b).count() <= anz_karten as usize);
				self.lichtung = mit_bitmap_filtern(self.lichtung.iter().copied(), in_höhle_bools);
				let spielerstand = self.nächster_spielerstand_mut();
				spielerstand.höhle += anz_karten;
			},
			Effekt::LichtungHöleAlle => {
				let len_lichtung = self.lichtung.len() as u32;
				let spielerstand = self.nächster_spielerstand_mut();
				spielerstand.höhle += len_lichtung;
				self.lichtung.clear();
			},
			Effekt::LichtungHöhleTypen(typen) => {
				let len_voher = self.lichtung.len() as u32;
				for typ in typen {
					self.lichtung.retain(|&mut k| !k.hat_typsymbol(typ))
				}
				let len_nachher = self.lichtung.len() as u32;
				let spielerstand = self.nächster_spielerstand_mut();
				spielerstand.höhle += len_voher - len_nachher;
			},
			Effekt::LichtungHandTyp(typ) => {
				while self.nächster_spielerstand_mut().handkarten.len() < 10 &&
					self.lichtung.iter().find(|k| k.hat_typsymbol(&typ)).is_some()
				{
					let index = spieler.lichtung_hand_typ(self, typ);
					let karte = self.lichtung.swap_remove(index);
					assert!(karte.hat_typsymbol(&typ));
					self.nächster_spielerstand_mut().handkarten.push(karte).map_err(|_| ()).unwrap()
				}
			},
			Effekt::LichtungLeeren => {
				self.lichtung.clear();
			},
			Effekt::KarteZiehenPlusExtrazug => {
				zugstatus.extrazug = true;
				self.karten_ziehen_n(1)?;
			},
			Effekt::Bedingung(effekt, bedingung) => {
				if self.bedingung_prüfen(&bedingung, bedingungskontext) {
					self.effekt_ausüben(spieler, zugstatus, effekt, bedingungskontext)?;
				}
			},
		}
		
		Some(effekt_genutzt)
	}
	
	// gibt zurück ob der Spieler eine Karte ausgespielt hat.
	fn eins_kostenlos_ablegen_typ<S: Spieler>(
		&mut self,
		spieler: &mut S,
		zugstatus: &mut Zugstatus,
		typsymbol: Typsymbol,
	) -> OptionSpielende<bool> {
		if let Some(ausspielen_info) = spieler
			.bonus_kostenlos_ablegen_typ(self, typsymbol)
		{
			self.eins_kostenlos_ablegen(
				spieler, zugstatus, ausspielen_info,
				|k| k.typen.contains(&typsymbol))?;
			Some(true)
		} else {
			Some(false)
		}
	}
	
	#[must_use]
	fn eins_kostenlos_ablegen<S: Spieler, B: FnOnce(&'static Karte) -> bool>(
		&mut self,
		spieler: &mut S,
		zugstatus: &mut Zugstatus,
		ausspielen_info: Ausspielen,
		bedingung: B,
	) -> OptionSpielende<()> {
		let karte = self.nächster_spielerstand_mut()
			.handkarten
			.remove(ausspielen_info.karten_idx)
			.unwrap();
		
		
		match (karte,ausspielen_info.ausspielziel) {
			(GanzeKarte::Hauptpflanze(hp), Ausspielziel::NeueHauptpflanze) => {
				assert!(bedingung(hp));
				self.hauptpflanze_ausspielen(spieler, zugstatus, hp, false, true)?;
			},
			(_, Ausspielziel::AnHauptpflanzeAnlegen {pos, .. }) => {
				// panic wenn die Position nicht zur Karte passt.
				let halbkarte = karte.halbkarte(pos);
				assert!(bedingung(halbkarte));
				self.anlegekarte_ausspielen(
					spieler,
					zugstatus,
					halbkarte,
					ausspielen_info.ausspielziel,
					false, // passend
					true // boni_und_soforteffekte_deaktiviert
				)?;
			},
			(_, Ausspielziel::NeueHauptpflanze) => panic!("Ausspielziel ist `NeueHauptfplanze` aber die Karte ist nicht `GanzeKarte::Hauptpflanze(_)`"),
		}
		
		Some(())
	}
	
	#[must_use]
	fn karten_ziehen_n(
		&mut self,
		mut anz_karten: u32
	) -> OptionSpielende<()> {
		while self.nächster_spielerstand().handkarten.len() < 10 && anz_karten > 0 {
			let karte = self.karte_ziehen()?;
			self.nächster_spielerstand_mut().handkarten.push(karte).map_err(|_| ()).unwrap();
			anz_karten -= 1;
		}
		Some(())
	}
	
	#[must_use]
	fn karte_ziehen_auswahl<S: Spieler>(
		&mut self,
		spieler: &mut S,
		karte_von: ZiehenVon,
	) -> OptionSpielende<()> {
		assert!(self.nächster_spielerstand().handkarten.len() < 10);
		let karte = match karte_von {
			ZiehenVon::Ziehstapel => self.karte_ziehen()?,
			ZiehenVon::Lichtung => if let Some(idx) = spieler.aus_lichtung_ziehen(self) {
				self.lichtung.swap_remove(idx)
			} else {
				return Some(());
			},
		};
		self.nächster_spielerstand_mut().handkarten.push(karte).map_err(|_| ()).unwrap();
		
		Some(())
	}
	
	/// Karte mit Preis `anzahl` muss bezahlt werden. Gibt zurück ob der Spieler
	/// passend zahlt.
	fn bezahlen<S: Spieler>(
		&mut self,
		spieler: &mut S,
		mut anzahl: u32,
		baumsymbol: Baumsymbol,
	) -> bool {
		let mut passend = true;
		while anzahl != 0 {
			let karte_idx = spieler.bezahlen(self);
			let karte = self.nächster_spielerstand_mut().handkarten.remove(karte_idx).unwrap();
			passend = passend && match karte {
				GanzeKarte::Hauptpflanze(k) => k.baumsymbol == baumsymbol,
				GanzeKarte::ZweigeteiltH { links: a, rechts: b } |
					GanzeKarte::ZweigeteiltV { oben: a, unten: b } => a.baumsymbol == baumsymbol || b.baumsymbol == baumsymbol,
				GanzeKarte::Winter => panic!(),
			};
			anzahl -= 1;
		}
		passend
	}
	
	fn erste_runde_logik<S: Spieler>(&mut self, spieler: &mut S) {
		// REGELFRAGE: Die Anleitung spezifiziert das nachziehen nur erlaubt ist
		//             wenn der Spieler keine *Bäume* auf der Hand hat.
		//             Sollten vlt auch Sträucher berücksichtigt werden?
		if self.nächster_spielerstand().handkarten.iter().find(|&&k| {
			match k {
				GanzeKarte::Hauptpflanze(k) if k.typen.contains(&Typsymbol::Baum) => true,
				_ => false,
			}
		}).is_some() {
			if spieler.handwechsel_keine_bäume(self) {
				self.nächster_spielerstand_mut().handkarten.clear();
				
				for _ in 0..6 {
					let karte = self.karte_ziehen().expect("Der Winter solle niemals in der ersten Runde kommen");
					self.nächster_spielerstand_mut().handkarten.push(karte).map_err(|_| ()).unwrap();
				}
			}
		}
	}
	
	#[must_use]
	fn karte_ziehen(&mut self) -> OptionSpielende<&'static GanzeKarte> {
		if self.ist_spiel_beendet() { return None; }
		match self.ziehstapel.pop() {
			Some(GanzeKarte::Winter) => {
				self.anz_winterkarten_gezogen += 1;
				self.karte_ziehen()
			},
			Some(karte) => Some(karte),
			None => panic!("Ziehstapel Leer"),
		}
	}
	
	fn zug_beendet_nächster_spieler(&mut self) {
		self.nächster_spieler += 1;
		if self.nächster_spieler == self.spieler.len() {
			self.nächster_spieler = 0;
			self.erste_runde = false;
		}
	}
	
	fn bedingung_prüfen(&self, bedingung: &Bedingung, kontext: Bedingungskontext) -> bool {
		match bedingung {
			Bedingung::MeisteNamen(name) =>
				self.bedinung_meiste(|&k| &k.bezeichnung == name),
			Bedingung::MeisteAnzTyp(typ) =>
				self.bedinung_meiste(|&k| k.typen.contains(typ)),
			Bedingung::MinAnzNamen(min, name) =>
				self.bedingung_min_anz(*min, |&k| &k.bezeichnung == name),
			Bedingung::MinAnzTypVerschieden(min, typ) => {
				let mut gesehen : SmallVec<[&'static str;5]> = SmallVec::new();
				let f = |&k: &&Karte| {
					let neu = k.typen.contains(typ) && !gesehen.contains(&k.bezeichnung);
					if neu {
						gesehen.push(k.bezeichnung);
					}
					neu
				};
				self.bedingung_min_anz(*min, f)
			},
			Bedingung::MinAnzTyp(min, typ) =>
				self.bedingung_min_anz(*min, |&k| k.typen.contains(typ)),
			Bedingung::TypGegenüber(typsymbol) => self.nächster_spielerstand()
				.wald[kontext.hauptpflanze_idx]
				.seite(kontext.kartenpos.expect(
					"Kartenposition muss für `Bedingung::TypGegenüber` gegeben sein"
				))
				.iter()
				.any(|&k| k.typen.contains(typsymbol)),
			Bedingung::MinAnzNamenAnPlatz(anz, name) => self.nächster_spielerstand()
				.wald[kontext.hauptpflanze_idx]
				.seite(kontext.kartenpos.expect(
					"Kartenposition muss für `Bedingung::MinAnzNamenAnPlatz` gegeben sein"
				))
				.iter()
				.filter(|k| &k.bezeichnung == name)
				.count() >= *anz as usize,
			Bedingung::Hauptpflanzenname(name) => &self.nächster_spielerstand()
				.wald[kontext.hauptpflanze_idx]
				.hauptpflanze
				.bezeichnung == name,
			Bedingung::Hauptpflanzentyp(typen) => self.nächster_spielerstand()
				.wald[kontext.hauptpflanze_idx]
				.hauptpflanze
				.typen
				.iter()
				.any(|&t| typen.contains(&t)),
		}
	}
	
	fn bedinung_meiste<B>(&self, b: B) -> bool
	where
		B: for <'a> Fn(&'a &'static Karte) -> bool
	{
		// unwrap: es muss mindestens einen Spieler geben(eig. sogar zwei),
		//         deswegen kann `.max()` nie `None` sein.
		let max = self.spieler
			.iter()
			.map(|s| s.iter_wald().filter(&b).count())
			.max()
			.unwrap();
		let anz = self.nächster_spielerstand()
			.iter_wald()
			.filter(&b).count();
		
		anz == max
	}
	
	fn bedingung_min_anz<B>(&self, min: u32, b: B) -> bool
	where
		B: for <'a> FnMut(&'a &'static Karte) -> bool
	{
		self.nächster_spielerstand().iter_wald().filter(b).count() as u32 >= min
	}
}



// TODO generische Datentypen zu komplex; umschreiben.
/// Filtert `v` sodass nur Elemente behalten werden bei denen der `bool` Wert
/// in `bitmap` am entspechenden Index `true` ist.
///
/// pannict wenn `v` und `bitmap` nicht gleich lang sind.
fn mit_bitmap_filtern<Item, Iter2, V, B, R>(v: V, bitmap: B) -> R
where
	V: Iterator<Item=Item> + ExactSizeIterator,
	B: IntoIterator<Item=bool, IntoIter=Iter2>,
	Iter2: Iterator<Item=bool> + ExactSizeIterator,
	R: FromIterator<Item>,
{
	let b_iter = bitmap.into_iter();
	assert!(v.len() == b_iter.len());
	v.zip(b_iter).filter_map(|(i, behalten)| {
		if behalten {
			Some(i)
		} else {
			None
		}
	}).collect()
}
