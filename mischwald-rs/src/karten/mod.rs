pub mod alpin;
pub mod hauptspiel;
pub mod waldrand;

use super::datentypen::*;

struct Kartenvorlage {
	pub kosten: u32,
	pub typen: Typsymbole,
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
