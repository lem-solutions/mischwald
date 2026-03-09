use noalloc_vec_rs::vec::Vec as MiniVec;
use smallvec::SmallVec;

use crate::Spielstand;
use crate::datentypen::*;

// Diese API zwingt die `Spieler`-Implementierung Indizes in `Vec`s aus
// `Spielstand` zurück zu geben. Dies und einpaar andere Dinge machen es
// möglich das der Spieler ungültige Entscheidungen zurückgibt. Das ist schlecht
// weil es sowohl für die `Spieler`-Implementierung als auf für die Spiellogik
// Fehlerpotential bietet. Es wäre optimal wenn die Gültigkeit durch das
// Typsystem garantiert werden könnte. Außderdem bieten viele(aber nicht alle)
// Effekte die möglichkeit, auch wenn `effekt_ausüben` mit `true` beantwortet
// wird, den Effekt doch nicht zu nutzen. Das ist nicht wirklich schlimm, ist
// allerdings unnötige API-Oberfläche und könnte verwirrend sein.

#[derive(Copy, Clone)]
pub enum Ausspielziel {
	NeueHauptpflanze,
	AnHauptpflanzeAnlegen {
		hauptpflanze_idx: usize,
		pos: Kartenposition,
	},
}

pub enum ZiehenVon {
	Ziehstapel,
	Lichtung,
}

pub enum Zugaktion {
	KarteAusspielen(Ausspielen),
	SetzlingAusspielen { karten_idx: usize },
	// Sollte der Spieler bereits 9 Karten auf der Hand haben, darf er nur eine
	// ziehen, deswegen kann das zweite `ZiehenVon` `None` sein.
	KartenZiehen(ZiehenVon, Option<ZiehenVon>),
}

pub struct Ausspielen {
	pub karten_idx: usize,
	pub ausspielziel: Ausspielziel,
}

// Das Feld „nächster_spieler“ im Spielstand ist die Numemr des Spielers für
// den eine Entscheidung getroffen werden soll.
pub trait Spieler {
	/// Der Spieler hat keine Bäume in den Anfagshandkarten. Möchte er
	/// seine Handkarten austauschen?
	fn handwechsel_keine_bäume(&mut self, spielstand: &Spielstand) -> bool;

	/// Der Spieler ist dran. Welche Zugaktion möchter er durchführen?
	fn zugkation(&mut self, spielstand: &Spielstand) -> Zugaktion;

	/// Der Spieler muss eine Karte die er ausspielen möchte bezahlen. Mit
	/// Welcher Karte möchte er bezahlen(index aus Handkarten)
	///
	/// Wenn eine Karte Kosten>1 hat wird diese Methode mehrmals aufgerufen.
	fn bezahlen(&mut self, spielstand: &Spielstand) -> usize;

	/// Der Spieler darf einen effekt ausüben. Möchte er diesen Effekt wahrnehmen?
	// brauchen wir eig. nicht: je
	fn effekt_ausüben(
		&mut self,
		spielstand: &Spielstand,
		effekt: &'static Effekt,
	) -> bool;

	/// Der Spieler darf eine Karte von der Lichtung ziehen. Welche möchte er nehmen?
	///
	/// `None` bedeutet das der Spieler zum jetzigen Zeitpunkt keine (weiteren)
	/// Karten aus der Lichtung ziehen möchte, auf wenn er noch welche ziehen
	/// darf.
	fn aus_lichtung_ziehen(&mut self, spielstand: &Spielstand) -> Option<usize>;

	/// Der Spieler darf(zusätzlich zum normalen Zug) eine Karte kostenpflichtig ausspielen,
	/// `None` bedeutet das der Spieler zum jetzigen Zeitpunk keine (weiteren)
	/// Karten ausspielen möchte.
	fn bonus_ausspielen_kostenpflichtig(
		&mut self,
		spielstand: &Spielstand,
	) -> Option<Ausspielen>;

	/// Der Spieler darf eine Karte ausspielen ohne diese zu Bezahlen, allerdings
	/// gelten Soforteffekte und Boni nicht.
	fn bonus_kostenlos_ablegen_typ(
		&mut self,
		spielstand: &Spielstand,
		typ: Typsymbol,
	) -> Option<Ausspielen>;

	/// Der Spieler darf eine Karte ausspielen ohne diese zu Bezahlen, allerdings
	/// gelten Soforteffekte und Boni nicht.
	fn bonus_kostenlos_ablegen_name(
		&mut self,
		spielstand: &Spielstand,
		name: &'static str,
	) -> Option<Ausspielen>;

	/// Der Spieler darf eine Karte als Setzling ausspielen.
	/// Der Rückgabewert ist ein index in die Handkarten.
	fn setzling_ausspielen(&mut self, spielstand: &Spielstand) -> Option<usize>;

	/// Der Spieler darf beliebig viele Handkarten in seine Höhle legen.
	/// Es werden so viele Karten nachgezogen wie abgelegt wurden.
	///
	/// Die Länge des Zurückgegebenen `noalloc_vec_rs::vec::Vec` muss der Anzahl
	/// Handkarten des Spielers entsprechen und bestimmt welche Karten abgelegt
	/// werden(true→ablegen, false→nicht ablegen).
	fn handkarten_höhle(&mut self, spielstand: &Spielstand)
	-> MiniVec<bool, 10>;

	/// Der Spieler darf bis zu `anz_karten` karten aus der Lichtung in seine
	/// Höhle legen.
	/// Die Länge des Zurückgegebenen `noalloc_vec_rs::vec::Vec` muss der Anzahl
	/// Karten in der Lichtung entsprechen und bestimmt welche Karten abgelegt
	/// werden(true→in Höhle, false→in Lichtung liegen lassen).
	fn lichtung_höhle(
		&mut self,
		spielstand: &Spielstand,
		anz_karten: u32,
	) -> SmallVec<[bool; 15]>;

	/// Der Spieler *MUSS* Eine Karte vom `typ` von der Lichtung auf die Hand nehmen. Der Rückgabewert ist ein Index in die Lichtung.
	fn lichtung_hand_typ(
		&mut self,
		spielstand: &Spielstand,
		typ: Typsymbol,
	) -> usize;
}
