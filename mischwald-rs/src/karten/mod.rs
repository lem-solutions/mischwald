pub mod alpin;
pub mod hauptspiel;
pub mod waldrand;

use crate::prelude::*;

struct Kartenvorlage {
	pub kosten: u32,
	pub typen: &'static [Typsymbol],
	pub soforteffekt: Effekt,
	pub dauereffekt: Dauereffekt,
	pub bonus: Effekt,
	pub punkte: Punkteffekt,
	pub bezeichnung: &'static str, // wird für manche Effekte gebraucht
}

const fn kartenvorlage_realisieren(
	vorlage: &Kartenvorlage,
	baumsymbol: Baumsymbol,
) -> Karte {
	Karte {
		kosten: vorlage.kosten,
		baumsymbol,
		typen: vorlage.typen,
		soforteffekt: vorlage.soforteffekt,
		dauereffekt: vorlage.dauereffekt,
		bonus: vorlage.bonus,
		punkte: vorlage.punkte,
		bezeichnung: vorlage.bezeichnung,
	}
}

const SAMMLUNG_SCHMETTERLINGE: &[u32] = &[0, 3, 6, 12, 20, 35, 55];

pub const SETZLING: Karte = Karte {
	kosten: 0,
	baumsymbol: Baumsymbol::Keins,
	typen: &[],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Setzling",
};

pub enum GanzeKarte {
	Hauptpflanze(&'static Karte),
	ZweigeteiltH {
		links: &'static Karte,
		rechts: &'static Karte,
	},
	ZweigeteiltV {
		oben: &'static Karte,
		unten: &'static Karte,
	},
	Winter,
}
impl GanzeKarte {
	pub fn halbkarte(&self, pos: Kartenposition) -> &'static Karte {
		match (self, pos) {
			(GanzeKarte::ZweigeteiltH { links: k, .. }, Kartenposition::Links)
			| (GanzeKarte::ZweigeteiltH { rechts: k, .. }, Kartenposition::Rechts)
			| (GanzeKarte::ZweigeteiltV { oben: k, .. }, Kartenposition::Oben)
			| (GanzeKarte::ZweigeteiltV { unten: k, .. }, Kartenposition::Unten) => k,
			_ => panic!("Ungültike Kartenposition für Ganze Karte"),
		}
	}

	pub fn hat_typsymbol(&self, symbol: &Typsymbol) -> bool {
		match self {
			Self::Hauptpflanze(a) => a.typen.contains(symbol),
			Self::ZweigeteiltH {
				links: a,
				rechts: b,
			}
			| Self::ZweigeteiltV { oben: a, unten: b } => {
				a.typen.contains(symbol) || b.typen.contains(symbol)
			}
			Self::Winter => false,
		}
	}
}

#[derive(Clone, Copy, PartialEq)]
pub enum Baumsymbol {
	// Für Setzlinge
	Keins,

	Ahorn,
	Birke,
	Buche,
	Douglasie,
	Eiche,
	Kastanie,
	Linde,
	Tanne,

	// Alpinerweiterung
	Lärche,
	Zirbelkiefer,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Typsymbol {
	Amphibie,
	Baum,
	Fledermaus,
	Hirsch,
	Insekt,
	Paarhufer,
	Pflanze,
	Pfotentier,
	Pilz,
	Schmetterling,
	Vogel,

	// Alpinerweiterung
	Alpen,

	// Waldranderweiterung
	Waldrand,
	Strauch, // Wird in der Anleitung „Sträucher“ genannt.
}

pub struct Karte {
	pub kosten: u32,
	pub baumsymbol: Baumsymbol,
	pub typen: &'static [Typsymbol],
	pub soforteffekt: Effekt,
	pub dauereffekt: Dauereffekt,
	pub bonus: Effekt,
	pub punkte: Punkteffekt,
	pub bezeichnung: &'static str, // wird für manche Effekte gebraucht
}

#[derive(Clone, Copy, PartialEq)]
pub enum Kartenposition {
	Oben,
	Unten,
	Links,
	Rechts,
}
impl Kartenposition {
	pub fn gegenüber(&self) -> Self {
		match self {
			Kartenposition::Oben => Kartenposition::Unten,
			Kartenposition::Unten => Kartenposition::Oben,
			Kartenposition::Links => Kartenposition::Rechts,
			Kartenposition::Rechts => Kartenposition::Links,
		}
	}
}
