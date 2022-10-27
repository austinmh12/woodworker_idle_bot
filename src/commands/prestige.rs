use std::time::Duration as StdDuration;
use serenity::builder::{CreateApplicationCommand, CreateActionRow};
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::component::ButtonStyle;
use serenity::model::prelude::interaction::{InteractionResponseType, application_command::{
	CommandDataOption,
	ApplicationCommandInteraction
}};
use serenity::prelude::Context;

use crate::player::get_player;
use crate::utils::Message;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> Message {
	let option = &options
		.get(0)
		.expect("Expected a Subcommand");
	
	let _player = get_player(player_id).await; // Just in case this is the first command ran
	match option.name.as_str() {
		"sawdust" => Message::SawdustPrestige(player_id),
		// "seed" => Message::SeedPrestige(player_id),
		_ => Message::how()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("prestige").description("reset your progress for boosts!")
		.create_option(|option| {
			option
				.name("sawdust")
				.description("sawdust prestige")
				.kind(CommandOptionType::SubCommand)
		})
		// .create_option(|option| {
		// 	option
		// 		.name("seed")
		// 		.description("seed prestige")
		// 		.kind(CommandOptionType::SubCommand)
		// })
}

pub async fn sawdust_prestige_player(ctx: &Context, command: &ApplicationCommandInteraction, player_id: u64) {
	let mut player = get_player(player_id).await;
	let sawdust_earned = player.sawdust_prestige.calculate_sawdust();

	command
		.create_interaction_response(&ctx.http, |response| {
			response
				.kind(InteractionResponseType::ChannelMessageWithSource)
				.interaction_response_data(|m| {
					m
						.content(format!("Do you want to sawdust prestige?\nIf you sawdust prestige now, you'll get **{}** sawdust\nSawdust gives a permanent 1% boost to cash earned", sawdust_earned))
						.components(|c| {
							c
								.create_action_row(|r| {
									r
										.create_button(|b| {
											b
												.custom_id("sawdust_prestige_confirm")
												.style(ButtonStyle::Success)
												.label("Confirm")
										})
										.create_button(|b| {
											b
												.custom_id("sawdust_prestige_cancel")
												.style(ButtonStyle::Danger)
												.label("Cancel")
										})
								})
						})
				})
		}).await.unwrap();
		let mut message = command.get_interaction_response(&ctx.http).await.unwrap();
		if let Some(response) = &message
			.await_component_interaction(&ctx)
			.timeout(StdDuration::from_secs(20))
			.author_id(command.user.id)
			.await
		{
			let button = &response.as_ref();
			match button.data.custom_id.as_str() {
				"sawdust_prestige_confirm" => {
					message
						.edit(&ctx.http, |e| {
							e
								.components(|c| {
									let mut r = CreateActionRow::default();
									r										
										.create_button(|b| {
											b
												.custom_id("sawdust_prestige_confirm")
												.style(ButtonStyle::Success)
												.label("Confirm")
												.disabled(true)
										})
										.create_button(|b| {
											b
												.custom_id("sawdust_prestige_cancel")
												.style(ButtonStyle::Danger)
												.label("Cancel")
												.disabled(true)
										});
									
									c.set_action_row(r)
								})
						}).await.unwrap();
					player.perform_sawdust_prestige(sawdust_earned);
					player.update().await;
					button
						.create_interaction_response(&ctx.http, |response| {
							response
								.kind(InteractionResponseType::ChannelMessageWithSource)
								.interaction_response_data(|m| {
									m
									.content(format!("Sawdust prestiged for **{}** sawdust, you now have **{}** sawdust", sawdust_earned, player.sawdust))
								})
						}).await.unwrap();
				},
				"sawdust_prestige_cancel" => {
					message
						.edit(&ctx.http, |e| {
							e
								.components(|c| {
									let mut r = CreateActionRow::default();
									r										
										.create_button(|b| {
											b
												.custom_id("sawdust_prestige_confirm")
												.style(ButtonStyle::Success)
												.label("Confirm")
												.disabled(true)
										})
										.create_button(|b| {
											b
												.custom_id("sawdust_prestige_cancel")
												.style(ButtonStyle::Danger)
												.label("Cancel")
												.disabled(true)
										});
									
									c.set_action_row(r)
								})
						}).await.unwrap();
					button
						.create_interaction_response(&ctx.http, |response| {
							response
								.kind(InteractionResponseType::ChannelMessageWithSource)
								.interaction_response_data(|m| m.content("Canceled!"))
						}).await.unwrap();
				},
				_ => (),
			}

			tokio::time::sleep(StdDuration::from_secs(15)).await;
			// button.delete_original_interaction_response(&ctx.http).await.unwrap();
			command.delete_original_interaction_response(&ctx.http).await.unwrap();
		}
}

// pub async fn seed_prestige_player(ctx: &Context, command: &ApplicationCommandInteraction, player_id: u64) {

// }