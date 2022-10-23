use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum BPUnlock {
	None,
	BirdHouse,
	Shelf,
	SideTable,
	CoffeeTable,
	DiningSet
}

impl Display for BPUnlock {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			BPUnlock::None => write!(f, ""),
			BPUnlock::BirdHouse => write!(f, "Bird House"),
			BPUnlock::Shelf => write!(f, "Shelf"),
			BPUnlock::SideTable => write!(f, "Side Table"),
			BPUnlock::CoffeeTable => write!(f, "Coffee Table"),
			BPUnlock::DiningSet => write!(f, "Dining Set"),
		}
	}
}