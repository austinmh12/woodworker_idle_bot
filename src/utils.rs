use std::error::Error;
use mongodb::{
	Client,
	options::{
		ClientOptions,
	},
	bson::{
		Document,
	}, 
};
use serenity::utils::Colour;

use crate::player::{Player, Color};

pub trait ToDoc {
	fn to_doc(&self) -> Document;
}

pub fn get_tree_time(player: &Player, tree: &str, actions: i64) -> i64 {
	let base_time = match tree {
		"pine" => 10.0,
		"oak" => 15.0,
		"maple" => 25.0,
		"walnut" => 35.0,
		"cherry" => 50.0,
		"purpleheart" => 80.0,
		_ => 10.0
	};
	let upgrade_mult = 1.0 + (player.upgrades.sharper_axes as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.sharper_axes as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult) as i64 * actions
}

pub fn get_dry_time(player: &Player, tree: &str, actions: i64) -> i64 {
	let base_time = match tree {
		"pine" => 10.0,
		"oak" => 15.0,
		"maple" => 25.0,
		"walnut" => 35.0,
		"cherry" => 50.0,
		"purpleheart" => 80.0,
		_ => 10.0
	};
	let upgrade_mult = 1.0 + (player.upgrades.hotter_kilns as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.hotter_kilns as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult) as i64 * actions
}

pub fn get_build_time(player: &Player, tree: &str, furniture: &str, actions: i64) -> i64 {
	let base_time = match tree {
		"pine" => 10.0,
		"oak" => 15.0,
		"maple" => 25.0,
		"walnut" => 35.0,
		"cherry" => 50.0,
		"purpleheart" => 80.0,
		_ => 10.0
	};
	let furniture_mult = match furniture {
		"birdhouse" => 1.0,
		"shelf" => 1.5,
		"side table" => 2.0,
		"coffee table" => 2.5,
		"dining set" => 3.0,
		_ => 1.0
	};
	let base_time = base_time * furniture_mult;
	let upgrade_mult = 1.0 + (player.upgrades.fast_drying_glue as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.fast_drying_glue as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult) as i64 * actions
}

pub async fn get_client() -> Result<Client, Box<dyn Error>> {
	let mon_client_uri = dotenv::var("MONGODB_URI").expect("No mongodb uri");
	let options = ClientOptions::parse(&mon_client_uri).await?;
	let client = Client::with_options(options)?;
	
	Ok(client)
}

pub fn default_colour() -> Colour {
	let color = Color::default();

	Colour::from_rgb(color.red, color.green, color.blue)
}

pub fn get_max_buyable(player: &Player, base: f64, exp: f64, owned: i64) -> i64 {
	let top = player.cash * (exp - 1.0);
	let bot = base * exp.powi(owned as i32);
	let inner = (top / bot) + 1.0;
	let log = inner.log(exp);
	
	log.floor() as i64
}

pub fn get_price(amount: i64, base: f64, exp: f64, owned: i64) -> f64 {
	let top1 = exp.powi(owned as i32);
	let top2 = exp.powi(amount as i32) - 1.0;
	let bottom = exp - 1.0;
	
	base * ((top1 * top2) / bottom)
}

pub fn get_max_buyable_amount_and_price(player: &Player, amount: i64, base: f64, exp: f64, owned: i64) -> (i64, f64) {
	let max_amount = get_max_buyable(&player, base, exp, owned);
	let amounts = vec![amount, max_amount];
	let amount = amounts.iter().min().unwrap().to_owned();
	
	(amount, get_price(amount, base, exp, owned))
}