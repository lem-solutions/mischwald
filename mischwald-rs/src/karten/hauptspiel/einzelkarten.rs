#![allow(non_upper_case_globals)]

use crate::datentypen::*;
use crate::karten::*;
use Typsymbol::*;

// TODO Sonderfunktionen:
//      Feldhase: Beliebig viele davon an desem Platz
//      Erdkröte: Bis zu 2 davon an diesem Platz
//      Holzbiene: Bringt selbst keine Punkte, erhöht aber die Anzahl Bäume, an dem sie liegt, um 1

// Horizontal
pub(super) const Feldhase: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Pfotentier],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProName(1, "Feldhase"),
	bezeichnung: "Feldhase",
};

pub(super) const Dachs: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pfotentier],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::EinsKostenlosAblegen(Pfotentier),
	punkte: Punkteffekt::Konstant(2),
	bezeichnung: "Dachs",
};

pub(super) const Bechsteinfledermaus: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Fledermaus],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(5, Bedingung::MinAnzTypVerschieden(3, Fledermaus)),
	bezeichnung: "Bechsteinfledermaus",
};

pub(super) const Wolf: Kartenvorlage = Kartenvorlage {
	kosten: 3,
	typen: &[Pfotentier],
	soforteffekt: Effekt::KartenZiehenAnzTyp(Typsymbol::Hirsch),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Extrazug,
	punkte: Punkteffekt::ProTyp(5, Hirsch),
	bezeichnung: "Wolf",
};

pub(super) const Waschbär: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pfotentier],
	soforteffekt: Effekt::HandkartenHöle,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Waschbär",
};

pub(super) const Reh: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Hirsch, Paarhufer],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::KartenZiehen(1),
	punkte: Punkteffekt::ProBaumsymbole(3, &[Baumsymbol::Buche]),
	bezeichnung: "Reh",
};

pub(super) const Rothirsch: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Hirsch, Paarhufer],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::EinsKostenlosAblegen(Hirsch),
	punkte: Punkteffekt::ProTypen(1, &[Baum, Pflanze]),
	bezeichnung: "Rothirsch",
};

pub(super) const Braunbär: Kartenvorlage = Kartenvorlage {
	kosten: 3,
	typen: &[Pfotentier],
	soforteffekt: Effekt::LichtungHöleAlle,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::KarteZiehenPlusExtrazug,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Braunbär",
};

pub(super) const Frischling: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Paarhufer],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Konstant(1),
	bezeichnung: "Frischling",
};

pub(super) const Stechmücke: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Insekt],
	soforteffekt: Effekt::BeliebigKostenlosAblegen(Fledermaus),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(1, Fledermaus),
	bezeichnung: "Stechmücke",
};

pub(super) const Holzbiene: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Insekt],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Holzbiene",
};

pub(super) const Steinmarder: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pfotentier],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProVollbelegtemBaum(5),
	bezeichnung: "Steinmarder",
};

pub(super) const Wildschwein: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Paarhufer],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(10, Bedingung::MinAnzNamen(1, "Frischling")),
	bezeichnung: "Wildschwein",
};

pub(super) const Fuchs: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Pfotentier],
	soforteffekt: Effekt::KartenZiehenAnzName("Feldhase"),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProName(2, "Feldhase"),
	bezeichnung: "Fuchs",
};

pub(super) const Luchs: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pfotentier],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(10, Bedingung::MinAnzNamen(1, "Reh")),
	bezeichnung: "Luchs",
};

pub(super) const Damhirsch: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Hirsch, Paarhufer],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::KartenZiehen(2),
	punkte: Punkteffekt::ProTyp(3, Paarhufer),
	bezeichnung: "Damhirsch",
};

pub(super) const BraunesLangohr: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Fledermaus],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(5, Bedingung::MinAnzTypVerschieden(3, Fledermaus)),
	bezeichnung: "Braunes Langohr",
};

pub(super) const Mopsfledermaus: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Fledermaus],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(5, Bedingung::MinAnzTypVerschieden(3, Fledermaus)),
	bezeichnung: "Mopsfledermaus",
};

pub(super) const Hufeisennase: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Fledermaus],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(5, Bedingung::MinAnzTypVerschieden(3, Fledermaus)),
	bezeichnung: "Hufeisennase",
};

pub(super) const Siebenschläfer: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pfotentier],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(15, Bedingung::TypGegenüber(Fledermaus)),
	bezeichnung: "Siebenschläfer",
};

// Vertikal
pub(super) const Schillerfalter: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Schmetterling, Insekt],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungTypVersch(Schmetterling, SAMMLUNG_SCHMETTERLINGE),
	bezeichnung: "Schillerfalter",
};

pub(super) const Tagpfauenauge: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Schmetterling, Insekt],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungTypVersch(Schmetterling, SAMMLUNG_SCHMETTERLINGE),
	bezeichnung: "Tagpfauenauge",
};

pub(super) const Kaisermantel: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Schmetterling, Insekt],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungTypVersch(Schmetterling, SAMMLUNG_SCHMETTERLINGE),
	bezeichnung: "Kaisermantel",
};

pub(super) const GroßerFuchs: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Schmetterling, Insekt],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungTypVersch(Schmetterling, SAMMLUNG_SCHMETTERLINGE),
	bezeichnung: "GroßerFuchs",
};

pub(super) const Trauermantel: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Schmetterling, Insekt],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungTypVersch(Schmetterling, SAMMLUNG_SCHMETTERLINGE),
	bezeichnung: "Trauermantel",
};

pub(super) const Pfifferling: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Pilz],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::BeiKarteAusspielenTyp(Baum, Effekt::KartenZiehen(1)),
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Pfifferling",
};

pub(super) const Laubfrosch: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Amphibie],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProName(5, "Stechmücke"),
	bezeichnung: "Pfifferling",
};

pub(super) const Glühwürmchen: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Insekt],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungName(&[0, 10, 15, 20]),
	bezeichnung: "Glühwürmchen",
};

pub(super) const Sumpfschildkröte: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Amphibie],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Konstant(5),
	bezeichnung: "Sumpfschildkröte",
};

pub(super) const Maulwurf: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Pfotentier],
	soforteffekt: Effekt::BeliebigKartenKostenpflichtigSpielen,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Maulwurf",
};

pub(super) const Feuersalamander: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Amphibie],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::EinsKostenlosAblegen(Pfotentier),
	punkte: Punkteffekt::SammlungName(&[5, 15, 25]),
	bezeichnung: "Feuersalamander",
};

pub(super) const Brombeeren: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Pflanze],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(2, Pflanze),
	bezeichnung: "Brombeeren",
};

pub(super) const Moos: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Pflanze],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(10, Bedingung::MinAnzTyp(10, Baum)),
	bezeichnung: "Moos",
};

pub(super) const Erdkröte: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Amphibie],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(5, Bedingung::MinAnzNamenAnPlatz(2, "Erdkröte")),
	bezeichnung: "Erdkröte",
};

pub(super) const Igel: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pfotentier],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::KartenZiehen(1),
	punkte: Punkteffekt::ProTyp(2, Schmetterling),
	bezeichnung: "Igel",
};

pub(super) const Fliegenpilz: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Pilz],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::BeiKarteAusspielenTyp(Pfotentier, Effekt::KartenZiehen(1)),
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Fliegenpilz",
};

pub(super) const Gimpel: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(2, Insekt),
	bezeichnung: "Gimpel",
};

pub(super) const Eichelhäher: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel],
	soforteffekt: Effekt::Extrazug,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Konstant(3),
	bezeichnung: "Eichelhäher",
};

pub(super) const Buchfink: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(5, Bedingung::Hauptpflanzenname("Buche")),
	bezeichnung: "Buchfink",
};

pub(super) const Parasol: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Pilz],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::BeiKarteAusspielenPositionTyp(
		Kartenposition::Unten,
		Baum,
		Effekt::KartenZiehen(1),
	),
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Parasol",
};

pub(super) const Buntspecht: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Vogel],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(10, Bedingung::MeisteAnzTyp(Baum)),
	bezeichnung: "Buntspecht",
};

pub(super) const Waldameise: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Insekt],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProKartenAnTypPosition(2, Baum, Kartenposition::Unten),
	bezeichnung: "Waldameise",
};

pub(super) const Baumfarn: Kartenvorlage = Kartenvorlage {
	kosten: 1,
	typen: &[Pflanze],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(6, Amphibie),
	bezeichnung: "Baumfarn",
};

pub(super) const Waldkauz: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Vogel],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::KartenZiehen(2),
	punkte: Punkteffekt::Konstant(5),
	bezeichnung: "Waldkauz",
};

pub(super) const Hirschkäfer: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Insekt],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::EinsKostenlosAblegen(Vogel),
	punkte: Punkteffekt::ProTyp(1, Pfotentier),
	bezeichnung: "Hirschkäfer",
};

pub(super) const Habicht: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Vogel],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(3, Vogel),
	bezeichnung: "Habicht",
};

pub(super) const Steinpilz: Kartenvorlage = Kartenvorlage {
	kosten: 2,
	typen: &[Pilz],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::BeiKarteAusspielenPositionTyp(
		Kartenposition::Unten,
		Baum,
		Effekt::KartenZiehen(1),
	),
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Keiner,
	bezeichnung: "Steinpilz",
};

pub(super) const Walderdbeeren: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Pflanze],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(10, Bedingung::MinAnzTypVerschieden(8, Baum)),
	bezeichnung: "Waldbeeren",
};

pub(super) const Eichhörnchen: Kartenvorlage = Kartenvorlage {
	kosten: 0,
	typen: &[Pfotentier],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(5, Bedingung::Hauptpflanzenname("Eiche")),
	bezeichnung: "Waldbeeren",
};

// Bäume

pub(super) const Douglasie: GanzeKarte = GanzeKarte::Hauptpflanze(&Karte {
	kosten: 2,
	baumsymbol: Baumsymbol::Douglasie,
	typen: &[Baum],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Extrazug,
	punkte: Punkteffekt::Konstant(5),
	bezeichnung: "Douglasie",
});

pub(super) const Eiche: GanzeKarte = GanzeKarte::Hauptpflanze(&Karte {
	kosten: 2,
	baumsymbol: Baumsymbol::Eiche,
	typen: &[Baum],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Extrazug,
	punkte: Punkteffekt::Bedingung(10, Bedingung::MinAnzTypVerschieden(8, Baum)),
	bezeichnung: "Eiche",
});

pub(super) const Birke: GanzeKarte = GanzeKarte::Hauptpflanze(&Karte {
	kosten: 0,
	baumsymbol: Baumsymbol::Birke,
	typen: &[Baum],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Konstant(1),
	bezeichnung: "Birke",
});

pub(super) const Ahorn: GanzeKarte = GanzeKarte::Hauptpflanze(&Karte {
	kosten: 2,
	baumsymbol: Baumsymbol::Ahorn,
	typen: &[Baum],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::ProTyp(1, Baum),
	bezeichnung: "Ahorn",
});

pub(super) const Tanne: GanzeKarte = GanzeKarte::Hauptpflanze(&Karte {
	kosten: 2,
	baumsymbol: Baumsymbol::Tanne,
	typen: &[Baum],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::EinsKostenlosAblegen(Pfotentier),
	punkte: Punkteffekt::ProKarteAnHauptpflanze(2),
	bezeichnung: "Tanne",
});

pub(super) const Linde: GanzeKarte = GanzeKarte::Hauptpflanze(&Karte {
	kosten: 1,
	baumsymbol: Baumsymbol::Linde,
	typen: &[Baum],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::BedingungOder {
		bedingung: Bedingung::MeisteNamen("Linde"),
		wenn: 3,
		sonst: 1,
	},
	bezeichnung: "Linde",
});

pub(super) const Kastanie: GanzeKarte = GanzeKarte::Hauptpflanze(&Karte {
	kosten: 1,
	baumsymbol: Baumsymbol::Kastanie,
	typen: &[Baum],
	soforteffekt: Effekt::Keiner,
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::SammlungName(&[1, 4, 9, 16, 25, 36, 49]),
	bezeichnung: "Kastanie",
});

pub(super) const Buche: GanzeKarte = GanzeKarte::Hauptpflanze(&Karte {
	kosten: 1,
	baumsymbol: Baumsymbol::Buche,
	typen: &[Baum],
	soforteffekt: Effekt::KartenZiehen(1),
	dauereffekt: Dauereffekt::Keiner,
	bonus: Effekt::Keiner,
	punkte: Punkteffekt::Bedingung(5, Bedingung::MinAnzNamen(4, "Buche")),
	bezeichnung: "Buche",
});
