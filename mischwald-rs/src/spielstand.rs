use noalloc_vec_rs::vec::Vec as MiniVec;
use rand::seq::SliceRandom;
use smallvec::SmallVec;

use crate::{datentypen::*, karten};

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
}
