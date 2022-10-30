use std::{fmt::Display, str::FromStr};
use bson::Bson;
use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	}, 
};

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Furniture {
	BirdHouse,
	Shelf,
	SideTable,
	CoffeeTable,
	DiningSet,
}

impl Display for Furniture {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Furniture::BirdHouse => write!(f, "Bird House"),
			Furniture::Shelf => write!(f, "Shelf"),
			Furniture::SideTable => write!(f, "Side Table"),
			Furniture::CoffeeTable => write!(f, "Coffee Table"),
			Furniture::DiningSet => write!(f, "Dining Set"),
		}
	}
}

impl FromStr for Furniture {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"birdhouse" => Ok(Furniture::BirdHouse),
			"shelf" => Ok(Furniture::Shelf),
			"sidetable" => Ok(Furniture::SideTable),
			"coffeetable" => Ok(Furniture::CoffeeTable),
			"diningset" => Ok(Furniture::DiningSet),
			_ => Err("No such Furniture".to_string())
		}
	}
}

impl From<Bson> for Furniture {
	fn from(a: Bson) -> Self {
		match a.as_str().unwrap_or("BirdHouse") {
			"BirdHouse" => Furniture::BirdHouse,
			"Shelf" => Furniture::Shelf,
			"SideTable" => Furniture::SideTable,
			"CoffeeTable" => Furniture::CoffeeTable,
			"DiningSet" => Furniture::DiningSet,
			_ => Furniture::BirdHouse
		}
	}
}

impl From<Furniture> for Bson {
	fn from(t: Furniture) -> Self {
		match t {
			Furniture::BirdHouse => Bson::String("BirdHouse".to_string()),
			Furniture::Shelf => Bson::String("Shelf".to_string()),
			Furniture::SideTable => Bson::String("SideTable".to_string()),
			Furniture::CoffeeTable => Bson::String("CoffeeTable".to_string()),
			Furniture::DiningSet => Bson::String("DiningSet".to_string()),
		}
	}
}