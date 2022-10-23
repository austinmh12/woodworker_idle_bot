use std::fmt::Display;

use bson::Bson;
use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	}, 
};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum Kiln {
	None = 0,
	SteelBucket = 1,
	Firebrick = 2,
	Hobby = 3,
	LabGrade = 4,
	Industrial = 5,
	WorldWide = 6
}

impl Display for Kiln {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			Kiln::None => write!(f, "None"),
			Kiln::SteelBucket => write!(f, "Steel Bucket"),
			Kiln::Firebrick => write!(f, "Firebrick"),
			Kiln::Hobby => write!(f, "Hobby"),
			Kiln::LabGrade => write!(f, "Lab Grade"),
			Kiln::Industrial => write!(f, "Industrial"),
			Kiln::WorldWide => write!(f, "World Wide"),
		}
	}
}

impl From<Bson> for Kiln {
	fn from(a: Bson) -> Self {
		match a.as_str().unwrap_or("None") {
			"None" => Kiln::None,
			"SteelBucket" => Kiln::SteelBucket,
			"Firebrick" => Kiln::Firebrick,
			"Hobby" => Kiln::Hobby,
			"LabGrade" => Kiln::LabGrade,
			"Industrial" => Kiln::Industrial,
			"WorldWide" => Kiln::WorldWide,
			_ => Kiln::None
		}
	}
}

impl From<Kiln> for Bson {
	fn from(a: Kiln) -> Self {
		match a {
			Kiln::None => Bson::String("None".to_string()),
			Kiln::SteelBucket => Bson::String("SteelBucket".to_string()),
			Kiln::Firebrick => Bson::String("Firebrick".to_string()),
			Kiln::Hobby => Bson::String("Hobby".to_string()),
			Kiln::LabGrade => Bson::String("LabGrade".to_string()),
			Kiln::Industrial => Bson::String("Industrial".to_string()),
			Kiln::WorldWide => Bson::String("WorldWide".to_string()),
		}
	}
}