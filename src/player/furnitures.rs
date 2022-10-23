use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
	}, 
};

use crate::utils::ToDoc;
use crate::player::{BPUnlock, Tree};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Blueprints {
	pub pine: FurnitureUnlocks,
	pub oak: FurnitureUnlocks,
	pub maple: FurnitureUnlocks,
	pub walnut: FurnitureUnlocks,
	pub cherry: FurnitureUnlocks,
	pub purpleheart: FurnitureUnlocks,
}

impl Blueprints {
	pub fn next_unlock(&self) -> Option<Tree> {
		if self.pine.next_unlock() != BPUnlock::None {
			Some(Tree::Pine(self.pine.next_unlock()))
		} else if self.oak.next_unlock() != BPUnlock::None {
			Some(Tree::Oak(self.oak.next_unlock()))
		} else if self.maple.next_unlock() != BPUnlock::None {
			Some(Tree::Maple(self.maple.next_unlock()))
		} else if self.walnut.next_unlock() != BPUnlock::None {
			Some(Tree::Walnut(self.walnut.next_unlock()))
		} else if self.cherry.next_unlock() != BPUnlock::None {
			Some(Tree::Cherry(self.cherry.next_unlock()))
		} else if self.purpleheart.next_unlock() != BPUnlock::None {
			Some(Tree::PurpleHeart(self.purpleheart.next_unlock()))
		} else {
			None
		}
	}

	pub fn unlock(&mut self, tree: Tree) {
		match tree {
			Tree::Pine(b) => match b {
				BPUnlock::BirdHouse => self.pine.birdhouse = true,
				BPUnlock::Shelf => self.pine.shelf = true,
				BPUnlock::SideTable => self.pine.side_table = true,
				BPUnlock::CoffeeTable => self.pine.coffee_table = true,
				BPUnlock::DiningSet => self.pine.dining_set = true,
				_ => ()
			},
			Tree::Oak(b) => match b {
				BPUnlock::BirdHouse => self.oak.birdhouse = true,
				BPUnlock::Shelf => self.oak.shelf = true,
				BPUnlock::SideTable => self.oak.side_table = true,
				BPUnlock::CoffeeTable => self.oak.coffee_table = true,
				BPUnlock::DiningSet => self.oak.dining_set = true,
				_ => ()
			},
			Tree::Maple(b) => match b {
				BPUnlock::BirdHouse => self.maple.birdhouse = true,
				BPUnlock::Shelf => self.maple.shelf = true,
				BPUnlock::SideTable => self.maple.side_table = true,
				BPUnlock::CoffeeTable => self.maple.coffee_table = true,
				BPUnlock::DiningSet => self.maple.dining_set = true,
				_ => ()
			},
			Tree::Walnut(b) => match b {
				BPUnlock::BirdHouse => self.walnut.birdhouse = true,
				BPUnlock::Shelf => self.walnut.shelf = true,
				BPUnlock::SideTable => self.walnut.side_table = true,
				BPUnlock::CoffeeTable => self.walnut.coffee_table = true,
				BPUnlock::DiningSet => self.walnut.dining_set = true,
				_ => ()
			},
			Tree::Cherry(b) => match b {
				BPUnlock::BirdHouse => self.cherry.birdhouse = true,
				BPUnlock::Shelf => self.cherry.shelf = true,
				BPUnlock::SideTable => self.cherry.side_table = true,
				BPUnlock::CoffeeTable => self.cherry.coffee_table = true,
				BPUnlock::DiningSet => self.cherry.dining_set = true,
				_ => ()
			},
			Tree::PurpleHeart(b) => match b {
				BPUnlock::BirdHouse => self.purpleheart.birdhouse = true,
				BPUnlock::Shelf => self.purpleheart.shelf = true,
				BPUnlock::SideTable => self.purpleheart.side_table = true,
				BPUnlock::CoffeeTable => self.purpleheart.coffee_table = true,
				BPUnlock::DiningSet => self.purpleheart.dining_set = true,
				_ => ()
			},
		}
	}
}

impl Default for Blueprints {
	fn default() -> Self {
		Self {
			pine: FurnitureUnlocks { birdhouse: true, ..Default::default() },
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
	pub birdhouse: i64,
	pub shelf: i64,
	pub side_table: i64,
	pub coffee_table: i64,
	pub dining_set: i64,
}

impl Default for FurnitureItems {
	fn default() -> Self {
		Self {
			birdhouse: 0,
			shelf: 0,
			side_table: 0,
			coffee_table: 0,
			dining_set: 0,
		}
	}
}

impl ToDoc for FurnitureItems {
	fn to_doc(&self) -> Document {
		doc! {
			"birdhouse": &self.birdhouse,
			"shelf": &self.shelf,
			"side_table": &self.side_table,
			"coffee_table": &self.coffee_table,
			"dining_set": &self.dining_set,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FurnitureUnlocks {
	pub birdhouse: bool,
	pub shelf: bool,
	pub side_table: bool,
	pub coffee_table: bool,
	pub dining_set: bool,
}

impl Default for FurnitureUnlocks {
	fn default() -> Self {
		Self {
			birdhouse: false,
			shelf: false,
			side_table: false,
			coffee_table: false,
			dining_set: false,
		}
	}
}

impl FurnitureUnlocks {
	fn to_vec(&self) -> Vec<bool> {
		vec![self.birdhouse, self.shelf, self.side_table, self.coffee_table, self.dining_set]
	}

	pub fn latest_unlock(&self) -> BPUnlock {
		let v = self
			.to_vec()
			.iter()
			.filter(|f| **f)
			.map(|f| f.to_owned())
			.collect::<Vec<bool>>();

		match v.len() {
			0usize => BPUnlock::None,
			1usize => BPUnlock::BirdHouse,
			2usize => BPUnlock::Shelf,
			3usize => BPUnlock::SideTable,
			4usize => BPUnlock::CoffeeTable,
			_ => BPUnlock::DiningSet
		}
	}

	pub fn next_unlock(&self) -> BPUnlock {
		match self.latest_unlock() {
			BPUnlock::None => BPUnlock::BirdHouse,
			BPUnlock::BirdHouse => BPUnlock::Shelf,
			BPUnlock::Shelf => BPUnlock::SideTable,
			BPUnlock::SideTable => BPUnlock::CoffeeTable,
			BPUnlock::CoffeeTable => BPUnlock::DiningSet,
			BPUnlock::DiningSet => BPUnlock::None
		}
	}
}

impl ToDoc for FurnitureUnlocks {
	fn to_doc(&self) -> Document {
		doc! {
			"birdhouse": &self.birdhouse,
			"shelf": &self.shelf,
			"side_table": &self.side_table,
			"coffee_table": &self.coffee_table,
			"dining_set": &self.dining_set,
		}
	}
}