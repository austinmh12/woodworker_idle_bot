use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::player::{get_player, Axe};

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> String {
	let option = options
		.get(0)
		.expect("Expected a string")
		.resolved
		.as_ref()
		.expect("Expected str");

	// println!("{}", player_id);
	let mut player = get_player(player_id).await;
	if let CommandDataOptionValue::String(tree) = option {
		match tree.as_str() {
			"pine" => {
				let amt = 1 + player.upgrades.sharper_axes + player.sawdust_upgrades.sharper_axes;
				player.logs.pine += amt;
				player.stats.pine_trees_chopped += amt;
				player.stats.pine_logs_earned += amt;
				player.update().await;
				let s = if amt >= 1 {
					"s"
				} else {
					""
				};
				format!("You chopped **{} pine** log{}!", amt, s)
			},
			"oak" => {
				if player.axe >= Axe::Iron {
					player.logs.oak += 1;
					player.stats.oak_trees_chopped += 1;
					player.stats.oak_logs_earned += 1;
					player.update().await;
					format!("You chopped {} oak log!", 1)
				} else {
					"You need an **Iron** axe to chop oak logs!".to_string()
				}
			},
			_ => "No such tree".to_string()
		}
	} else {
		"No such tree".to_string()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("chop").description("Chop trees for lumber!").create_option(|option| {
		option
			.name("tree")
			.description("The type of tree to chop")
			.kind(CommandOptionType::String)
			.required(true)
			.add_string_choice("Pine", "pine")
			.add_string_choice("Oak", "oak")
			.add_string_choice("Maple", "maple")
			.add_string_choice("Walnut", "walnut")
			.add_string_choice("Cherry", "cherry")
			.add_string_choice("Purpleheart", "purpleheart")
	})
}