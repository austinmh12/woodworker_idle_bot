use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
	}, 
};

use crate::utils::ToDoc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blueprints {
	pub pine: FurnitureUnlocks,
	pub oak: FurnitureUnlocks,
	pub maple: FurnitureUnlocks,
	pub walnut: FurnitureUnlocks,
	pub cherry: FurnitureUnlocks,
	pub purpleheart: FurnitureUnlocks,
}

impl Default for Blueprints {
	fn default() -> Self {
		Self {
			pine: FurnitureUnlocks { birdhouse: true },
			oak: FurnitureUnlocks::default(),
			maple: FurnitureUnlocks::default(),
			walnut: FurnitureUnlocks::default(),
			cherry: FurnitureUnlocks::default(),
			purpleheart: FurnitureUnlocks::default(),
		}
	}
}

impl ToDoc for Blueprints {
	fn to_doc(&self) -> Document {
		doc! {
			"pine": &self.pine.to_doc(),
			"oak": &self.oak.to_doc(),
			"maple": &self.maple.to_doc(),
			"walnut": &self.walnut.to_doc(),
			"cherry": &self.cherry.to_doc(),
			"purpleheart": &self.purpleheart.to_doc(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Furniture {
	pub pine: FurnitureItems,
	pub oak: FurnitureItems,
	pub maple: FurnitureItems,
	pub walnut: FurnitureItems,
	pub cherry: FurnitureItems,
	pub purpleheart: FurnitureItems,
}

impl Default for Furniture {
	fn default() -> Self {
		Self {
			pine: FurnitureItems::default(),
			oak: FurnitureItems::default(),
			maple: FurnitureItems::default(),
			walnut: FurnitureItems::default(),
			cherry: FurnitureItems::default(),
			purpleheart: FurnitureItems::default(),
		}
	}
}

impl ToDoc for Furniture {
	fn to_doc(&self) -> Document {
		doc! {
			"pine": &self.pine.to_doc(),
			"oak": &self.oak.to_doc(),
			"maple": &self.maple.to_doc(),
			"walnut": &self.walnut.to_doc(),
			"cherry": &self.cherry.to_doc(),
			"purpleheart": &self.purpleheart.to_doc(),
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FurnitureItems {
	pub birdhouse: i64
}

impl Default for FurnitureItems {
	fn default() -> Self {
		Self {
			birdhouse: 0
		}
	}
}

impl ToDoc for FurnitureItems {
	fn to_doc(&self) -> Document {
		doc! {
			"birdhouse": &self.birdhouse,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FurnitureUnlocks {
	pub birdhouse: bool
}

impl Default for FurnitureUnlocks {
	fn default() -> Self {
		Self {
			birdhouse: false
		}
	}
}

impl ToDoc for FurnitureUnlocks {
	fn to_doc(&self) -> Document {
		doc! {
			"birdhouse": &self.birdhouse,
		}
	}
}