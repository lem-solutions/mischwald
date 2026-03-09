#![allow(non_upper_case_globals)]

use crate::datentypen::*;
use crate::karten::{Kartenvorlage, SAMMLUNG_SCHMETTERLINGE};
use Typsymbol::*;

// TODO Sonderfunktionen:
//      Brennessel: Beliebig viele Schmetterlinge and diesem [Strauch], [Baum]

// Horizontal

pub(super) const Wisent: Kartenvorlage = Kartenvorlage {
	kosten: 3,
	typen: &[Paarhufer, Waldrand],
	soforteffekt: Effekt::Extrazug,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProBaumsymbole(
		2,
		&[Baumsymbol::Buche, Baumsymbol::Eiche],
	),
	bezeichnung: "Wisent",
};

pub(super) const Bache: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Paarhufer, Waldrand],
	soforteffekt: Effekt::LichtungLeeren,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::EinsKostenlosAblegenName("Frischling"),
	punkte: Punkteffekt::ProName(10, "Frischling"),
	bezeichnung: "Bache",
};

pub(super) const Bienenschwarm: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Insekt, Waldrand],
	soforteffekt: Effekt::LichtungHöhleTypen(&[Pflanze, Strauch, Baum]),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(1, Pflanze),
	bezeichnung: "Bienenschwarm",
};

pub(super) const Schnake: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Insekt, Waldrand],
	soforteffekt: Effekt::BeliebigKostenlosAblegen(Fledermaus),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::LichtungHandTyp(Fledermaus),
	punkte: Punkteffekt::ProTyp(1, Fledermaus),
	bezeichnung: "Schnake",
};

pub(super) const Waldiltis: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Pfotentier, Waldrand],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Extrazug,
	punkte: Punkteffekt::Bedingung(
		10,
		Bedingung::Hauptpflanzentyp(&[Baum, Strauch]),
	),
	bezeichnung: "Waldiltis",
};

pub(super) const Frischling: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Paarhufer, Waldrand],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Konstant(1),
	bezeichnung: "Frischling",
};

pub(super) const Zwergfledermaus: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Fledermaus, Waldrand],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(
		5,
		Bedingung::MinAnzTypVerschieden(3, Fledermaus),
	),
	bezeichnung: "Zwergfledermaus",
};

pub(super) const Wildkatze: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pfotentier, Waldrand],
	soforteffekt: Effekt::KartenZiehenLichtung(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(1, Waldrand),
	bezeichnung: "Wildkatze",
};

// Vertikal

pub(super) const Schleiereule: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Vogel, Waldrand],
	soforteffekt: Effekt::Bedingung(
		&Effekt::Extrazug,
		Bedingung::MinAnzTyp(1, Fledermaus),
	),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(3, Fledermaus),
	bezeichnung: "Schleiereule",
};

pub(super) const Fingerhut: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Pflanze, Waldrand],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungTypVersch(Pflanze, &[1, 2, 6, 10, 15]),
	bezeichnung: "Fingerhut",
};

pub(super) const Nachtigall: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel, Waldrand],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Extrazug,
	punkte: Punkteffekt::Bedingung(5, Bedingung::Hauptpflanzentyp(&[Strauch])),
	bezeichnung: "Nachtigall",
};

pub(super) const Schermaus: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Pfotentier, Waldrand],
	soforteffekt: Effekt::BeliebigSprösslingeAusspielen,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Extrazug,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Schermaus",
};

pub(super) const Landkärtchen: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Insekt, Schmetterling, Waldrand],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungTypVersch(
		Schmetterling,
		SAMMLUNG_SCHMETTERLINGE,
	),
	bezeichnung: "Landkärtchen",
};

pub(super) const Brennessel: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Pflanze, Waldrand],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(2, Schmetterling),
	bezeichnung: "Brennessel",
};

pub(super) const Elster: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel, Waldrand],
	soforteffekt: Effekt::KartenZiehenLichtung(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::LichtungHöhle(2),
	punkte: Punkteffekt::Konstant(3),
	bezeichnung: "Elster",
};

pub(super) const GrünesHeupferd: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Insekt, Waldrand],
	soforteffekt: Effekt::EinsKostenlosAblegen(Vogel),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(1, Insekt),
	bezeichnung: "Grünes Heupferd",
};
// Sträucher

pub(super) const Schlehdorn: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Waldrand, Strauch],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::BeiKarteAusspielenTyp(
		Schmetterling,
		Effekt::KartenZiehen(1),
	),
	bonus: Effekt::EinsKostenlosAblegen(Schmetterling),
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Schlehdorn",
};

pub(super) const Haselnuss: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Waldrand, Strauch],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::BeiKarteAusspielenTyp(
		Fledermaus,
		Effekt::KartenZiehen(1),
	),
	bonus: Effekt::EinsKostenlosAblegen(Fledermaus),
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Haselnuss",
};

pub(super) const Holunder: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Waldrand, Strauch],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::BeiKarteAusspielenTyp(
		Pflanze,
		Effekt::KartenZiehen(1),
	),
	bonus: Effekt::EinsKostenlosAblegen(Pflanze),
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Holunder",
};
