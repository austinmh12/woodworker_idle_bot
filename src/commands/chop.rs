use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

pub fn run(player_id: u64, options: &[CommandDataOption]) -> String {
	let option = options
		.get(0)
		.expect("Expected a string")
		.resolved
		.as_ref()
		.expect("Expected str");

	println!("{}", player_id);
	if let CommandDataOptionValue::String(tree) = option {
		match tree.as_str() {
			"pine" => "You chopped a pine".to_string(),
			"oak" => "You must have a iron axe!".to_string(),
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
	})
}