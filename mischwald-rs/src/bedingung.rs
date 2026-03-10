use crate::karten::Kartenposition;
use crate::{Spielstand, prelude::*};

/// Manche Bedingungen hängen von der Position der Karte auf der die Bedingung steht ab. `Bedingungskontext` beschreibt diese Position.
#[derive(Copy, Clone)]
pub struct Bedingungskontext {
	pub hauptpflanze_idx: usize,
	pub kartenpos: Option<Kartenposition>,
}

#[derive(Clone, Copy)]
pub enum Bedingung {
	MeisteNamen(&'static str),
	MeisteAnzTyp(Typsymbol),
	MinAnzNamen(u32, &'static str),
	MinAnzTypVerschieden(u32, Typsymbol),
	MinAnzTyp(u32, Typsymbol),
	TypGegenüber(Typsymbol),
	MinAnzNamenAnPlatz(u32, &'static str),
	Hauptpflanzenname(&'static str),
	Hauptpflanzentyp(&'static [Typsymbol]),
}
impl Bedingung {
	pub fn prüfen(
		&self,
		spielstand: &Spielstand,
		kontext: &Bedingungskontext,
	) -> bool {
		match self {
			Bedingung::MeisteNamen(name) => {
				Self::bedinung_meiste(spielstand, |&k| &k.bezeichnung == name)
			}
			Bedingung::MeisteAnzTyp(typ) => {
				Self::bedinung_meiste(spielstand, |&k| k.typen.contains(typ))
			}
			Bedingung::MinAnzNamen(min, name) => {
				Self::bedingung_min_anz(spielstand, *min, |&k| &k.bezeichnung == name)
			}
			Bedingung::MinAnzTypVerschieden(min, typ) => {
				let mut gesehen: SmallVec<[&'static str; 5]> = SmallVec::new();
				let f = |&k: &&Karte| {
					let neu = k.typen.contains(typ) && !gesehen.contains(&k.bezeichnung);
					if neu {
						gesehen.push(k.bezeichnung);
					}
					neu
				};
				Self::bedingung_min_anz(spielstand, *min, f)
			}
			Bedingung::MinAnzTyp(min, typ) => {
				Self::bedingung_min_anz(spielstand, *min, |&k| k.typen.contains(typ))
			}
			Bedingung::TypGegenüber(typsymbol) => {
				spielstand.nächster_spielerstand().wald[kontext.hauptpflanze_idx]
					.seite(kontext.kartenpos.expect(
						"Kartenposition muss für `Bedingung::TypGegenüber` gegeben sein",
					))
					.iter()
					.any(|&k| k.typen.contains(typsymbol))
			}
			Bedingung::MinAnzNamenAnPlatz(anz, name) => {
				spielstand.nächster_spielerstand().wald[kontext.hauptpflanze_idx]
					.seite(kontext.kartenpos.expect(
						"Kartenposition muss für `Bedingung::MinAnzNamenAnPlatz` gegeben sein",
					))
					.iter()
					.filter(|k| &k.bezeichnung == name)
					.count()
					>= *anz as usize
			}
			Bedingung::Hauptpflanzenname(name) => {
				&spielstand.nächster_spielerstand().wald[kontext.hauptpflanze_idx]
					.hauptpflanze
					.bezeichnung
					== name
			}
			Bedingung::Hauptpflanzentyp(typen) => {
				spielstand.nächster_spielerstand().wald[kontext.hauptpflanze_idx]
					.hauptpflanze
					.typen
					.iter()
					.any(|&t| typen.contains(&t))
			}
		}
	}

	fn bedinung_meiste<B>(spielstand: &Spielstand, b: B) -> bool
	where
		B: for<'a> Fn(&'a &'static Karte) -> bool,
	{
		// unwrap: es muss mindestens einen Spieler geben(eig. sogar zwei),
		//         deswegen kann `.max()` nie `None` sein.
		let max = spielstand
			.spieler
			.iter()
			.map(|s| s.iter_wald().filter(&b).count())
			.max()
			.unwrap();
		let anz = spielstand
			.nächster_spielerstand()
			.iter_wald()
			.filter(&b)
			.count();

		anz == max
	}

	fn bedingung_min_anz<B>(spielstand: &Spielstand, min: u32, b: B) -> bool
	where
		B: for<'a> FnMut(&'a &'static Karte) -> bool,
	{
		spielstand
			.nächster_spielerstand()
			.iter_wald()
			.filter(b)
			.count() as u32
			>= min
	}
}
