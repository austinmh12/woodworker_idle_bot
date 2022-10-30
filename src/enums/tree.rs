use std::{fmt::Display, str::FromStr};
use bson::Bson;
use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	}, 
};

#[derive(Debug, Clone, PartialEq, Copy, Serialize, Deserialize)]
pub enum Tree {
	Pine,
	Oak,
	Maple,
	Walnut,
	Cherry,
	PurpleHeart,
}

impl Display for Tree {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Tree::Pine => write!(f, "Pine"),
			Tree::Oak => write!(f, "Oak"),
			Tree::Maple => write!(f, "Maple"),
			Tree::Walnut => write!(f, "Walnut"),
			Tree::Cherry => write!(f, "Cherry"),
			Tree::PurpleHeart => write!(f, "Purple Heart"),
		}
	}
}

impl FromStr for Tree {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"pine" => Ok(Tree::Pine),
			"oak" => Ok(Tree::Oak),
			"maple" => Ok(Tree::Maple),
			"walnut" => Ok(Tree::Walnut),
			"cherry" => Ok(Tree::Cherry),
			"purpleheart" => Ok(Tree::PurpleHeart),
			_ => Err("No such tree".to_string())
		}
	}
}

impl From<Bson> for Tree {
	fn from(a: Bson) -> Self {
		match a.as_str().unwrap_or("Pine") {
			"Pine" => Tree::Pine,
			"Oak" => Tree::Oak,
			"Maple" => Tree::Maple,
			"Walnut" => Tree::Walnut,
			"Cherry" => Tree::Cherry,
			"PurpleHeart" => Tree::PurpleHeart,
			_ => Tree::Pine
		}
	}
}

impl From<Tree> for Bson {
	fn from(t: Tree) -> Self {
		match t {
			Tree::Pine => Bson::String("Pine".to_string()),
			Tree::Oak => Bson::String("Oak".to_string()),
			Tree::Maple => Bson::String("Maple".to_string()),
			Tree::Walnut => Bson::String("Walnut".to_string()),
			Tree::Cherry => Bson::String("Cherry".to_string()),
			Tree::PurpleHeart => Bson::String("PurpleHeart".to_string()),
		}
	}
}