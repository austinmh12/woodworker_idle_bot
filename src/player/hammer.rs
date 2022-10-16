use std::fmt::Display;

use bson::Bson;
use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	}, 
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Hammer {
	None = 0,
	Stone = 1,
	Iron = 2,
	Steel = 3,
	Mithril = 4,
	Adamant = 5,
	Rune = 6
}

impl Display for Hammer {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			Hammer::None => write!(f, "None"),
			Hammer::Stone => write!(f, "Stone"),
			Hammer::Iron => write!(f, "Iron"),
			Hammer::Steel => write!(f, "Steel"),
			Hammer::Mithril => write!(f, "Mithril"),
			Hammer::Adamant => write!(f, "Adamant"),
			Hammer::Rune => write!(f, "Rune"),
		}
	}
}

impl From<Bson> for Hammer {
	fn from(a: Bson) -> Self {
		match a.as_str().unwrap_or("None") {
			"None" => Hammer::None,
			"Stone" => Hammer::Stone,
			"Iron" => Hammer::Iron,
			"Steel" => Hammer::Steel,
			"Mithril" => Hammer::Mithril,
			"Adamant" => Hammer::Adamant,
			"Rune" => Hammer::Rune,
			_ => Hammer::None
		}
	}
}

impl From<Hammer> for Bson {
	fn from(a: Hammer) -> Self {
		match a {
			Hammer::None => Bson::String("None".to_string()),
			Hammer::Stone => Bson::String("Stone".to_string()),
			Hammer::Iron => Bson::String("Iron".to_string()),
			Hammer::Steel => Bson::String("Steel".to_string()),
			Hammer::Mithril => Bson::String("Mithril".to_string()),
			Hammer::Adamant => Bson::String("Adamant".to_string()),
			Hammer::Rune => Bson::String("Rune".to_string()),
		}
	}
}