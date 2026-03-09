use crate::datentypen::*;

mod einzelkarten;
use super::*;
use einzelkarten::*;

macro_rules! karte_h {
	($links:expr, $links_baumsymbol:ident, $rechts:expr, $rechts_baumsymbol:ident) => {
		&GanzeKarte::ZweigeteiltH {
			links: &kartenvorlage_realisieren(&$links, Baumsymbol::$links_baumsymbol),
			rechts: &kartenvorlage_realisieren(
				&$rechts,
				Baumsymbol::$rechts_baumsymbol,
			),
		}
	};
}
macro_rules! karte_v {
	($oben:expr, $oben_baumsymbol:ident, $unten:expr, $unten_baumsymbol:ident) => {
		&GanzeKarte::ZweigeteiltV {
			oben: &kartenvorlage_realisieren(&$oben, Baumsymbol::$oben_baumsymbol),
			unten: &kartenvorlage_realisieren(&$unten, Baumsymbol::$unten_baumsymbol),
		}
	};
}

/// Alle Spielkarten der Erweiterung
pub const KARTEN: [&GanzeKarte; 36] = [
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Haselnuss,
		Baumsymbol::Eiche,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Haselnuss,
		Baumsymbol::Birke,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Haselnuss,
		Baumsymbol::Kastanie,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Haselnuss,
		Baumsymbol::Buche,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Holunder,
		Baumsymbol::Ahorn,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Holunder,
		Baumsymbol::Eiche,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Holunder,
		Baumsymbol::Birke,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Holunder,
		Baumsymbol::Linde,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Schlehdorn,
		Baumsymbol::Douglasie,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Schlehdorn,
		Baumsymbol::Ahorn,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Schlehdorn,
		Baumsymbol::Tanne,
	)),
	&GanzeKarte::Hauptpflanze(&kartenvorlage_realisieren(
		&Schlehdorn,
		Baumsymbol::Birke,
	)),
	karte_h!(Bache, Tanne, Wildkatze, Kastanie),
	karte_h!(Zwergfledermaus, Linde, Frischling, Tanne),
	karte_h!(Frischling, Kastanie, Schnake, Buche),
	karte_h!(Wildkatze, Ahorn, Schnake, Eiche),
	karte_h!(Zwergfledermaus, Birke, Wisent, Buche),
	karte_h!(Wisent, Buche, Waldiltis, Ahorn),
	karte_h!(Wildkatze, Eiche, Bienenschwarm, Birke),
	karte_h!(Bienenschwarm, Buche, Zwergfledermaus, Ahorn),
	karte_h!(Waldiltis, Tanne, Frischling, Douglasie),
	karte_h!(Schnake, Birke, Waldiltis, Eiche),
	karte_h!(Bache, Birke, Bienenschwarm, Ahorn),
	karte_h!(Wisent, Eiche, Bache, Ahorn),
	karte_v!(Landkärtchen, Linde, Fingerhut, Douglasie),
	karte_v!(Schleiereule, Ahorn, GrünesHeupferd, Eiche),
	karte_v!(Nachtigall, Buche, Fingerhut, Ahorn),
	karte_v!(Elster, Tanne, Brennessel, Kastanie),
	karte_v!(Landkärtchen, Tanne, Schermaus, Ahorn),
	karte_v!(Elster, Birke, GrünesHeupferd, Buche),
	karte_v!(Nachtigall, Eiche, Brennessel, Ahorn),
	karte_v!(Landkärtchen, Eiche, GrünesHeupferd, Tanne),
	karte_v!(Elster, Buche, Fingerhut, Birke),
	karte_v!(Landkärtchen, Ahorn, Brennessel, Birke),
	karte_v!(Nachtigall, Kastanie, Schermaus, Buche),
	karte_v!(Schleiereule, Birke, Fingerhut, Eiche),
];
