use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
	}, 
};

use crate::utils::ToDoc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WoodsInt {
	pub pine: i64,
	pub oak: i64,
	pub maple: i64,
	pub walnut: i64,
	pub cherry: i64,
	pub purpleheart: i64,
}

impl WoodsInt {
	pub fn total(&self) -> i64 {
		self.pine + self.oak + self.maple + self.walnut + self.cherry + self.purpleheart
	}
}

impl Default for WoodsInt {
	fn default() -> Self {
		Self {
			pine: 0,
			oak: 0,
			maple: 0,
			walnut: 0,
			cherry: 0,
			purpleheart: 0,
		}
	}
}

impl ToDoc for WoodsInt {
	fn to_doc(&self) -> Document {
		doc! {
			"pine": &self.pine,
			"oak": &self.oak,
			"maple": &self.maple,
			"walnut": &self.walnut,
			"cherry": &self.cherry,
			"purpleheart": &self.purpleheart,
		}
	}
}

