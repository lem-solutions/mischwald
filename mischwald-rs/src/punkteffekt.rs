use crate::bedingung::*;
use crate::karten::{Baumsymbol, Kartenposition, Typsymbol};

#[derive(Clone, Copy)]
pub enum Punkteffekt {
	Keiner,
	Konstant(u32),
	Bedingung(u32, Bedingung),
	ProTyp(u32, Typsymbol),
	ProTypUnterschiedlich(u32, Typsymbol),
	ProTypen(u32, &'static [Typsymbol]),
	ProBaumsymbole(u32, &'static [Baumsymbol]),
	ProName(u32, &'static str),
	ProKarteAnHauptpflanze(u32),
	ProVollbelegtemBaum(u32),
	// z. B. Waldameise: 2 Punkte je Karte unten an einem Baum
	ProKartenAnTypPosition(u32, Typsymbol, Kartenposition),
	ProKarteInHöhle(u32),
	BedingungOder {
		bedingung: Bedingung,
		wenn: u32,
		sonst: u32,
	},
	// Bewertung nach Anzahl der Karten(nach Namen)
	// Wenn die Anzahl mehr als das Maximum der Bewertungsliste ist wird das höchste Element gewählt
	SammlungName(&'static [u32]),
	// Bewertung nach Anzahl der *verschiedenen* Karten mit dem gegebenen Typ.
	SammlungTypVersch(Typsymbol, &'static [u32]),
}
