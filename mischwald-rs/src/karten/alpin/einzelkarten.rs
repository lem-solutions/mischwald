#![allow(non_upper_case_globals)]

use crate::datentypen::*;
use crate::karten::{Kartenvorlage, SAMMLUNG_SCHMETTERLINGE};
use Typsymbol::*;

// TODO Sonderfunktionen:
//      „Schneehase“ zählt als „Hase“ für die Wertung

// Horizontal
pub(super) const Gämse: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Paarhufer, Alpen],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProBaumsymbole(3, &[Baumsymbol::Lärche]),
	bezeichnung: "Gämse",
};

pub(super) const Auerhuhn: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel, Alpen],
	soforteffekt: Effekt::EinsKostenlosAblegen(Pflanze),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(1, Pflanze),
	bezeichnung: "Auerhuhn",
};

pub(super) const Schneehase: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Pfotentier, Alpen],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProName(1, "Hase"),
	bezeichnung: "Schneehase",
};

pub(super) const Alpenmurmeltier: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pfotentier, Alpen],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTypUnterschiedlich(3, Pflanze),
	bezeichnung: "Alpenmurmeltier",
};

pub(super) const Alpenfledermaus: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Fledermaus, Alpen],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(
		5,
		Bedingung::MinAnzTypVerschieden(3, Fledermaus),
	),
	bezeichnung: "Alpenfledermaus",
};

pub(super) const Steinbock: Kartenvorlage = Kartenvorlage {
	kosten: 3,
	typen: &[Paarhufer, Alpen],
	soforteffekt: Effekt::Extrazug,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Konstant(10),
	bezeichnung: "Steinbock",
};

// Vertikal

pub(super) const AlpenApollofalter: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Schmetterling, Insekt, Alpen],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungTypVersch(
		Schmetterling,
		SAMMLUNG_SCHMETTERLINGE,
	),
	bezeichnung: "Alpen-Apollofalter",
};

pub(super) const Herbsttrompete: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Pilz, Alpen],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::BeiKarteAusspielenTyp(
		Alpen,
		Effekt::KartenZiehen(1),
	),
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Herbsttrompete",
};

pub(super) const Steinadler: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel, Alpen],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTypen(1, &[Pfotentier, Amphibie]),
	bezeichnung: "Steinadler",
};

pub(super) const Kolkrabe: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel, Alpen],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Konstant(5),
	bezeichnung: "Kolkrabe",
};

pub(super) const Heidelbeere: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pflanze, Alpen],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::EinsKostenlosAblegen(Amphibie),
	punkte: Punkteffekt::ProTypUnterschiedlich(2, Vogel),
	bezeichnung: "Heidelbeere",
};

pub(super) const Bartgeier: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel, Alpen],
	soforteffekt: Effekt::LichtungHöhle(2),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProKarteInHöhle(1),
	bezeichnung: "Bartgeier",
};

pub(super) const Bergmolch: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Amphibie, Alpen],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::EinsKostenlosAblegen2x(Alpen, Insekt),
	punkte: Punkteffekt::ProTyp(2, Insekt),
	bezeichnung: "Bergmolch",
};

pub(super) const Edelweiß: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pflanze, Alpen],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::KartenZiehen(1),
	punkte: Punkteffekt::Konstant(3),
	bezeichnung: "Edelweiß",
};

pub(super) const Enzian: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Pflanze, Alpen],
	soforteffekt: Effekt::EinsKostenlosAblegen(Schmetterling),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(3, Schmetterling),
	bezeichnung: "Enzian",
};

// Bäume

pub(super) const Zirbelkiefer: GanzeKarte = GanzeKarte::Hauptpflanze(&Karte {
	kosten: 2,
	baumsymbol: Baumsymbol::Zirbelkiefer,
	typen: &[Baum, Alpen],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::KartenZiehen(1),
	punkte: Punkteffekt::ProTyp(1, Alpen),
	bezeichnung: "Zirbelkiefer",
});

pub(super) const EuropäischeLärche: GanzeKarte =
	GanzeKarte::Hauptpflanze(&Karte {
		kosten: 1,
		baumsymbol: Baumsymbol::Lärche,
		typen: &[Baum, Alpen],
		soforteffekt: Effekt::Keiner,
		dauereffekt: Dauereffekt::Keiner,
		bonus: Effekt::EinsKostenlosAblegen(Alpen),
		punkte: Punkteffekt::Konstant(3),
		bezeichnung: "Europäische Lärche",
	});
