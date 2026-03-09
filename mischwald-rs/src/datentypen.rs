pub type Typsymbole = &'static [Typsymbol];

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
			(GanzeKarte::ZweigeteiltH { links: k, .. }, Kartenposition::Links) |
			(GanzeKarte::ZweigeteiltH { rechts: k, .. }, Kartenposition::Rechts) |
			(GanzeKarte::ZweigeteiltV { oben: k, .. }, Kartenposition::Oben) |
			(GanzeKarte::ZweigeteiltV { unten: k, .. }, Kartenposition::Unten) =>
			{
				k
			},
			_ => panic!("Ungültike Kartenposition für Ganze Karte"),
		}
	}
	
	pub fn hat_typsymbol(&self, symbol: &Typsymbol) -> bool {
		match self {
			Self::Hauptpflanze(a) => a.typen.contains(symbol),
			Self::ZweigeteiltH { links: a, rechts: b } |
			Self::ZweigeteiltV { oben: a, unten: b } => {
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

#[derive(Clone, Copy, PartialEq)]
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
	pub typen: Typsymbole,
	pub soforteffekt: Effekt,
	pub dauereffekt: Dauereffekt,
	pub bonus: Effekt,
	pub punkte: Punkteffekt,
	pub bezeichnung: &'static str, // wird für manche Effekte gebraucht
}

#[derive(Clone, Copy)]
pub enum Effekt {
	Keiner,
	/// Führe nach diesem einen weiteren vollständigen Spielzug durch. Dabei hast
	/// du wie immer die Wahl zwischen den Aktionen A und B.
	Extrazug,
	/// Ziehe so viele Karten vom Stabel wie angegeben.
	KartenZiehen(u32),
	/// Nimm x Karten aus der Lichtung auf die Hand.
	KartenZiehenLichtung(u32),
	/// Ziehe so viele Karten vom Stapel, wie sich Karten mit dem Symbol in deinem
	/// Wald befinden. (Handkartenlimit beachten.)
	KartenZiehenAnzTyp(Typsymbol),
	/// Ziehe so viele Karten vom Stapel, wie sich Karten mit dieser Bezeichnung
	/// in deinem Wald befinden. (Handkartenlimit beachten.)
	KartenZiehenAnzName(&'static str),
	/// Du kannst in diesem Zug beliebig viele Karten ausspielen, musst aber deren
	/// Kosten bezahlen.
	BeliebigKartenKostenpflichtigSpielen,
	/// Lege eine Karte mit entsprechendem Typsymbol von deiner Hand in deinen
	/// Wald. Dieses brauchst du nicht zu bezahlen, darfst aber weder seinen
	/// Effekt noch seinen Bonus nutzen.
	EinsKostenlosAblegen(Typsymbol),
	/// Zwei mal `EinsKostenlosAblegen`.
	EinsKostenlosAblegen2x(Typsymbol, Typsymbol),
	/// Lege eine entsprechende Karte von deiner Hand in deinen Wald. Dieses
	/// brauchst du nicht zu bezahlen, darfst aber weder seinen Effekt noch seinen
	/// Bonus nutzen.
	EinsKostenlosAblegenName(&'static str),
	/// Beliebig oft `EinsKostenlosAblegen`.
	BeliebigKostenlosAblegen(Typsymbol),
	/// Spiele beliebig viele Karten aus deiner Hand als Sprösslinge in deinen
	/// Wald.
	BeliebigSprösslingeAusspielen,
	/// Lege beliebig viele Handkarten in deine Höhle. Ziehe genauso viele vom
	/// Stapel nach.
	HandkartenHöle,
	/// Lege n Karten von der Lichtung in deine Höhle.
	LichtungHöhle(u32),
	/// Lege alle Karten aus der Lichtung in deine Höhle.
	LichtungHöleAlle,
	/// Lege folgende Karten aus der Lichtung in deine Höhle
	LichtungHöhleTypen(&'static [Typsymbol]),
	/// alle Karten mit dem Typsymbol aus der Lichtung auf die Hand
	LichtungHandTyp(Typsymbol),
	/// Nimm alle Karten aus der Lichtung aus dem Spiel
	LichtungLeeren,
	/// `KartenZiehen(1)` plus `Extrazug`
	KarteZiehenPlusExtrazug,
	/// Der bedingte Effekt darf nicht `Effekt::Keiner` sein.
	Bedingung(&'static Effekt, Bedingung),
}


// TODO Evtl. Structartige Enumvarianten statt Tupleartigen?
#[derive(Clone, Copy)]
pub enum Dauereffekt {
	Keiner,
	/// Immer wenn du eine Karte ausspielst mit [Typsymbol]: ...
	BeiKarteAusspielenTyp(Typsymbol, Effekt),
	/// Immer wenn du eine Karte Oben/Unten/Links/Rechts an eine Hauptpflanze mit
	/// [Typsymbol] legst: ...
	BeiKarteAusspielenPositionTyp(Kartenposition, Typsymbol, Effekt),
}
impl Dauereffekt {
	pub(crate) fn ist_einer(&self) -> bool {
		match self {
			Dauereffekt::Keiner => false,
			_ => true,
		}
	}
	
	pub(crate) fn endeffekt(&self) -> &Effekt {
		match self {
			Self::Keiner => panic!("Dauereffekt::Keiner hat keinen Endeffekt"),
			Self::BeiKarteAusspielenTyp(_, effekt) => &effekt,
			Self::BeiKarteAusspielenPositionTyp(_, _, effekt) => &effekt,
		}
	}
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
	BedingungOder{bedingung: Bedingung, wenn: u32, sonst: u32},
	// Bewertung nach Anzahl der Karten(nach Namen)
	// Wenn die Anzahl mehr als das Maximum der Bewertungsliste ist wird das höchste Element gewählt
	SammlungName(&'static [u32]),
	// Bewertung nach Anzahl der *verschiedenen* Karten mit dem gegebenen Typ.
	SammlungTypVersch(Typsymbol, &'static [u32]),
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
