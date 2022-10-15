use bson::Bson;
use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	}, 
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Axe {
	Stone = 1,
	Iron = 2,
	Steel = 3,
	Mithril = 4,
	Adamant = 5,
	Rune = 6
}

impl From<Bson> for Axe {
	fn from(a: Bson) -> Self {
		match a.as_str().unwrap_or("Stone") {
			"Stone" => Axe::Stone,
			"Iron" => Axe::Iron,
			"Steel" => Axe::Steel,
			"Mithril" => Axe::Mithril,
			"Adamant" => Axe::Adamant,
			"Rune" => Axe::Rune,
			_ => Axe::Stone
		}
	}
}

impl From<Axe> for Bson {
	fn from(a: Axe) -> Self {
		match a {
			Axe::Stone => Bson::String("Stone".to_string()),
			Axe::Iron => Bson::String("Iron".to_string()),
			Axe::Steel => Bson::String("Steel".to_string()),
			Axe::Mithril => Bson::String("Mithril".to_string()),
			Axe::Adamant => Bson::String("Adamant".to_string()),
			Axe::Rune => Bson::String("Rune".to_string()),
		}
	}
}