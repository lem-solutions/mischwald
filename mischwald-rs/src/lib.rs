pub mod karten;

mod bedingung;
mod effekt;
mod punkteffekt;
mod spieler;
mod spiellogik;
mod spielstand;

pub use spieler::Spieler;
pub use spielstand::Spielstand;

pub mod prelude {
	pub use noalloc_vec_rs::vec::Vec as MiniVec;
	pub use smallvec::SmallVec;

	pub use crate::bedingung::*;
	pub use crate::effekt::*;
	pub use crate::karten::{
		Baumsymbol, GanzeKarte, Karte, Kartenposition, Typsymbol,
	};
	pub use crate::punkteffekt::*;
}
