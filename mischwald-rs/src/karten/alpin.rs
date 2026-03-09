use crate::datentypen::*;

mod einzelkarten;
use super::*;
use einzelkarten::*;

macro_rules! karte_h {
	($links:expr, $links_baumsymbol:ident, $rechts:expr, $rechts_baumsymbol:ident) => {
		&GanzeKarte::ZweigeteiltH {
			links: &kartenvorlage_realisieren(&$links, Baumsymbol::$links_baumsymbol),
			rechts: &kartenvorlage_realisieren(&$rechts, Baumsymbol::$rechts_baumsymbol),
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

/// (fast) Alle Spielkarten der Erweiterung.
///
/// Eine vertikal getielte Karte fehlt!
pub const KARTEN: [&GanzeKarte; 35] = [
	&EuropäischeLärche,
	&EuropäischeLärche,
	&EuropäischeLärche,
	&EuropäischeLärche,
	&EuropäischeLärche,
	&EuropäischeLärche,
	&EuropäischeLärche,
	&Zirbelkiefer,
	&Zirbelkiefer,
	&Zirbelkiefer,
	&Zirbelkiefer,
	&Zirbelkiefer,
	&Zirbelkiefer,
	&Zirbelkiefer,
	karte_h!(Alpenfledermaus, Lärche, Schneehase, Zirbelkiefer),
	karte_h!(Alpenfledermaus, Tanne, Steinbock, Zirbelkiefer),
	karte_h!(Auerhuhn, Zirbelkiefer, Steinbock, Douglasie),
	karte_h!(Schneehase, Lärche, Gämse, Douglasie),
	karte_h!(Alpenmurmeltier, Buche, Gämse, Zirbelkiefer),
	karte_h!(Steinbock, Lärche, Alpenmurmeltier, Birke),
	karte_h!(Alpenmurmeltier, Lärche, Auerhuhn, Douglasie),
	karte_h!(Auerhuhn, Lärche, Alpenfledermaus, Zirbelkiefer),
	karte_h!(Schneehase, Tanne, Alpenmurmeltier, Zirbelkiefer),
	karte_h!(Gämse, Lärche, Auerhuhn, Buche),
	karte_v!(Bartgeier, Tanne, Enzian, Lärche),
	karte_v!(Bartgeier, Zirbelkiefer, Edelweiß, Lärche),
	karte_v!(Steinadler, Tanne, Enzian, Zirbelkiefer),
	karte_v!(AlpenApollofalter, Tanne, Bergmolch, Zirbelkiefer),
	karte_v!(AlpenApollofalter, Douglasie, Edelweiß, Zirbelkiefer),
	karte_v!(AlpenApollofalter, Lärche, Heidelbeere, Birke),
	karte_v!(Bartgeier, Lärche, Bergmolch, Tanne),
	karte_v!(Kolkrabe, Douglasie, Heidelbeere, Zirbelkiefer),
	karte_v!(Steinadler, Buche, Herbsttrompete, Zirbelkiefer),
	karte_v!(AlpenApollofalter, Zirbelkiefer, Herbsttrompete, Lärche),
	karte_v!(Steinadler, Lärche, Bergmolch, Douglasie),
	// Eine vertikal geteilte Karte fehlt!!!
];
