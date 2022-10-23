use std::fmt::Display;

use crate::player::BPUnlock;

#[derive(Debug, Clone, PartialEq)]
pub enum Tree {
	Pine(BPUnlock),
	Oak(BPUnlock),
	Maple(BPUnlock),
	Walnut(BPUnlock),
	Cherry(BPUnlock),
	PurpleHeart(BPUnlock),
}

impl Display for Tree {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Tree::Pine(b) => write!(f, "Pine {}", b),
			Tree::Oak(b) => write!(f, "Oak {}", b),
			Tree::Maple(b) => write!(f, "Maple {}", b),
			Tree::Walnut(b) => write!(f, "Walnut {}", b),
			Tree::Cherry(b) => write!(f, "Cherry {}", b),
			Tree::PurpleHeart(b) => write!(f, "Purple Heart {}", b),
		}
	}
}