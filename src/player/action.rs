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

impl ToDoc for Option<Action> {
	fn to_doc(&self) -> bson::Document {
		match &self {
			Some(a) => a.to_doc(),
			None => doc! {},
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ActionEnum {
	Chopping,
	Drying,
	Building,
}

impl From<Bson> for ActionEnum {
	fn from(a: Bson) -> Self {
		match a.as_str().unwrap_or("Chopping") {
			"Chopping" => ActionEnum::Chopping,
			"Drying" => ActionEnum::Drying,
			"Building" => ActionEnum::Building,
			_ => ActionEnum::Chopping
		}
	}
}

impl From<ActionEnum> for Bson {
	fn from(a: ActionEnum) -> Self {
		match a {
			ActionEnum::Chopping => Bson::String("Chopping".to_string()),
			ActionEnum::Drying => Bson::String("Drying".to_string()),
			ActionEnum::Building => Bson::String("Building".to_string()),
		}
	}
}