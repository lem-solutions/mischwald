use std::collections::HashSet;

use crate::Spielstand;
use crate::bedingung::*;
use crate::karten::{Baumsymbol, Karte, Kartenposition, Typsymbol};

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
	// Wenn die Anzahl mehr als das Maximum der Bewertungsliste ist wird das
	// höchste Element gewählt
	SammlungName(&'static [u32]),
	// Bewertung nach Anzahl der *verschiedenen* Karten mit dem gegebenen Typ.
	SammlungTypVersch(Typsymbol, &'static [u32]),
}
impl Punkteffekt {
	/// Gibt, falls der Punkteffekt *KEINE* Sammlung ist, den Punktewert der Karte
	/// zurück. Falls es eine Sammlung ist, ist der Rückgabewert 0.
	pub fn einzelbewertung(
		&self,
		spielstand: &Spielstand,
		spieler_idx: usize,
		kontext: &Bedingungskontext,
	) -> u32 {
		let wald = spielstand.spieler[spieler_idx].iter_wald();
		match *self {
			Punkteffekt::Keiner
			| Punkteffekt::SammlungName(_)
			| Punkteffekt::SammlungTypVersch(_, _) => 0,
			Punkteffekt::Konstant(p) => p,
			Punkteffekt::ProKarteInHöhle(p) => {
				spielstand.spieler[spieler_idx].höhle * p
			}
			Punkteffekt::Bedingung(p, b) => {
				if b.prüfen(spielstand, kontext) {
					p
				} else {
					0
				}
			}
			Punkteffekt::BedingungOder {
				bedingung,
				wenn,
				sonst,
			} => {
				if bedingung.prüfen(spielstand, kontext) {
					wenn
				} else {
					sonst
				}
			}
			Punkteffekt::ProTypUnterschiedlich(p, t) => {
				let anz: u32 = spielstand.spieler[spieler_idx]
					.iter_wald()
					.filter_map(|k| {
						if k.typen.contains(&t) {
							Some(k.bezeichnung)
						} else {
							None
						}
					})
					.fold(HashSet::new(), |mut set, n| {
						set.insert(n);
						set
					})
					.len()
					.try_into()
					.unwrap();
				anz * p
			}
			Punkteffekt::ProTyp(p, t) => {
				anz_im_wald(wald, |k| k.typen.contains(&t)) * p
			}
			Punkteffekt::ProTypen(p, t) => {
				anz_im_wald(wald, |k| k.typen.iter().any(|kt| t.contains(kt))) * p
			}
			Punkteffekt::ProBaumsymbole(p, sym) => {
				anz_im_wald(wald, |k| sym.contains(&k.baumsymbol)) * p
			}
			Punkteffekt::ProName(p, n) => {
				anz_im_wald(wald, |k| k.bezeichnung == n) * p
			}
			Punkteffekt::ProKarteAnHauptpflanze(p) => {
				// TODO War es pro Anlegekarte oder pro *weiterer* Anlegekarte?
				//      `Punkteffekt::*` kommentieren!
				let anz: u32 = spielstand.spieler[spieler_idx].wald
					[kontext.hauptpflanze_idx]
					.iter_anlegekarten()
					.count()
					.try_into()
					.unwrap();
				anz * p
			}
			Punkteffekt::ProVollbelegtemBaum(p) => {
				let anz: u32 = spielstand.spieler[spieler_idx]
					.wald
					.iter()
					.filter(|hp| {
						!hp.oben.is_empty()
							&& !hp.unten.is_empty()
							&& !hp.links.is_empty()
							&& !hp.rechts.is_empty()
					})
					.count()
					.try_into()
					.unwrap();
				anz * p
			}
			Punkteffekt::ProKartenAnTypPosition(p, t, pos) => {
				let anz: u32 = spielstand.spieler[spieler_idx]
					.wald
					.iter()
					.filter(|hp| hp.hauptpflanze.typen.contains(&t))
					.map(|hp| hp.seite(pos).len())
					.sum::<usize>()
					.try_into()
					.unwrap();
				anz * p
			}
		}
	}
}

fn anz_im_wald<F>(
	wald: impl Iterator<Item = &'static Karte>,
	mut filter: F,
) -> u32
where
	F: FnMut(&'static Karte) -> bool,
{
	wald.filter(|k| filter(k)).count().try_into().unwrap()
}
