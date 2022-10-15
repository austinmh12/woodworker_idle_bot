use serde::{Serialize, Deserialize};
use mongodb::{
	bson::{
		doc,
		Document,
	}, 
};

use crate::utils::ToDoc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Color {
	pub red: u8,
	pub green: u8,
	pub blue: u8,
}

impl ToDoc for Color {
	fn to_doc(&self) -> Document {
		doc! {
			"red": *&self.red as i64,
			"green": *&self.green as i64,
			"blue": *&self.blue as i64,
		}
	}
}

impl Default for Color {
	fn default() -> Self {
		Self {
			red: 177,
			green: 126,
			blue: 34
		}
	}
}