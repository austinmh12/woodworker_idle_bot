use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
	},
};
use chrono::{
	DateTime, 
	Utc,
};

use crate::utils::ToDoc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OfflineTimer {
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub pine_log: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub oak_log: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub maple_log: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub walnut_log: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub cherry_log: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub purpleheart_log: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub pine_lumber: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub oak_lumber: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub maple_lumber: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub walnut_lumber: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub cherry_lumber: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub purpleheart_lumber: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub pine_birdhouse: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub pine_shelf: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub pine_side_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub pine_coffee_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub pine_dining_set: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub oak_birdhouse: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub oak_shelf: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub oak_side_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub oak_coffee_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub oak_dining_set: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub maple_birdhouse: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub maple_shelf: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub maple_side_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub maple_coffee_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub maple_dining_set: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub walnut_birdhouse: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub walnut_shelf: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub walnut_side_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub walnut_coffee_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub walnut_dining_set: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub cherry_birdhouse: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub cherry_shelf: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub cherry_side_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub cherry_coffee_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub cherry_dining_set: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub purpleheart_birdhouse: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub purpleheart_shelf: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub purpleheart_side_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub purpleheart_coffee_table: DateTime<Utc>,
	#[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
	pub purpleheart_dining_set: DateTime<Utc>,
}

impl Default for OfflineTimer {
	fn default() -> Self {
		Self {
			pine_log: Utc::now(),
			oak_log: Utc::now(),
			maple_log: Utc::now(),
			walnut_log: Utc::now(),
			cherry_log: Utc::now(),
			purpleheart_log: Utc::now(),
			pine_lumber: Utc::now(),
			oak_lumber: Utc::now(),
			maple_lumber: Utc::now(),
			walnut_lumber: Utc::now(),
			cherry_lumber: Utc::now(),
			purpleheart_lumber: Utc::now(),
			pine_birdhouse: Utc::now(),
			pine_shelf: Utc::now(),
			pine_side_table: Utc::now(),
			pine_coffee_table: Utc::now(),
			pine_dining_set: Utc::now(),
			oak_birdhouse: Utc::now(),
			oak_shelf: Utc::now(),
			oak_side_table: Utc::now(),
			oak_coffee_table: Utc::now(),
			oak_dining_set: Utc::now(),
			maple_birdhouse: Utc::now(),
			maple_shelf: Utc::now(),
			maple_side_table: Utc::now(),
			maple_coffee_table: Utc::now(),
			maple_dining_set: Utc::now(),
			walnut_birdhouse: Utc::now(),
			walnut_shelf: Utc::now(),
			walnut_side_table: Utc::now(),
			walnut_coffee_table: Utc::now(),
			walnut_dining_set: Utc::now(),
			cherry_birdhouse: Utc::now(),
			cherry_shelf: Utc::now(),
			cherry_side_table: Utc::now(),
			cherry_coffee_table: Utc::now(),
			cherry_dining_set: Utc::now(),
			purpleheart_birdhouse: Utc::now(),
			purpleheart_shelf: Utc::now(),
			purpleheart_side_table: Utc::now(),
			purpleheart_coffee_table: Utc::now(),
			purpleheart_dining_set: Utc::now(),
		}
	}
}

impl ToDoc for OfflineTimer {
	fn to_doc(&self) -> bson::Document {
		doc! {
			"pine_log": self.pine_log,
			"oak_log": self.oak_log,
			"maple_log": self.maple_log,
			"walnut_log": self.walnut_log,
			"cherry_log": self.cherry_log,
			"purpleheart_log": self.purpleheart_log,
			"pine_lumber": self.pine_lumber,
			"oak_lumber": self.oak_lumber,
			"maple_lumber": self.maple_lumber,
			"walnut_lumber": self.walnut_lumber,
			"cherry_lumber": self.cherry_lumber,
			"purpleheart_lumber": self.purpleheart_lumber,
			"pine_birdhouse": self.pine_birdhouse,
			"pine_shelf": self.pine_shelf,
			"pine_side_table": self.pine_side_table,
			"pine_coffee_table": self.pine_coffee_table,
			"pine_dining_set": self.pine_dining_set,
			"oak_birdhouse": self.oak_birdhouse,
			"oak_shelf": self.oak_shelf,
			"oak_side_table": self.oak_side_table,
			"oak_coffee_table": self.oak_coffee_table,
			"oak_dining_set": self.oak_dining_set,
			"maple_birdhouse": self.maple_birdhouse,
			"maple_shelf": self.maple_shelf,
			"maple_side_table": self.maple_side_table,
			"maple_coffee_table": self.maple_coffee_table,
			"maple_dining_set": self.maple_dining_set,
			"walnut_birdhouse": self.walnut_birdhouse,
			"walnut_shelf": self.walnut_shelf,
			"walnut_side_table": self.walnut_side_table,
			"walnut_coffee_table": self.walnut_coffee_table,
			"walnut_dining_set": self.walnut_dining_set,
			"cherry_birdhouse": self.cherry_birdhouse,
			"cherry_shelf": self.cherry_shelf,
			"cherry_side_table": self.cherry_side_table,
			"cherry_coffee_table": self.cherry_coffee_table,
			"cherry_dining_set": self.cherry_dining_set,
			"purpleheart_birdhouse": self.purpleheart_birdhouse,
			"purpleheart_shelf": self.purpleheart_shelf,
			"purpleheart_side_table": self.purpleheart_side_table,
			"purpleheart_coffee_table": self.purpleheart_coffee_table,
			"purpleheart_dining_set": self.purpleheart_dining_set,
		}
	}
}