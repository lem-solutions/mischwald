use crate::bedingung::*;
use crate::karten::{Karte, Kartenposition, Typsymbol};
use crate::spieler::Ausspielziel;
use crate::spielstand::*;

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
		!matches!(self, Dauereffekt::Keiner)
	}

	pub(crate) fn endeffekt(&self) -> &Effekt {
		match self {
			Self::Keiner => panic!("Dauereffekt::Keiner hat keinen Endeffekt"),
			Self::BeiKarteAusspielenTyp(_, effekt) => effekt,
			Self::BeiKarteAusspielenPositionTyp(_, _, effekt) => effekt,
		}
	}

	pub(crate) fn ausgelöster_effekt<'a>(
		&'a self,
		spielstand: &Spielstand,
		karte: &'static Karte,
		ziel: &Ausspielziel,
	) -> Option<&'a Effekt> {
		match self {
			Dauereffekt::Keiner => None,
			Dauereffekt::BeiKarteAusspielenTyp(typsymbol, effekt)
				if karte.typen.contains(typsymbol) =>
			{
				Some(effekt)
			}
			Dauereffekt::BeiKarteAusspielenPositionTyp(
				pos_bedingung,
				hp_typ,
				effekt,
			) => match ziel {
				Ausspielziel::NeueHauptpflanze => None,
				Ausspielziel::AnHauptpflanzeAnlegen {
					hauptpflanze_idx,
					pos,
				} => {
					let hauptfplanze = &spielstand.nächster_spielerstand().wald
						[*hauptpflanze_idx]
						.hauptpflanze;
					if pos == pos_bedingung && hauptfplanze.typen.contains(hp_typ) {
						Some(effekt)
					} else {
						None
					}
				}
			},
			_ => None,
		}
	}
}
