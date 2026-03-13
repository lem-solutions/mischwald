use crate::{karten, prelude::*};
use rand::seq::SliceRandom;
use std::collections::{HashMap, HashSet, hash_map::Entry};

// +3 für die Winterkarten nicht nötig, da das Maximum für das Spiel nie ausgereizt wird.
const MAX_ZIEHSTAPEL: usize = karten::hauptspiel::KARTEN.len()
	+ karten::alpin::KARTEN.len()
	+ karten::waldrand::KARTEN.len();
const MAX_SPIELER: usize = 5;

#[allow(clippy::identity_op)]
fn anz_karten_weglegen(anz_spieler: usize, anz_erweiterungen: usize) -> usize {
	match (anz_erweiterungen, anz_spieler) {
		(0, 2) => 30,
		(0, 3) => 20,
		(0, 4) => 10,
		(0, 5) => 0,

		(1, 2) => 10 + 45,
		(1, 3) => 10 + 30,
		(1, 4) => 10 + 15,
		(1, 5) => 10 + 0,

		(2, 2) => 10 + 80,
		(2, 3) => 10 + 50,
		(2, 4) => 10 + 35,
		(2, 5) => 10 + 20,

		_ => panic!(
			"ungültige Anzahl Spieler oder Erweiterungen: Spieler:{}, Erweiterungen:{}",
			anz_spieler, anz_erweiterungen
		),
	}
}

/// Hilfstyp für die Bewertung von Sammlungen auf Typ-Basis.
struct Typsammlung {
	bewertungsschlüssel: &'static [u32],
	sammlungen: Vec<HashSet<&'static str>>,
}
impl Typsammlung {
	fn neu(bewertungsschlüssel: &'static [u32]) -> Self {
		Typsammlung {
			bewertungsschlüssel,
			sammlungen: Vec::new(),
		}
	}

	fn einfügen(&mut self, karte: &'static Karte) {
		for sammlung in self.sammlungen.iter_mut() {
			if sammlung.insert(karte.bezeichnung) {
				return;
			}
		}
		let mut set = HashSet::new();
		set.insert(karte.bezeichnung);
		self.sammlungen.push(set);
	}

	fn bewerten(&self) -> u32 {
		let mut pkt = 0;
		for sammlung in self.sammlungen.iter() {
			if sammlung.is_empty() {
				continue;
			}
			let idx = sammlung.len() - 1;
			let idx_gedeckelt = idx.min(self.bewertungsschlüssel.len() - 1);
			pkt += self.bewertungsschlüssel[idx_gedeckelt];
		}
		pkt
	}
}

pub struct Hauptpflanze {
	pub(crate) hauptpflanze: &'static Karte,
	pub(crate) oben: SmallVec<[&'static Karte; 1]>,
	pub(crate) unten: SmallVec<[&'static Karte; 1]>,
	pub(crate) links: SmallVec<[&'static Karte; 1]>,
	pub(crate) rechts: SmallVec<[&'static Karte; 1]>,
}
impl Hauptpflanze {
	pub(crate) fn neu(karte: &'static Karte) -> Self {
		Hauptpflanze {
			hauptpflanze: karte,
			oben: SmallVec::new(),
			unten: SmallVec::new(),
			links: SmallVec::new(),
			rechts: SmallVec::new(),
		}
	}

	pub(crate) fn seite(
		&self,
		pos: Kartenposition,
	) -> &SmallVec<[&'static Karte; 1]> {
		match pos {
			Kartenposition::Oben => &self.oben,
			Kartenposition::Unten => &self.unten,
			Kartenposition::Links => &self.links,
			Kartenposition::Rechts => &self.rechts,
		}
	}

	pub(crate) fn seite_mut(
		&mut self,
		pos: Kartenposition,
	) -> &mut SmallVec<[&'static Karte; 1]> {
		match pos {
			Kartenposition::Oben => &mut self.oben,
			Kartenposition::Unten => &mut self.unten,
			Kartenposition::Links => &mut self.links,
			Kartenposition::Rechts => &mut self.rechts,
		}
	}

	pub(crate) fn iter_anlegekarten(
		&self,
	) -> impl Iterator<Item = &'static Karte> {
		self
			.oben
			.iter()
			.chain(self.unten.iter())
			.chain(self.links.iter())
			.chain(self.rechts.iter())
			.copied()
	}

	pub(crate) fn iter_anlegekarten_mit_kontext(
		&self,
		hp_idx: usize,
	) -> impl Iterator<Item = (&'static Karte, Bedingungskontext)> {
		self
			.iter_seite_kon(Kartenposition::Oben, hp_idx)
			.chain(self.iter_seite_kon(Kartenposition::Unten, hp_idx))
			.chain(self.iter_seite_kon(Kartenposition::Links, hp_idx))
			.chain(self.iter_seite_kon(Kartenposition::Rechts, hp_idx))
	}

	fn iter_seite_kon(
		&self,
		seite: Kartenposition,
		hp_idx: usize,
	) -> impl Iterator<Item = (&'static Karte, Bedingungskontext)> {
		self.seite(seite).iter().map(move |k| {
			(
				*k,
				Bedingungskontext {
					hauptpflanze_idx: hp_idx,
					kartenpos: Some(seite),
				},
			)
		})
	}
}

pub struct Spielerstand {
	pub(crate) handkarten: MiniVec<&'static GanzeKarte, 10>,
	pub(crate) höhle: u32,
	pub(crate) wald: SmallVec<[Hauptpflanze; 20]>,
	pub(crate) dauereffekte: SmallVec<[&'static Dauereffekt; 10]>,
}
impl Spielerstand {
	fn neu(handkarten: &[&'static GanzeKarte]) -> Self {
		assert_eq!(handkarten.len(), 6);
		let mut handkarten_v = MiniVec::new();
		handkarten_v.extend(handkarten);
		Spielerstand {
			handkarten: handkarten_v,
			höhle: 0,
			wald: SmallVec::new(),
			dauereffekte: SmallVec::new(),
		}
	}

	pub fn iter_wald(&self) -> impl Iterator<Item = &'static Karte> {
		self.wald.iter().flat_map(|hauptpflanze| {
			[hauptpflanze.hauptpflanze]
				.into_iter()
				.chain(hauptpflanze.iter_anlegekarten())
		})
	}

	fn iter_wald_mit_kontext(
		&self,
	) -> impl Iterator<Item = (&'static Karte, Bedingungskontext)> {
		self
			.wald
			.iter()
			.enumerate()
			.flat_map(|(idx, hauptpflanze)| {
				let k = Bedingungskontext {
					hauptpflanze_idx: idx,
					kartenpos: None,
				};
				[(hauptpflanze.hauptpflanze, k)]
					.into_iter()
					.chain(hauptpflanze.iter_anlegekarten_mit_kontext(idx))
			})
	}

	fn sammlungen_bewerten(&self) -> u32 {
		// REGELFRAGE: Wenn der Spieler z. B. folgende Schmetterlinge hat: A A B B C
		//             bekommt er nur Punkte für eine Sammlung mit Größe 3 oder
		//             bekommt er auch Punkte für eine Sammlung mit Größe 2 dazu?
		let mut sammlungen_name: HashMap<&'static str, u32> = HashMap::new();
		let mut sammlung_name_schlüssel: HashMap<&'static str, &'static [u32]> =
			HashMap::new();
		let mut sammlungen_typ: HashMap<Typsymbol, Typsammlung> = HashMap::new();

		// Typensammlungen in zwei Schritten da es Typsammlungen gibt bei
		// denen nicht jede Karte mit dem Typ selbst die Sammlung als
		// Punkteffekt hat.
		for karte in self.iter_wald() {
			match karte.punkte {
				Punkteffekt::SammlungName(schlüssel) => {
					*sammlungen_name.entry(karte.bezeichnung).or_insert(0) += 1;
					match sammlung_name_schlüssel.entry(karte.bezeichnung) {
						// Verschieden bewertete Sammelungen
						Entry::Occupied(e) => assert_eq!(*e.get(), schlüssel),
						Entry::Vacant(e) => {
							e.insert(schlüssel);
						}
					}
				}
				Punkteffekt::SammlungTypVersch(typ, schlüssel) => {
					match sammlungen_typ.entry(typ) {
						// Verschieden bewertete Sammelungen
						Entry::Occupied(e) => {
							assert_eq!(e.get().bewertungsschlüssel, schlüssel)
						}
						Entry::Vacant(e) => {
							e.insert(Typsammlung::neu(schlüssel));
						}
					}
				}
				_ => continue,
			}
		}

		for karte in self.iter_wald() {
			for typ in karte.typen {
				if let Some(s) = sammlungen_typ.get_mut(typ) {
					s.einfügen(karte);
				}
			}
		}

		let mut pkt = 0;
		for (name, anz_u32) in sammlungen_name {
			let anz: usize = anz_u32.try_into().unwrap();
			assert!(anz > 0);
			let schlüssel = sammlung_name_schlüssel[name];
			let idx = anz - 1;
			let idx_gedeckelt = idx.min(schlüssel.len() - 1);
			pkt += schlüssel[idx_gedeckelt];
		}
		pkt += sammlungen_typ.values().map(|s| s.bewerten()).sum::<u32>();

		pkt
	}
}
impl Default for Spielerstand {
	fn default() -> Self {
		Spielerstand {
			handkarten: MiniVec::new(),
			höhle: 0,
			wald: SmallVec::new(),
			dauereffekte: SmallVec::new(),
		}
	}
}

pub struct Spielstand {
	pub(crate) ziehstapel: MiniVec<&'static GanzeKarte, MAX_ZIEHSTAPEL>,
	pub(crate) lichtung: SmallVec<[&'static GanzeKarte; 15]>,
	pub(crate) spieler: MiniVec<Spielerstand, MAX_SPIELER>,
	pub(crate) anz_winterkarten_gezogen: u32,
	pub(crate) nächster_spieler: usize,
	pub(crate) erste_runde: bool,
}
impl Spielstand {
	pub fn neu<R: rand::Rng>(
		anz_spieler: usize,
		erweiterungen: &[&[&'static GanzeKarte]],
		rng: &mut R,
	) -> Self {
		assert!((2..=5).contains(&anz_spieler));

		let mut ziehstapel: MiniVec<&'static GanzeKarte, MAX_ZIEHSTAPEL> =
			MiniVec::new();
		ziehstapel.extend(erweiterungen.iter().flat_map(|s| s.iter()));
		ziehstapel.shuffle(rng);
		ziehstapel.truncate(
			ziehstapel.len() - anz_karten_weglegen(anz_spieler, erweiterungen.len()),
		);

		let wintergrenze = ziehstapel.len() / 3;

		let mut spieler = MiniVec::new();
		for _ in 0..anz_spieler {
			let handkarten = &ziehstapel[ziehstapel.len() - 6..];
			spieler
				.push(Spielerstand::neu(handkarten))
				.map_err(|_| ())
				.unwrap();
			ziehstapel.truncate(ziehstapel.len() - 6);
		}

		// unwrap: Wenn das Maximum ausgereizt war, wurden bereits genug Karten weggelegt
		ziehstapel
			.insert(wintergrenze, &GanzeKarte::Winter)
			.map_err(|_| ())
			.unwrap();
		ziehstapel
			.insert(rng.random_range(..wintergrenze), &GanzeKarte::Winter)
			.map_err(|_| ())
			.unwrap();
		ziehstapel
			.insert(rng.random_range(..wintergrenze + 1), &GanzeKarte::Winter)
			.map_err(|_| ())
			.unwrap();

		Spielstand {
			ziehstapel,
			lichtung: SmallVec::new(),
			spieler,
			anz_winterkarten_gezogen: 0,
			nächster_spieler: 0,
			erste_runde: false,
		}
	}

	pub fn nächster_spieler(&self) -> usize {
		self.nächster_spieler
	}

	pub fn ist_spiel_beendet(&self) -> bool {
		self.anz_winterkarten_gezogen == 3
	}

	pub(crate) fn nächster_spielerstand(&self) -> &Spielerstand {
		&self.spieler[self.nächster_spieler]
	}

	pub(crate) fn nächster_spielerstand_mut(&mut self) -> &mut Spielerstand {
		&mut self.spieler[self.nächster_spieler]
	}

	pub fn bewerten(&self, spieler_idx: usize) -> u32 {
		let spielerstand = &self.spieler[spieler_idx];
		spielerstand
			.iter_wald_mit_kontext()
			.map(|(karte, kontext)| {
				karte.punkte.einzelbewertung(self, spieler_idx, &kontext)
			})
			.sum::<u32>()
			+ spielerstand.höhle
			+ spielerstand.sammlungen_bewerten()
	}
}
