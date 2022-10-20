use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
};

use crate::player::{get_player, Hammer, Player, Action, ActionEnum};
use crate::utils;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> String {
	let tree = &options
		.get(0)
		.expect("Expected a Subcommand")
		.name;
	let furniture = &options
		.get(0)
		.expect("Expected subcommand")
		.options
		.get(0)
		.expect("Expected string")
		.name;

	let mut player = get_player(player_id).await;
	match player.current_action.action {
		ActionEnum::None => (),
		_ => return format!("You're busy for another **{}s**!", player.current_action.time_to_complete()),
	}
	if player.hammer == Hammer::None {
		return "You don't have a hammer! Buy one from the store!".to_string();
	}
	match tree.as_str() {
		// Don't need to check hammer::none since we do it above.
		"pine" => match furniture.as_str() {
			"birdhouse" => {
				if player.lumber.pine < 1 {
					return format!("You don't have enough pine lumber, you need **{}** more", 1 - player.lumber.pine);
				}

				build_player_update(&mut player, "pine", "birdhouse").await
			},
			"shelf" => {
				if player.lumber.pine < 2 {
					return format!("You don't have enough pine lumber, you need **{}** more", 2 - player.lumber.pine);
				}

				build_player_update(&mut player, "pine", "shelf").await
			},
			"sidetable" => {
				if player.lumber.pine < 3 {
					return format!("You don't have enough pine lumber, you need **{}** more", 3 - player.lumber.pine);
				}

				build_player_update(&mut player, "pine", "sidetable").await
			},
			"coffeetable" => {
				if player.lumber.pine < 4 {
					return format!("You don't have enough pine lumber, you need **{}** more", 4 - player.lumber.pine);
				}

				build_player_update(&mut player, "pine", "coffeetable").await
			},
			"diningset" => {
				if player.lumber.pine < 5 {
					return format!("You don't have enough pine lumber, you need **{}** more", 5 - player.lumber.pine);
				}

				build_player_update(&mut player, "pine", "diningset").await
			},
			_ => "No such furniture".to_string()
		},
		"oak" => {
			if player.hammer < Hammer::Iron {
				return "You need a **Iron** hammer to build with oak lumber!".to_string();
			}
			match furniture.as_str() {
				"birdhouse" => {
					if player.lumber.oak < 1 {
						return format!("You don't have enough oak lumber, you need **{}** more", 1 - player.lumber.oak);
					}
	
					build_player_update(&mut player, "oak", "birdhouse").await
				},
				"shelf" => {
					if player.lumber.oak < 2 {
						return format!("You don't have enough oak lumber, you need **{}** more", 2 - player.lumber.oak);
					}
	
					build_player_update(&mut player, "oak", "shelf").await
				},
				"sidetable" => {
					if player.lumber.oak < 3 {
						return format!("You don't have enough oak lumber, you need **{}** more", 3 - player.lumber.oak);
					}
	
					build_player_update(&mut player, "oak", "sidetable").await
				},
				"coffeetable" => {
					if player.lumber.oak < 4 {
						return format!("You don't have enough oak lumber, you need **{}** more", 4 - player.lumber.oak);
					}
	
					build_player_update(&mut player, "oak", "coffeetable").await
				},
				"diningset" => {
					if player.lumber.oak < 5 {
						return format!("You don't have enough oak lumber, you need **{}** more", 5 - player.lumber.oak);
					}
	
					build_player_update(&mut player, "oak", "diningset").await
				},
				_ => "No such furniture".to_string()
			}
		},
		"maple" => {
			if player.hammer < Hammer::Steel {
				return "You need a **Steel** hammer to build with maple lumber!".to_string();
			}
			match furniture.as_str() {
				"birdhouse" => {
					if player.lumber.maple < 1 {
						return format!("You don't have enough maple lumber, you need **{}** more", 1 - player.lumber.maple);
					}
	
					build_player_update(&mut player, "maple", "birdhouse").await
				},
				"shelf" => {
					if player.lumber.maple < 2 {
						return format!("You don't have enough maple lumber, you need **{}** more", 2 - player.lumber.maple);
					}
	
					build_player_update(&mut player, "maple", "shelf").await
				},
				"sidetable" => {
					if player.lumber.maple < 3 {
						return format!("You don't have enough maple lumber, you need **{}** more", 3 - player.lumber.maple);
					}
	
					build_player_update(&mut player, "maple", "sidetable").await
				},
				"coffeetable" => {
					if player.lumber.maple < 4 {
						return format!("You don't have enough maple lumber, you need **{}** more", 4 - player.lumber.maple);
					}
	
					build_player_update(&mut player, "maple", "coffeetable").await
				},
				"diningset" => {
					if player.lumber.maple < 5 {
						return format!("You don't have enough maple lumber, you need **{}** more", 5 - player.lumber.maple);
					}
	
					build_player_update(&mut player, "maple", "diningset").await
				},
				_ => "No such furniture".to_string()
			}
		},
		"walnut" => {
			if player.hammer < Hammer::Mithril {
				return "You need a **Mithril** hammer to build with walnut lumber!".to_string();
			}
			match furniture.as_str() {
				"birdhouse" => {
					if player.lumber.walnut < 1 {
						return format!("You don't have enough walnut lumber, you need **{}** more", 1 - player.lumber.walnut);
					}
	
					build_player_update(&mut player, "walnut", "birdhouse").await
				},
				"shelf" => {
					if player.lumber.walnut < 2 {
						return format!("You don't have enough walnut lumber, you need **{}** more", 2 - player.lumber.walnut);
					}
	
					build_player_update(&mut player, "walnut", "shelf").await
				},
				"sidetable" => {
					if player.lumber.walnut < 3 {
						return format!("You don't have enough walnut lumber, you need **{}** more", 3 - player.lumber.walnut);
					}
	
					build_player_update(&mut player, "walnut", "sidetable").await
				},
				"coffeetable" => {
					if player.lumber.walnut < 4 {
						return format!("You don't have enough walnut lumber, you need **{}** more", 4 - player.lumber.walnut);
					}
	
					build_player_update(&mut player, "walnut", "coffeetable").await
				},
				"diningset" => {
					if player.lumber.walnut < 5 {
						return format!("You don't have enough walnut lumber, you need **{}** more", 5 - player.lumber.walnut);
					}
	
					build_player_update(&mut player, "walnut", "diningset").await
				},
				_ => "No such furniture".to_string()
			}
		},
		"cherry" => {
			if player.hammer < Hammer::Adamant {
				return "You need a **Adamant** hammer to build with cherry lumber!".to_string();
			}
			match furniture.as_str() {
				"birdhouse" => {
					if player.lumber.cherry < 1 {
						return format!("You don't have enough cherry lumber, you need **{}** more", 1 - player.lumber.cherry);
					}
	
					build_player_update(&mut player, "cherry", "birdhouse").await
				},
				"shelf" => {
					if player.lumber.cherry < 2 {
						return format!("You don't have enough cherry lumber, you need **{}** more", 2 - player.lumber.cherry);
					}
	
					build_player_update(&mut player, "cherry", "shelf").await
				},
				"sidetable" => {
					if player.lumber.cherry < 3 {
						return format!("You don't have enough cherry lumber, you need **{}** more", 3 - player.lumber.cherry);
					}
	
					build_player_update(&mut player, "cherry", "sidetable").await
				},
				"coffeetable" => {
					if player.lumber.cherry < 4 {
						return format!("You don't have enough cherry lumber, you need **{}** more", 4 - player.lumber.cherry);
					}
	
					build_player_update(&mut player, "cherry", "coffeetable").await
				},
				"diningset" => {
					if player.lumber.cherry < 5 {
						return format!("You don't have enough cherry lumber, you need **{}** more", 5 - player.lumber.cherry);
					}
	
					build_player_update(&mut player, "cherry", "diningset").await
				},
				_ => "No such furniture".to_string()
			}
		},
		"purpleheart" => {
			if player.hammer < Hammer::Rune {
				return "You need a **Rune** hammer to build with purpleheart lumber!".to_string();
			}
			match furniture.as_str() {
				"birdhouse" => {
					if player.lumber.purpleheart < 1 {
						return format!("You don't have enough purpleheart lumber, you need **{}** more", 1 - player.lumber.purpleheart);
					}
	
					build_player_update(&mut player, "purpleheart", "birdhouse").await
				},
				"shelf" => {
					if player.lumber.purpleheart < 2 {
						return format!("You don't have enough purpleheart lumber, you need **{}** more", 2 - player.lumber.purpleheart);
					}
	
					build_player_update(&mut player, "purpleheart", "shelf").await
				},
				"sidetable" => {
					if player.lumber.purpleheart < 3 {
						return format!("You don't have enough purpleheart lumber, you need **{}** more", 3 - player.lumber.purpleheart);
					}
	
					build_player_update(&mut player, "purpleheart", "sidetable").await
				},
				"coffeetable" => {
					if player.lumber.purpleheart < 4 {
						return format!("You don't have enough purpleheart lumber, you need **{}** more", 4 - player.lumber.purpleheart);
					}
	
					build_player_update(&mut player, "purpleheart", "coffeetable").await
				},
				"diningset" => {
					if player.lumber.purpleheart < 5 {
						return format!("You don't have enough purpleheart lumber, you need **{}** more", 5 - player.lumber.purpleheart);
					}
	
					build_player_update(&mut player, "purpleheart", "diningset").await
				},
				_ => "No such furniture".to_string()
			}
		},
		_ => "No such tree".to_string()
	}
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("build").description("Build furniture with lumber!")
		.create_option(|option| {
			option
				.name("pine")
				.description("Dry a pine log")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
				})
		})
		.create_option(|option| {
			option
				.name("oak")
				.description("Dry an oak log")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
				})	
		})
		.create_option(|option| {
			option
				.name("maple")
				.description("Dry a maple log")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
				})	
		})
		.create_option(|option| {
			option
				.name("walnut")
				.description("Dry a walnut log")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
				})	
		})
		.create_option(|option| {
			option
				.name("cherry")
				.description("Dry a cherry log")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
				})	
		})
		.create_option(|option| {
			option
				.name("purpleheart")
				.description("Dry a purpleheart log")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("birdhouse")
						.description("build a bird house")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("shelf")
						.description("build a shelf")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("sidetable")
						.description("build a side table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("coffeetable")
						.description("build a coffee table")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("diningset")
						.description("build a dining set")
						.kind(CommandOptionType::SubCommand)
				})	
		})
}

fn build_furniture(player: &Player, tree: &str, furniture: &str) -> Option<Action> {
	// returns None if insta-dried.
	let build_time = utils::get_build_time(player, tree, furniture);
	if build_time == 0 {
		return None;
	} else {
		return Some(Action::building(build_time, tree, furniture));
	}
}

pub fn determine_furniture_earned(player: &Player) -> i64 {
	let base_furniture = 1;
	let upgrade = player.upgrades.industrial_nails;
	let sawdust_upgrade = player.sawdust_upgrades.industrial_nails;
	
	(base_furniture + upgrade) * (1 + sawdust_upgrade)
}

pub async fn build_player_update(player: &mut Player, tree: &str, furniture: &str) -> String {
	let action = build_furniture(&player, tree, furniture);
	match action {
		Some(a) => {
			player.current_action = a.clone();
			player.update().await;
			
			format!("You started build a **{} {}**! You'll be done in **{}s**", tree, furniture, a.time_to_complete())
		}
		None => {
			let amount = determine_furniture_earned(&player);
			update_player_build(player, amount, tree, furniture);
			player.update().await;

			format!("You built **{} {} {}**!", amount, tree, furniture)
		}
	}
}

pub fn update_player_build(player: &mut Player, amount: i64, tree: &str, furniture: &str) {
	player.current_action = Action::none();
	match tree {
		"pine" => match furniture {
			"birdhouse" => {
				player.lumber.pine -= 1;
				player.furniture.pine.birdhouse += amount;
				player.sawdust_prestige.furniture.pine.birdhouse += amount;
				player.seed_prestige.furniture.pine.birdhouse += amount;
			},
			"shelf" => {
				player.lumber.pine -= 2;
				player.furniture.pine.shelf += amount;
				player.sawdust_prestige.furniture.pine.shelf += amount;
				player.seed_prestige.furniture.pine.shelf += amount;
			},
			"sidetable" => {
				player.lumber.pine -= 3;
				player.furniture.pine.side_table += amount;
				player.sawdust_prestige.furniture.pine.side_table += amount;
				player.seed_prestige.furniture.pine.side_table += amount;
			},
			"coffeetable" => {
				player.lumber.pine -= 4;
				player.furniture.pine.coffee_table += amount;
				player.sawdust_prestige.furniture.pine.coffee_table += amount;
				player.seed_prestige.furniture.pine.coffee_table += amount;
			},
			"diningset" => {
				player.lumber.pine -= 5;
				player.furniture.pine.dining_set += amount;
				player.sawdust_prestige.furniture.pine.dining_set += amount;
				player.seed_prestige.furniture.pine.dining_set += amount;
			},
			_ => (),
		},
		"oak" => match furniture {
			"birdhouse" => {
				player.lumber.oak -= 1;
				player.furniture.oak.birdhouse += amount;
				player.sawdust_prestige.furniture.oak.birdhouse += amount;
				player.seed_prestige.furniture.oak.birdhouse += amount;
			},
			"shelf" => {
				player.lumber.oak -= 2;
				player.furniture.oak.shelf += amount;
				player.sawdust_prestige.furniture.oak.shelf += amount;
				player.seed_prestige.furniture.oak.shelf += amount;
			},
			"sidetable" => {
				player.lumber.oak -= 3;
				player.furniture.oak.side_table += amount;
				player.sawdust_prestige.furniture.oak.side_table += amount;
				player.seed_prestige.furniture.oak.side_table += amount;
			},
			"coffeetable" => {
				player.lumber.oak -= 4;
				player.furniture.oak.coffee_table += amount;
				player.sawdust_prestige.furniture.oak.coffee_table += amount;
				player.seed_prestige.furniture.oak.coffee_table += amount;
			},
			"diningset" => {
				player.lumber.oak -= 5;
				player.furniture.oak.dining_set += amount;
				player.sawdust_prestige.furniture.oak.dining_set += amount;
				player.seed_prestige.furniture.oak.dining_set += amount;
			},
			_ => (),
		},
		"maple" => match furniture {
			"birdhouse" => {
				player.lumber.maple -= 1;
				player.furniture.maple.birdhouse += amount;
				player.sawdust_prestige.furniture.maple.birdhouse += amount;
				player.seed_prestige.furniture.maple.birdhouse += amount;
			},
			"shelf" => {
				player.lumber.maple -= 2;
				player.furniture.maple.shelf += amount;
				player.sawdust_prestige.furniture.maple.shelf += amount;
				player.seed_prestige.furniture.maple.shelf += amount;
			},
			"sidetable" => {
				player.lumber.maple -= 3;
				player.furniture.maple.side_table += amount;
				player.sawdust_prestige.furniture.maple.side_table += amount;
				player.seed_prestige.furniture.maple.side_table += amount;
			},
			"coffeetable" => {
				player.lumber.maple -= 4;
				player.furniture.maple.coffee_table += amount;
				player.sawdust_prestige.furniture.maple.coffee_table += amount;
				player.seed_prestige.furniture.maple.coffee_table += amount;
			},
			"diningset" => {
				player.lumber.maple -= 5;
				player.furniture.maple.dining_set += amount;
				player.sawdust_prestige.furniture.maple.dining_set += amount;
				player.seed_prestige.furniture.maple.dining_set += amount;
			},
			_ => (),
		},
		"walnut" => match furniture {
			"birdhouse" => {
				player.lumber.walnut -= 1;
				player.furniture.walnut.birdhouse += amount;
				player.sawdust_prestige.furniture.walnut.birdhouse += amount;
				player.seed_prestige.furniture.walnut.birdhouse += amount;
			},
			"shelf" => {
				player.lumber.walnut -= 2;
				player.furniture.walnut.shelf += amount;
				player.sawdust_prestige.furniture.walnut.shelf += amount;
				player.seed_prestige.furniture.walnut.shelf += amount;
			},
			"sidetable" => {
				player.lumber.walnut -= 3;
				player.furniture.walnut.side_table += amount;
				player.sawdust_prestige.furniture.walnut.side_table += amount;
				player.seed_prestige.furniture.walnut.side_table += amount;
			},
			"coffeetable" => {
				player.lumber.walnut -= 4;
				player.furniture.walnut.coffee_table += amount;
				player.sawdust_prestige.furniture.walnut.coffee_table += amount;
				player.seed_prestige.furniture.walnut.coffee_table += amount;
			},
			"diningset" => {
				player.lumber.walnut -= 5;
				player.furniture.walnut.dining_set += amount;
				player.sawdust_prestige.furniture.walnut.dining_set += amount;
				player.seed_prestige.furniture.walnut.dining_set += amount;
			},
			_ => (),
		},
		"cherry" => match furniture {
			"birdhouse" => {
				player.lumber.cherry -= 1;
				player.furniture.cherry.birdhouse += amount;
				player.sawdust_prestige.furniture.cherry.birdhouse += amount;
				player.seed_prestige.furniture.cherry.birdhouse += amount;
			},
			"shelf" => {
				player.lumber.cherry -= 2;
				player.furniture.cherry.shelf += amount;
				player.sawdust_prestige.furniture.cherry.shelf += amount;
				player.seed_prestige.furniture.cherry.shelf += amount;
			},
			"sidetable" => {
				player.lumber.cherry -= 3;
				player.furniture.cherry.side_table += amount;
				player.sawdust_prestige.furniture.cherry.side_table += amount;
				player.seed_prestige.furniture.cherry.side_table += amount;
			},
			"coffeetable" => {
				player.lumber.cherry -= 4;
				player.furniture.cherry.coffee_table += amount;
				player.sawdust_prestige.furniture.cherry.coffee_table += amount;
				player.seed_prestige.furniture.cherry.coffee_table += amount;
			},
			"diningset" => {
				player.lumber.cherry -= 5;
				player.furniture.cherry.dining_set += amount;
				player.sawdust_prestige.furniture.cherry.dining_set += amount;
				player.seed_prestige.furniture.cherry.dining_set += amount;
			},
			_ => (),
		},
		"purpleheart" => match furniture {
			"birdhouse" => {
				player.lumber.purpleheart -= 1;
				player.furniture.purpleheart.birdhouse += amount;
				player.sawdust_prestige.furniture.purpleheart.birdhouse += amount;
				player.seed_prestige.furniture.purpleheart.birdhouse += amount;
			},
			"shelf" => {
				player.lumber.purpleheart -= 2;
				player.furniture.purpleheart.shelf += amount;
				player.sawdust_prestige.furniture.purpleheart.shelf += amount;
				player.seed_prestige.furniture.purpleheart.shelf += amount;
			},
			"sidetable" => {
				player.lumber.purpleheart -= 3;
				player.furniture.purpleheart.side_table += amount;
				player.sawdust_prestige.furniture.purpleheart.side_table += amount;
				player.seed_prestige.furniture.purpleheart.side_table += amount;
			},
			"coffeetable" => {
				player.lumber.purpleheart -= 4;
				player.furniture.purpleheart.coffee_table += amount;
				player.sawdust_prestige.furniture.purpleheart.coffee_table += amount;
				player.seed_prestige.furniture.purpleheart.coffee_table += amount;
			},
			"diningset" => {
				player.lumber.purpleheart -= 5;
				player.furniture.purpleheart.dining_set += amount;
				player.sawdust_prestige.furniture.purpleheart.dining_set += amount;
				player.seed_prestige.furniture.purpleheart.dining_set += amount;
			},
			_ => (),
		},
		_ => ()
	}
}