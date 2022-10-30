use std::fmt::Display;

use crate::player::BPUnlock;

#[derive(Debug, Clone, PartialEq)]
pub enum TreeBPUnlock {
	Pine(BPUnlock),
	Oak(BPUnlock),
	Maple(BPUnlock),
	Walnut(BPUnlock),
	Cherry(BPUnlock),
	PurpleHeart(BPUnlock),
}

impl Display for TreeBPUnlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			TreeBPUnlock::Pine(b) => write!(f, "Pine {}", b),
			TreeBPUnlock::Oak(b) => write!(f, "Oak {}", b),
			TreeBPUnlock::Maple(b) => write!(f, "Maple {}", b),
			TreeBPUnlock::Walnut(b) => write!(f, "Walnut {}", b),
			TreeBPUnlock::Cherry(b) => write!(f, "Cherry {}", b),
			TreeBPUnlock::PurpleHeart(b) => write!(f, "Purple Heart {}", b),
		}
	}
}