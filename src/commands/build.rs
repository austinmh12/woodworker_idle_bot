use std::str::FromStr;

use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};

use crate::enums::{Tree, Furniture};
use crate::player::{get_player, Hammer, Player, Action, ActionEnum};
use crate::utils::Message;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> Message {
	let tree_input = &options
		.get(0)
		.expect("Expected a Subcommand");
	let furniture_input = &tree_input
		.options
		.get(0)
		.expect("Expected subcommand");
	let mut actions = if &furniture_input.options.len() == &0usize {
		1
	} else {
		match furniture_input.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
			CommandDataOptionValue::Integer(i) => i.to_owned(),
			_ => 1
		}
	};
	let mut player = get_player(player_id).await;
	if actions > 5 + player.sawdust_upgrades.endurance_training {
		actions = 5 + player.sawdust_upgrades.endurance_training;
	}
	let tree = Tree::from_str(&tree_input.name).expect("No such tree.");
	let furniture = Furniture::from_str(&furniture_input.name).expect("No such furniture");

	match player.current_action.action {
		ActionEnum::None => (),
		_ => {
			if player.queued_actions.len() < (player.sawdust_upgrades.multitasking + 2) as usize {
				()
			} else {
				return Message::Content(format!("You're busy for another **{}s**!", player.current_action.time_to_complete()));
			}
		},
	}
	if player.hammer == Hammer::None {
		return Message::Content("You don't have a hammer! Buy one from the store!".to_string());
	}

	match tree {
		// Don't need to check hammer::none since we do it above.
		Tree::Pine => match furniture {
			Furniture::BirdHouse => {
				if !player.blueprints.pine.birdhouse {
					return Message::Content(format!("You don't know how to make a pine bird house!"));
				}
				if player.lumber.pine + player.queued_lumber(tree) < 1 * actions {
					return Message::Content(format!("You don't have enough pine lumber, you need **{}** more", (1 * actions) - player.lumber.pine + player.queued_lumber(tree)));
				}

				player.lumber.pine -= 1 * actions;
			},
			Furniture::Shelf => {
				if !player.blueprints.pine.shelf {
					return Message::Content(format!("You don't know how to make a pine shelf!"));
				}
				if player.lumber.pine + player.queued_lumber(tree) < 2 * actions {
					return Message::Content(format!("You don't have enough pine lumber, you need **{}** more", (2 * actions) - player.lumber.pine + player.queued_lumber(tree)));
				}

				player.lumber.pine -= 2 * actions;
			},
			Furniture::SideTable => {
				if !player.blueprints.pine.side_table {
					return Message::Content(format!("You don't know how to make a pine side table!"));
				}
				if player.lumber.pine + player.queued_lumber(tree) < 3 * actions {
					return Message::Content(format!("You don't have enough pine lumber, you need **{}** more", (3 * actions) - player.lumber.pine + player.queued_lumber(tree)));
				}

				player.lumber.pine -= 3 * actions;
			},
			Furniture::CoffeeTable => {
				if !player.blueprints.pine.coffee_table {
					return Message::Content(format!("You don't know how to make a pine coffee table!"));
				}
				if player.lumber.pine + player.queued_lumber(tree) < 4 * actions {
					return Message::Content(format!("You don't have enough pine lumber, you need **{}** more", (4 * actions) - player.lumber.pine + player.queued_lumber(tree)));
				}

				player.lumber.pine -= 4 * actions;
			},
			Furniture::DiningSet => {
				if !player.blueprints.pine.dining_set {
					return Message::Content(format!("You don't know how to make a pine dining set!"));
				}
				if player.lumber.pine + player.queued_lumber(tree) < 5 * actions {
					return Message::Content(format!("You don't have enough pine lumber, you need **{}** more", (5 * actions) - player.lumber.pine + player.queued_lumber(tree)));
				}

				player.lumber.pine -= 5 * actions;
			},
		},
		Tree::Oak => {
			if player.hammer < Hammer::Iron {
				return Message::Content("You need a **Iron** hammer to build with oak lumber!".to_string());
			}
			match furniture {
				Furniture::BirdHouse => {
					if !player.blueprints.oak.birdhouse {
						return Message::Content(format!("You don't know how to make a oak bird house!"));
					}
					if player.lumber.oak + player.queued_lumber(tree) < 1 * actions {
						return Message::Content(format!("You don't have enough oak lumber, you need **{}** more", (1 * actions) - player.lumber.oak + player.queued_lumber(tree)));
					}

				player.lumber.oak -= 1 * actions;
				},
				Furniture::Shelf => {
					if !player.blueprints.oak.shelf {
						return Message::Content(format!("You don't know how to make a oak shelf!"));
					}
					if player.lumber.oak + player.queued_lumber(tree) < 2 * actions {
						return Message::Content(format!("You don't have enough oak lumber, you need **{}** more", (2 * actions) - player.lumber.oak + player.queued_lumber(tree)));
					}

				player.lumber.oak -= 2 * actions;
				},
				Furniture::SideTable => {
					if !player.blueprints.oak.side_table {
						return Message::Content(format!("You don't know how to make a oak side table!"));
					}
					if player.lumber.oak + player.queued_lumber(tree) < 3 * actions {
						return Message::Content(format!("You don't have enough oak lumber, you need **{}** more", (3 * actions) - player.lumber.oak + player.queued_lumber(tree)));
					}

				player.lumber.oak -= 3 * actions;
				},
				Furniture::CoffeeTable => {
					if !player.blueprints.oak.coffee_table {
						return Message::Content(format!("You don't know how to make a oak coffee table!"));
					}
					if player.lumber.oak + player.queued_lumber(tree) < 4 * actions {
						return Message::Content(format!("You don't have enough oak lumber, you need **{}** more", (4 * actions) - player.lumber.oak + player.queued_lumber(tree)));
					}

				player.lumber.oak -= 4 * actions;
				},
				Furniture::DiningSet => {
					if !player.blueprints.oak.dining_set {
						return Message::Content(format!("You don't know how to make a oak dining set!"));
					}
					if player.lumber.oak + player.queued_lumber(tree) < 5 * actions {
						return Message::Content(format!("You don't have enough oak lumber, you need **{}** more", (5 * actions) - player.lumber.oak + player.queued_lumber(tree)));
					}

				player.lumber.oak -= 5 * actions;
				}
			}
		},
		Tree::Maple => {
			if player.hammer < Hammer::Steel {
				return Message::Content("You need a **Steel** hammer to build with maple lumber!".to_string());
			}
			match furniture {
				Furniture::BirdHouse => {
					if !player.blueprints.maple.birdhouse {
						return Message::Content(format!("You don't know how to make a maple bird house!"));
					}
					if player.lumber.maple + player.queued_lumber(tree) < 1 * actions {
						return Message::Content(format!("You don't have enough maple lumber, you need **{}** more", (1 * actions) - player.lumber.maple + player.queued_lumber(tree)));
					}

				player.lumber.maple -= 1 * actions;
				},
				Furniture::Shelf => {
					if !player.blueprints.maple.shelf {
						return Message::Content(format!("You don't know how to make a maple shelf!"));
					}
					if player.lumber.maple + player.queued_lumber(tree) < 2 * actions {
						return Message::Content(format!("You don't have enough maple lumber, you need **{}** more", (2 * actions) - player.lumber.maple + player.queued_lumber(tree)));
					}

				player.lumber.maple -= 2 * actions;
				},
				Furniture::SideTable => {
					if !player.blueprints.maple.side_table {
						return Message::Content(format!("You don't know how to make a maple side table!"));
					}
					if player.lumber.maple + player.queued_lumber(tree) < 3 * actions {
						return Message::Content(format!("You don't have enough maple lumber, you need **{}** more", (3 * actions) - player.lumber.maple + player.queued_lumber(tree)));
					}

				player.lumber.maple -= 3 * actions;
				},
				Furniture::CoffeeTable => {
					if !player.blueprints.maple.coffee_table {
						return Message::Content(format!("You don't know how to make a maple coffee table!"));
					}
					if player.lumber.maple + player.queued_lumber(tree) < 4 * actions {
						return Message::Content(format!("You don't have enough maple lumber, you need **{}** more", (4 * actions) - player.lumber.maple + player.queued_lumber(tree)));
					}

				player.lumber.maple -= 4 * actions;
				},
				Furniture::DiningSet => {
					if !player.blueprints.maple.dining_set {
						return Message::Content(format!("You don't know how to make a maple dining set!"));
					}
					if player.lumber.maple + player.queued_lumber(tree) < 5 * actions {
						return Message::Content(format!("You don't have enough maple lumber, you need **{}** more", (5 * actions) - player.lumber.maple + player.queued_lumber(tree)));
					}

				player.lumber.maple -= 5 * actions;
				}
			}
		},
		Tree::Walnut => {
			if player.hammer < Hammer::Mithril {
				return Message::Content("You need a **Mithril** hammer to build with walnut lumber!".to_string());
			}
			match furniture {
				Furniture::BirdHouse => {
					if !player.blueprints.walnut.birdhouse {
						return Message::Content(format!("You don't know how to make a walnut bird house!"));
					}
					if player.lumber.walnut + player.queued_lumber(tree) < 1 * actions {
						return Message::Content(format!("You don't have enough walnut lumber, you need **{}** more", (1 * actions) - player.lumber.walnut + player.queued_lumber(tree)));
					}

				player.lumber.walnut -= 1 * actions;
				},
				Furniture::Shelf => {
					if !player.blueprints.walnut.shelf {
						return Message::Content(format!("You don't know how to make a walnut shelf!"));
					}
					if player.lumber.walnut + player.queued_lumber(tree) < 2 * actions {
						return Message::Content(format!("You don't have enough walnut lumber, you need **{}** more", (2 * actions) - player.lumber.walnut + player.queued_lumber(tree)));
					}

				player.lumber.walnut -= 2 * actions;
				},
				Furniture::SideTable => {
					if !player.blueprints.walnut.side_table {
						return Message::Content(format!("You don't know how to make a walnut side table!"));
					}
					if player.lumber.walnut + player.queued_lumber(tree) < 3 * actions {
						return Message::Content(format!("You don't have enough walnut lumber, you need **{}** more", (3 * actions) - player.lumber.walnut + player.queued_lumber(tree)));
					}

				player.lumber.walnut -= 3 * actions;
				},
				Furniture::CoffeeTable => {
					if !player.blueprints.walnut.coffee_table {
						return Message::Content(format!("You don't know how to make a walnut coffee table!"));
					}
					if player.lumber.walnut + player.queued_lumber(tree) < 4 * actions {
						return Message::Content(format!("You don't have enough walnut lumber, you need **{}** more", (4 * actions) - player.lumber.walnut + player.queued_lumber(tree)));
					}

				player.lumber.walnut -= 4 * actions;
				},
				Furniture::DiningSet => {
					if !player.blueprints.walnut.dining_set {
						return Message::Content(format!("You don't know how to make a walnut dining set!"));
					}
					if player.lumber.walnut + player.queued_lumber(tree) < 5 * actions {
						return Message::Content(format!("You don't have enough walnut lumber, you need **{}** more", (5 * actions) - player.lumber.walnut + player.queued_lumber(tree)));
					}

				player.lumber.walnut -= 5 * actions;
				}
			}
		},
		Tree::Cherry => {
			if player.hammer < Hammer::Adamant {
				return Message::Content("You need a **Adamant** hammer to build with cherry lumber!".to_string());
			}
			match furniture {
				Furniture::BirdHouse => {
					if !player.blueprints.cherry.birdhouse {
						return Message::Content(format!("You don't know how to make a cherry bird house!"));
					}
					if player.lumber.cherry + player.queued_lumber(tree) < 1 * actions {
						return Message::Content(format!("You don't have enough cherry lumber, you need **{}** more", (1 * actions) - player.lumber.cherry + player.queued_lumber(tree)));
					}

				player.lumber.cherry -= 1 * actions;
				},
				Furniture::Shelf => {
					if !player.blueprints.cherry.shelf {
						return Message::Content(format!("You don't know how to make a cherry shelf!"));
					}
					if player.lumber.cherry + player.queued_lumber(tree) < 2 * actions {
						return Message::Content(format!("You don't have enough cherry lumber, you need **{}** more", (2 * actions) - player.lumber.cherry + player.queued_lumber(tree)));
					}

				player.lumber.cherry -= 2 * actions;
				},
				Furniture::SideTable => {
					if !player.blueprints.cherry.side_table {
						return Message::Content(format!("You don't know how to make a cherry side table!"));
					}
					if player.lumber.cherry + player.queued_lumber(tree) < 3 * actions {
						return Message::Content(format!("You don't have enough cherry lumber, you need **{}** more", (3 * actions) - player.lumber.cherry + player.queued_lumber(tree)));
					}

				player.lumber.cherry -= 3 * actions;
				},
				Furniture::CoffeeTable => {
					if !player.blueprints.cherry.coffee_table {
						return Message::Content(format!("You don't know how to make a cherry coffee table!"));
					}
					if player.lumber.cherry + player.queued_lumber(tree) < 4 * actions {
						return Message::Content(format!("You don't have enough cherry lumber, you need **{}** more", (4 * actions) - player.lumber.cherry + player.queued_lumber(tree)));
					}

				player.lumber.cherry -= 4 * actions;
				},
				Furniture::DiningSet => {
					if !player.blueprints.cherry.dining_set {
						return Message::Content(format!("You don't know how to make a cherry dining set!"));
					}
					if player.lumber.cherry + player.queued_lumber(tree) < 5 * actions {
						return Message::Content(format!("You don't have enough cherry lumber, you need **{}** more", (5 * actions) - player.lumber.cherry + player.queued_lumber(tree)));
					}

				player.lumber.cherry -= 5 * actions;
				}
			}
		},
		Tree::PurpleHeart => {
			if player.hammer < Hammer::Rune {
				return Message::Content("You need a **Rune** hammer to build with purpleheart lumber!".to_string());
			}
			match furniture {
				Furniture::BirdHouse => {
					if !player.blueprints.purpleheart.birdhouse {
						return Message::Content(format!("You don't know how to make a purpleheart bird house!"));
					}
					if player.lumber.purpleheart + player.queued_lumber(tree) < 1 * actions {
						return Message::Content(format!("You don't have enough purpleheart lumber, you need **{}** more", (1 * actions) - player.lumber.purpleheart + player.queued_lumber(tree)));
					}

				player.lumber.purpleheart -= 1 * actions;
				},
				Furniture::Shelf => {
					if !player.blueprints.purpleheart.shelf {
						return Message::Content(format!("You don't know how to make a purpleheart shelf!"));
					}
					if player.lumber.purpleheart + player.queued_lumber(tree) < 2 * actions {
						return Message::Content(format!("You don't have enough purpleheart lumber, you need **{}** more", (2 * actions) - player.lumber.purpleheart + player.queued_lumber(tree)));
					}

				player.lumber.purpleheart -= 2 * actions;
				},
				Furniture::SideTable => {
					if !player.blueprints.purpleheart.side_table {
						return Message::Content(format!("You don't know how to make a purpleheart side table!"));
					}
					if player.lumber.purpleheart + player.queued_lumber(tree) < 3 * actions {
						return Message::Content(format!("You don't have enough purpleheart lumber, you need **{}** more", (3 * actions) - player.lumber.purpleheart + player.queued_lumber(tree)));
					}

				player.lumber.purpleheart -= 3 * actions;
				},
				Furniture::CoffeeTable => {
					if !player.blueprints.purpleheart.coffee_table {
						return Message::Content(format!("You don't know how to make a purpleheart coffee table!"));
					}
					if player.lumber.purpleheart + player.queued_lumber(tree) < 4 * actions {
						return Message::Content(format!("You don't have enough purpleheart lumber, you need **{}** more", (4 * actions) - player.lumber.purpleheart + player.queued_lumber(tree)));
					}

				player.lumber.purpleheart -= 4 * actions;
				},
				Furniture::DiningSet => {
					if !player.blueprints.purpleheart.dining_set {
						return Message::Content(format!("You don't know how to make a purpleheart dining set!"));
					}
					if player.lumber.purpleheart + player.queued_lumber(tree) < 5 * actions {
						return Message::Content(format!("You don't have enough purpleheart lumber, you need **{}** more", (5 * actions) - player.lumber.purpleheart + player.queued_lumber(tree)));
					}

				player.lumber.purpleheart -= 5 * actions;
				}
			}
		}
	}
	player.update().await;

	Message::Content(build_player_update(&mut player, tree, furniture, actions).await)
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("build").description("Build furniture with lumber!")
		.create_option(|option| {
			option
				.name("pine")
				.description("build with pine lumber")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
		})
		.create_option(|option| {
			option
				.name("oak")
				.description("build with oak lumber")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})	
		})
		.create_option(|option| {
			option
				.name("maple")
				.description("build with maple lumber")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})	
		})
		.create_option(|option| {
			option
				.name("walnut")
				.description("build with walnut lumber")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})	
		})
		.create_option(|option| {
			option
				.name("cherry")
				.description("build with cherry lumber")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})	
		})
		.create_option(|option| {
			option
				.name("purpleheart")
				.description("build with purpleheart lumber")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|sub| {
							sub
								.name("amount")
								.description("amount to chop")
								.kind(CommandOptionType::Integer)
								.required(false)
								.min_int_value(1)
						})
				})	
		})
}

fn build_furniture(player: &Player, tree: Tree, furniture: Furniture, actions: i64) -> Option<Action> {
	// returns None if insta-dried.
	let build_time = get_player_build_time(player, tree, furniture, actions);
	if build_time == 0 {
		return None;
	} else {
		return Some(Action::building(build_time, tree, furniture, actions));
	}
}

pub fn determine_player_furniture_earned(player: &Player) -> i64 {
	let base_furniture = 1;
	let upgrade = player.upgrades.industrial_nails;
	let sawdust_upgrade = player.sawdust_upgrades.self_tapping_screws;
	
	(base_furniture + upgrade) * (1 + sawdust_upgrade)
}

pub fn determine_cnc_furniture_earned(player: &Player) -> i64 {
	let base_furniture = 1;
	let upgrade = player.upgrades.high_quality_bits;
	let sawdust_upgrade = player.sawdust_upgrades.stronger_motors;
	
	(base_furniture + upgrade) * (1 + sawdust_upgrade)
}

pub async fn build_player_update(player: &mut Player, tree: Tree, furniture: Furniture, actions: i64) -> String {
	let action = build_furniture(&player, tree, furniture, actions);
	match action {
		Some(a) => {
			match player.current_action.action {
				ActionEnum::None => {
					player.current_action = a.clone();
					player.update().await;

					format!("You started to build a **{} {}**! You'll be done in **{}s**", tree, furniture, a.time_to_complete())
				},
				_ => {
					let queued_action = player.queue_action(a);
					player.update().await;

					format!("You started to build a **{} {}**! You'll be done in **{}s**", tree, furniture, queued_action.time_to_complete())
				},
			}
		}
		None => {
			let amount = update_player_build(player);
			player.update().await;

			format!("You built **{} {} {}**!", amount, tree, furniture)
		}
	}
}

pub fn update_player_build(player: &mut Player) -> i64 {
	let times = player.current_action.amount;
	let amount = times * determine_player_furniture_earned(&player);
	let tree = player.current_action.tree;
	let furniture = player.current_action.furniture.unwrap();
	player.current_action = Action::none();
	match tree {
		Tree::Pine => match furniture {
			Furniture::BirdHouse => {
				player.furniture.pine.birdhouse += amount;
				player.sawdust_prestige.furniture.pine.birdhouse += amount;
				player.seed_prestige.furniture.pine.birdhouse += amount;
				player.stats.pine_birdhouses_built += amount;
			},
			Furniture::Shelf => {
				player.furniture.pine.shelf += amount;
				player.sawdust_prestige.furniture.pine.shelf += amount;
				player.seed_prestige.furniture.pine.shelf += amount;
				player.stats.pine_shelves_built += amount;
			},
			Furniture::SideTable => {
				player.furniture.pine.side_table += amount;
				player.sawdust_prestige.furniture.pine.side_table += amount;
				player.seed_prestige.furniture.pine.side_table += amount;
				player.stats.pine_side_tables_built += amount;
			},
			Furniture::CoffeeTable => {
				player.furniture.pine.coffee_table += amount;
				player.sawdust_prestige.furniture.pine.coffee_table += amount;
				player.seed_prestige.furniture.pine.coffee_table += amount;
				player.stats.pine_coffee_tables_built += amount;
			},
			Furniture::DiningSet => {
				player.furniture.pine.dining_set += amount;
				player.sawdust_prestige.furniture.pine.dining_set += amount;
				player.seed_prestige.furniture.pine.dining_set += amount;
				player.stats.pine_dining_sets_built += amount;
			}
		},
		Tree::Oak => match furniture {
			Furniture::BirdHouse => {
				player.furniture.oak.birdhouse += amount;
				player.sawdust_prestige.furniture.oak.birdhouse += amount;
				player.seed_prestige.furniture.oak.birdhouse += amount;
				player.stats.oak_birdhouses_built += amount;
			},
			Furniture::Shelf => {
				player.furniture.oak.shelf += amount;
				player.sawdust_prestige.furniture.oak.shelf += amount;
				player.seed_prestige.furniture.oak.shelf += amount;
				player.stats.oak_shelves_built += amount;
			},
			Furniture::SideTable => {
				player.furniture.oak.side_table += amount;
				player.sawdust_prestige.furniture.oak.side_table += amount;
				player.seed_prestige.furniture.oak.side_table += amount;
				player.stats.oak_side_tables_built += amount;
			},
			Furniture::CoffeeTable => {
				player.furniture.oak.coffee_table += amount;
				player.sawdust_prestige.furniture.oak.coffee_table += amount;
				player.seed_prestige.furniture.oak.coffee_table += amount;
				player.stats.oak_coffee_tables_built += amount;
			},
			Furniture::DiningSet => {
				player.furniture.oak.dining_set += amount;
				player.sawdust_prestige.furniture.oak.dining_set += amount;
				player.seed_prestige.furniture.oak.dining_set += amount;
				player.stats.oak_dining_sets_built += amount;
			}
		},
		Tree::Maple => match furniture {
			Furniture::BirdHouse => {
				player.furniture.maple.birdhouse += amount;
				player.sawdust_prestige.furniture.maple.birdhouse += amount;
				player.seed_prestige.furniture.maple.birdhouse += amount;
				player.stats.maple_birdhouses_built += amount;
			},
			Furniture::Shelf => {
				player.furniture.maple.shelf += amount;
				player.sawdust_prestige.furniture.maple.shelf += amount;
				player.seed_prestige.furniture.maple.shelf += amount;
				player.stats.maple_shelves_built += amount;
			},
			Furniture::SideTable => {
				player.furniture.maple.side_table += amount;
				player.sawdust_prestige.furniture.maple.side_table += amount;
				player.seed_prestige.furniture.maple.side_table += amount;
				player.stats.maple_side_tables_built += amount;
			},
			Furniture::CoffeeTable => {
				player.furniture.maple.coffee_table += amount;
				player.sawdust_prestige.furniture.maple.coffee_table += amount;
				player.seed_prestige.furniture.maple.coffee_table += amount;
				player.stats.maple_coffee_tables_built += amount;
			},
			Furniture::DiningSet => {
				player.furniture.maple.dining_set += amount;
				player.sawdust_prestige.furniture.maple.dining_set += amount;
				player.seed_prestige.furniture.maple.dining_set += amount;
				player.stats.maple_dining_sets_built += amount;
			}
		},
		Tree::Walnut => match furniture {
			Furniture::BirdHouse => {
				player.furniture.walnut.birdhouse += amount;
				player.sawdust_prestige.furniture.walnut.birdhouse += amount;
				player.seed_prestige.furniture.walnut.birdhouse += amount;
				player.stats.walnut_birdhouses_built += amount;
			},
			Furniture::Shelf => {
				player.furniture.walnut.shelf += amount;
				player.sawdust_prestige.furniture.walnut.shelf += amount;
				player.seed_prestige.furniture.walnut.shelf += amount;
				player.stats.walnut_shelves_built += amount;
			},
			Furniture::SideTable => {
				player.furniture.walnut.side_table += amount;
				player.sawdust_prestige.furniture.walnut.side_table += amount;
				player.seed_prestige.furniture.walnut.side_table += amount;
				player.stats.walnut_side_tables_built += amount;
			},
			Furniture::CoffeeTable => {
				player.furniture.walnut.coffee_table += amount;
				player.sawdust_prestige.furniture.walnut.coffee_table += amount;
				player.seed_prestige.furniture.walnut.coffee_table += amount;
				player.stats.walnut_coffee_tables_built += amount;
			},
			Furniture::DiningSet => {
				player.furniture.walnut.dining_set += amount;
				player.sawdust_prestige.furniture.walnut.dining_set += amount;
				player.seed_prestige.furniture.walnut.dining_set += amount;
				player.stats.walnut_dining_sets_built += amount;
			}
		},
		Tree::Cherry => match furniture {
			Furniture::BirdHouse => {
				player.furniture.cherry.birdhouse += amount;
				player.sawdust_prestige.furniture.cherry.birdhouse += amount;
				player.seed_prestige.furniture.cherry.birdhouse += amount;
				player.stats.cherry_birdhouses_built += amount;
			},
			Furniture::Shelf => {
				player.furniture.cherry.shelf += amount;
				player.sawdust_prestige.furniture.cherry.shelf += amount;
				player.seed_prestige.furniture.cherry.shelf += amount;
				player.stats.cherry_shelves_built += amount;
			},
			Furniture::SideTable => {
				player.furniture.cherry.side_table += amount;
				player.sawdust_prestige.furniture.cherry.side_table += amount;
				player.seed_prestige.furniture.cherry.side_table += amount;
				player.stats.cherry_side_tables_built += amount;
			},
			Furniture::CoffeeTable => {
				player.furniture.cherry.coffee_table += amount;
				player.sawdust_prestige.furniture.cherry.coffee_table += amount;
				player.seed_prestige.furniture.cherry.coffee_table += amount;
				player.stats.cherry_coffee_tables_built += amount;
			},
			Furniture::DiningSet => {
				player.furniture.cherry.dining_set += amount;
				player.sawdust_prestige.furniture.cherry.dining_set += amount;
				player.seed_prestige.furniture.cherry.dining_set += amount;
				player.stats.cherry_dining_sets_built += amount;
			}
		},
		Tree::PurpleHeart => match furniture {
			Furniture::BirdHouse => {
				player.furniture.purpleheart.birdhouse += amount;
				player.sawdust_prestige.furniture.purpleheart.birdhouse += amount;
				player.seed_prestige.furniture.purpleheart.birdhouse += amount;
				player.stats.purpleheart_birdhouses_built += amount;
			},
			Furniture::Shelf => {
				player.furniture.purpleheart.shelf += amount;
				player.sawdust_prestige.furniture.purpleheart.shelf += amount;
				player.seed_prestige.furniture.purpleheart.shelf += amount;
				player.stats.purpleheart_shelves_built += amount;
			},
			Furniture::SideTable => {
				player.furniture.purpleheart.side_table += amount;
				player.sawdust_prestige.furniture.purpleheart.side_table += amount;
				player.seed_prestige.furniture.purpleheart.side_table += amount;
				player.stats.purpleheart_side_tables_built += amount;
			},
			Furniture::CoffeeTable => {
				player.furniture.purpleheart.coffee_table += amount;
				player.sawdust_prestige.furniture.purpleheart.coffee_table += amount;
				player.seed_prestige.furniture.purpleheart.coffee_table += amount;
				player.stats.purpleheart_coffee_tables_built += amount;
			},
			Furniture::DiningSet => {
				player.furniture.purpleheart.dining_set += amount;
				player.sawdust_prestige.furniture.purpleheart.dining_set += amount;
				player.seed_prestige.furniture.purpleheart.dining_set += amount;
				player.stats.purpleheart_dining_sets_built += amount;
			}
		}
	}

	amount
}

pub fn get_player_build_time(player: &Player, tree: Tree, furniture: Furniture, actions: i64) -> i64 {
	let base_time = match tree {
		Tree::Pine => 10.0,
		Tree::Oak => 15.0,
		Tree::Maple => 25.0,
		Tree::Walnut => 35.0,
		Tree::Cherry => 50.0,
		Tree::PurpleHeart => 80.0
	};
	let furniture_mult = match furniture {
		Furniture::BirdHouse => 1.0,
		Furniture::Shelf => 1.5,
		Furniture::SideTable => 2.0,
		Furniture::CoffeeTable => 2.5,
		Furniture::DiningSet => 3.0
	};
	let base_time = base_time * furniture_mult;
	let upgrade_mult = 1.0 + (player.upgrades.fast_drying_glue as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.longer_clamps as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult).round() as i64 * actions
}

pub fn get_cnc_build_time(player: &Player, tree: Tree, furniture: Furniture, actions: i64) -> i64 {
	let base_time = match tree {
		Tree::Pine => 100.0,
		Tree::Oak => 150.0,
		Tree::Maple => 250.0,
		Tree::Walnut => 350.0,
		Tree::Cherry => 500.0,
		Tree::PurpleHeart => 800.0
	};
	let furniture_mult = match furniture {
		Furniture::BirdHouse => 1.0,
		Furniture::Shelf => 1.5,
		Furniture::SideTable => 2.0,
		Furniture::CoffeeTable => 2.5,
		Furniture::DiningSet => 3.0
	};
	let base_time = base_time * furniture_mult;
	let upgrade_mult = 1.0 + (player.upgrades.wd_40 as f64 * 0.1);
	let sawdust_mult = 1.0 + (player.sawdust_upgrades.saved_gcode as f64 * 0.1);

	((base_time / upgrade_mult) / sawdust_mult).round() as i64 * actions
}