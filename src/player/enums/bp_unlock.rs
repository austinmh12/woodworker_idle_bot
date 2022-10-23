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
			BPUnlock::BirdHouse => write!(f, "Bird House blueprint"),
			BPUnlock::Shelf => write!(f, "Shelf blueprint"),
			BPUnlock::SideTable => write!(f, "Side Table blueprint"),
			BPUnlock::CoffeeTable => write!(f, "Coffee Table blueprint"),
			BPUnlock::DiningSet => write!(f, "Dining Set blueprint"),
		}
	}
}