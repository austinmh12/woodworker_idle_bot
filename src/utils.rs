use std::{error::Error};
use mongodb::{
	Client,
	options::{
		ClientOptions,
	},
	bson::{
		Document,
	}, 
};
use serenity::{
	utils::Colour,
	builder::{CreateEmbed},
	prelude::Context,
	model::prelude::{
		ReactionType,
		interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType}
	}
};
use std::time::Duration as StdDuration;

use crate::player::{Player, Color};

pub trait ToDoc {
	fn to_doc(&self) -> Document;
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

#[derive(Clone)]
pub struct PaginatedEmbed {
	pub pages: Vec<CreateEmbed>,
}

impl PaginatedEmbed {
	pub fn new(pages: Vec<CreateEmbed>) -> Self {
		Self {
			pages
		}
	}

	pub async fn scroll_through(self, ctx: &Context, command: ApplicationCommandInteraction) {
		let left_arrow = ReactionType::try_from("⬅️").expect("No left arrow");
		let right_arrow = ReactionType::try_from("➡️").expect("No right arrow");
		let mut idx: i16 = 0;
		command
			.create_interaction_response(&ctx.http, |resp| {
				resp
					.kind(InteractionResponseType::ChannelMessageWithSource)
					.interaction_response_data(|m| {
						let mut cur_embed = self.pages[idx as usize].clone();
						if self.pages.len() > 1 {
							cur_embed.footer(|f| f.text(format!("{}/{}", idx + 1, self.pages.len())));
						}
						m.set_embed(cur_embed);

						m
					})
			}).await.unwrap();

		if self.pages.len() == 1 {
			tokio::time::sleep(StdDuration::from_secs(30)).await;
		} else {
			let mut message = command
				.get_interaction_response(&ctx.http)
				.await
				.unwrap();
			message.react(&ctx.http, left_arrow).await.unwrap();
			message.react(&ctx.http, right_arrow).await.unwrap();

			loop {
				if let Some(reaction) = &message
					.await_reaction(&ctx)
					.timeout(StdDuration::from_secs(60))
					.author_id(command.user.id)
					.removed(true)
					.await
				{
					let emoji = &reaction.as_inner_ref().emoji;
					match emoji.as_data().as_str() {
						"⬅️" => idx = (idx - 1).rem_euclid(self.pages.len() as i16),
						"➡️" => idx = (idx + 1) % self.pages.len() as i16,
						_ => {
							println!("{}", &emoji.as_data().as_str());
							continue
						}
					};
				} else {
					message.delete_reactions(&ctx).await.expect("Couldn't remove arrows");
					break;
				}
				message.edit(&ctx, |m| {
					let mut cur_embed = self.pages[idx as usize].clone();
					if self.pages.len() > 1 {
						cur_embed.footer(|f| f.text(format!("{}/{}", idx + 1, self.pages.len())));
					}
					m.set_embed(cur_embed);

					m
				}).await.unwrap();
			}
		}
		// command
		// 	.delete_original_interaction_response(&ctx.http)
		// 	.await
		// 	.unwrap();
	}
}

pub enum Message {
	Content(String),
	Embed(CreateEmbed),
	Pages(PaginatedEmbed),
	SawdustPrestige(u64), // Special messages to trigger prestiges
	// SeedPrestige(u64),
}

impl Message {
	pub fn how() -> Self {
		Message::Content("How did you get here?".to_string())
	}

	pub fn under_construction() -> Self {
		Message::Content("Under construction!".to_string())
	}
}