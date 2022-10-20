use std::fmt::Display;

use bson::Bson;
use chrono::{
	DateTime, 
	Utc,
	Duration,
};
use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	}, 
};

use crate::utils::ToDoc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Action {
	pub action: ActionEnum,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub start: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub end: DateTime<Utc>,
	pub tree: String,
	pub furniture: Option<String>,
}

impl Action {
	pub fn chopping(duration: i64, tree: &str) -> Self {
		let start = Utc::now();
		let end = start + Duration::seconds(duration);
		
		Self {
			action: ActionEnum::Chopping,
			start,
			end,
			tree: tree.into(),
			furniture: None,
		}
	}

	pub fn drying(duration: i64, tree: &str) -> Self {
		let start = Utc::now();
		let end = start + Duration::seconds(duration);
		
		Self {
			action: ActionEnum::Drying,
			start,
			end,
			tree: tree.into(),
			furniture: None,
		}
	}

	pub fn building(duration: i64, tree: &str, furniture: &str) -> Self {
		let start = Utc::now();
		let end = start + Duration::seconds(duration);
		
		Self {
			action: ActionEnum::Building,
			start,
			end,
			tree: tree.into(),
			furniture: Some(furniture.into()),
		}
	}

	pub fn none() -> Self {
		Self {
			action: ActionEnum::None,
			start: Utc::now(),
			end: Utc::now(),
			tree: "".into(),
			furniture: None,
		}
	}

	pub fn time_to_complete(&self) -> i64 {
		// Returns the total number of seconds until the endtime
		&self.end.timestamp() - Utc::now().timestamp()
	}
}

impl ToDoc for Action {
	fn to_doc(&self) -> bson::Document {
		doc! {
			"action": &self.action,
			"start": &self.start,
			"end": &self.end,
			"tree": &self.tree,
			"furniture": &self.furniture,
		}
	}
}

impl Display for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self.action {
			ActionEnum::None => write!(f, "None!"),
			ActionEnum::Building => {
				let alt = "".to_string();
				let furniture = &self.furniture.as_ref().unwrap_or(&alt);
				write!(f, "{} {} {}, **{}s** left", &self.action, &self.tree, furniture, &self.time_to_complete())
			},
			_ => write!(f, "{} {}, **{}s** left", &self.action, &self.tree, &self.time_to_complete())
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum ActionEnum {
	Chopping,
	Drying,
	Building,
	None,
}

impl Display for ActionEnum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match &self {
			ActionEnum::Chopping => write!(f, "Chopping"),
			ActionEnum::Drying => write!(f, "Drying"),
			ActionEnum::Building => write!(f, "Building"),
			ActionEnum::None => write!(f, "None"),
		}
	}
}

impl From<Bson> for ActionEnum {
	fn from(a: Bson) -> Self {
		match a.as_str().unwrap_or("None") {
			"Chopping" => ActionEnum::Chopping,
			"Drying" => ActionEnum::Drying,
			"Building" => ActionEnum::Building,
			_ => ActionEnum::None
		}
	}
}

impl From<ActionEnum> for Bson {
	fn from(a: ActionEnum) -> Self {
		match a {
			ActionEnum::Chopping => Bson::String("Chopping".to_string()),
			ActionEnum::Drying => Bson::String("Drying".to_string()),
			ActionEnum::Building => Bson::String("Building".to_string()),
			ActionEnum::None => Bson::String("None".to_string()),
		}
	}
}