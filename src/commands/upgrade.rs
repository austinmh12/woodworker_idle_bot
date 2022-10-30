use serenity::builder::{CreateApplicationCommand, CreateEmbed};
use serenity::model::prelude::command::{
	CommandOptionType
};
use serenity::model::prelude::interaction::application_command::{
	CommandDataOption,
	CommandDataOptionValue
};
use crate::player::get_player;
use crate::utils::{self, Message};

const SHARPENING_BOOKS_BASE: f64 = 100.0;
const SHARPENING_BOOKS_EXP: f64 = 1.01;
const GYM_PASS_BASE: f64 = 250.0;
const GYM_PASS_EXP: f64 = 1.03;
const SHARPER_AXES_BASE: f64 = 500.0;
const SHARPER_AXES_EXP: f64 = 1.025;
const WIDER_AXES_BASE: f64 = 500.0;
const WIDER_AXES_EXP: f64 = 1.05;
const THERMODYNAMICS_BASE: f64 = 1000.0;
const THERMODYNAMICS_EXP: f64 = 1.06;
const PULL_CARTS_BASE: f64 = 1300.0;
const PULL_CARTS_EXP: f64 = 1.08;
const HOTTER_KILNS_BASE: f64 = 2500.0;
const HOTTER_KILNS_EXP: f64 = 1.075;
const BETTER_TEMPERATURES_BASE: f64 = 2500.0;
const BETTER_TEMPERATURES_EXP: f64 = 1.1;
const FAST_DRYING_GLUE_BASE: f64 = 5000.0;
const FAST_DRYING_GLUE_EXP: f64 = 1.11;
const INDUSTRIAL_NAILS_BASE: f64 = 7200.0;
const INDUSTRIAL_NAILS_EXP: f64 = 1.12;
const WD_40_BASE: f64 = 10000.0;
const WD_40_EXP: f64 = 1.15;
const HIGH_QUALITY_BITS_BASE: f64 = 15000.0;
const HIGH_QUALITY_BITS_EXP: f64 = 1.2;

const TREE_FERTILIZER_BASE: f64 = 1.0;
const TREE_FERTILIZER_EXP: f64 = 1.01;
const LESS_BARK_BASE: f64 = 2.0;
const LESS_BARK_EXP: f64 = 1.03;
const DOUBLE_SWINGS_BASE: f64 = 4.0;
const DOUBLE_SWINGS_EXP: f64 = 1.025;
const DUAL_WIELDING_BASE: f64 = 5.0;
const DUAL_WIELDING_EXP: f64 = 1.05;
const PREHEATING_BASE: f64 = 10.0;
const PREHEATING_EXP: f64 = 1.06;
const EFFICIENT_PACKING_BASE: f64 = 12.0;
const EFFICIENT_PACKING_EXP: f64 = 1.08;
const ELECTRIC_HEATERS_BASE: f64 = 15.0;
const ELECTRIC_HEATERS_EXP: f64 = 1.075;
const READING_GLASSES_BASE: f64 = 20.0;
const READING_GLASSES_EXP: f64 = 1.1;
const LONGER_CLAMPS_BASE: f64 = 30.0;
const LONGER_CLAMPS_EXP: f64 = 1.11;
const SELF_TAPPING_SCREWS_BASE: f64 = 35.0;
const SELF_TAPPING_SCREWS_EXP: f64 = 1.12;
const SAVED_GCODE_BASE: f64 = 50.0;
const SAVED_GCODE_EXP: f64 = 1.15;
const STRONGER_MOTORS_BASE: f64 = 65.0;
const STRONGER_MOTORS_EXP: f64 = 1.2;
const DUST_COLLECTION_BASE: f64 = 100.0;
const DUST_COLLECTION_EXP: f64 = 1.3;
const FIRE_STARTER_BASE: f64 = 150.0;
const FIRE_STARTER_EXP: f64 = 1.35;
const MULTITASKING_BASE: f64 = 250.0;
const MULTITASKING_EXP: f64 = 1.4;
const ENDURANCE_TRAINING_BASE: f64 = 350.0;
const ENDURANCE_TRAINING_EXP: f64 = 1.5;

pub async fn run(player_id: u64, options: &[CommandDataOption]) -> Message {
	// store.view
	// store.buy.slot amount
	let action = &options
		.get(0)
		.expect("Expected a SubcommandGroup");
	let subaction = &action
		.options
		.get(0)
		.expect("Subcommand");
	let mut player = get_player(player_id).await;
	match action.name.as_str() {
		"normal" => match subaction.name.as_str() {
			"view" => {
				let mut ret = CreateEmbed::default();
				let mut desc = "Welcome to the normal upgrades! See something you like, buy it with **/upgrade normal buy**\n".to_string();
				desc.push_str(&format!("You have **${:.2}**\n\n", player.cash));
				desc.push_str(&format!("**1:** Sharpening Books | Decreases chop time - ${:.2}\n", utils::get_price(1, SHARPENING_BOOKS_BASE, SHARPENING_BOOKS_EXP, player.upgrades.sharpening_books)));
				desc.push_str(&format!("**2:** Gym Pass | Increases logs per chop - ${:.2}\n", utils::get_price(1, GYM_PASS_BASE, GYM_PASS_EXP, player.upgrades.gym_pass)));
				desc.push_str(&format!("**3:** Sharper Axes | Loggers chop faster - ${:.2}\n", utils::get_price(1, SHARPER_AXES_BASE, SHARPER_AXES_EXP, player.upgrades.sharper_axes)));
				desc.push_str(&format!("**4:** Wider Axes | Loggers chop more per cycle - ${:.2}\n", utils::get_price(1, WIDER_AXES_BASE, WIDER_AXES_EXP, player.upgrades.wider_axes)));
				desc.push_str(&format!("**5:** Thermodynamics | Decreases dry time - ${:.2}\n", utils::get_price(1, THERMODYNAMICS_BASE, THERMODYNAMICS_EXP, player.upgrades.thermodynamics)));
				desc.push_str(&format!("**6:** Pull Carts | Increases lumber per dry - ${:.2}\n", utils::get_price(1, PULL_CARTS_BASE, PULL_CARTS_EXP, player.upgrades.pull_carts)));
				desc.push_str(&format!("**7:** Hotter Kilns | Lumberers dry faster - ${:.2}\n", utils::get_price(1, HOTTER_KILNS_BASE, HOTTER_KILNS_EXP, player.upgrades.hotter_kilns)));
				desc.push_str(&format!("**8:** Better Temperatures | Lumberers dry more per cycle - ${:.2}\n", utils::get_price(1, BETTER_TEMPERATURES_BASE, BETTER_TEMPERATURES_EXP, player.upgrades.better_temperatures)));
				desc.push_str(&format!("**9:** Fast Drying Glue | Decrease time to build - ${:.2}\n", utils::get_price(1, FAST_DRYING_GLUE_BASE, FAST_DRYING_GLUE_EXP, player.upgrades.fast_drying_glue)));
				desc.push_str(&format!("**10:** Industrial Nails | Increase furniture per build - ${:.2}\n", utils::get_price(1, INDUSTRIAL_NAILS_BASE, INDUSTRIAL_NAILS_EXP, player.upgrades.industrial_nails)));
				desc.push_str(&format!("**11:** WD40 | CNCs build faster - ${:.2}\n", utils::get_price(1, WD_40_BASE, WD_40_EXP, player.upgrades.wd_40)));
				desc.push_str(&format!("**12:** High Quality Bits | CNCs build more per cycle - ${:.2}\n", utils::get_price(1, HIGH_QUALITY_BITS_BASE, HIGH_QUALITY_BITS_EXP, player.upgrades.high_quality_bits)));
				ret
					.title("Normal Upgrades")
					.description(&desc)
					.colour(utils::default_colour());

				Message::Embed(ret)
			},
			"buy" => {
				let slot = match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
					CommandDataOptionValue::Integer(i) => i.to_owned(),
					_ => 0,
				};
				if !(1..=12).contains(&slot) {
					return Message::Content("Invalid slot".to_string());
				}
				let amount = if &subaction.options.len() == &1usize {
					1
				} else {
					match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
						CommandDataOptionValue::Integer(i) => i.to_owned(),
						_ => 1
					}
				};
				let (count, total_price) = match slot {
					1 => utils::get_max_buyable_amount_and_price(&player, amount, SHARPENING_BOOKS_BASE, SHARPENING_BOOKS_EXP, player.upgrades.sharpening_books),
					2 => utils::get_max_buyable_amount_and_price(&player, amount, GYM_PASS_BASE, GYM_PASS_EXP, player.upgrades.gym_pass),
					3 => utils::get_max_buyable_amount_and_price(&player, amount, SHARPER_AXES_BASE, SHARPER_AXES_EXP, player.upgrades.sharper_axes),
					4 => utils::get_max_buyable_amount_and_price(&player, amount, WIDER_AXES_BASE, WIDER_AXES_EXP, player.upgrades.wider_axes),
					5 => utils::get_max_buyable_amount_and_price(&player, amount, THERMODYNAMICS_BASE, THERMODYNAMICS_EXP, player.upgrades.thermodynamics),
					6 => utils::get_max_buyable_amount_and_price(&player, amount, PULL_CARTS_BASE, PULL_CARTS_EXP, player.upgrades.pull_carts),
					7 => utils::get_max_buyable_amount_and_price(&player, amount, HOTTER_KILNS_BASE, HOTTER_KILNS_EXP, player.upgrades.hotter_kilns),
					8 => utils::get_max_buyable_amount_and_price(&player, amount, BETTER_TEMPERATURES_BASE, BETTER_TEMPERATURES_EXP, player.upgrades.better_temperatures),
					9 => utils::get_max_buyable_amount_and_price(&player, amount, FAST_DRYING_GLUE_BASE, FAST_DRYING_GLUE_EXP, player.upgrades.fast_drying_glue),
					10 => utils::get_max_buyable_amount_and_price(&player, amount, INDUSTRIAL_NAILS_BASE, INDUSTRIAL_NAILS_EXP, player.upgrades.industrial_nails),
					11 => utils::get_max_buyable_amount_and_price(&player, amount, WD_40_BASE, WD_40_EXP, player.upgrades.wd_40),
					12 => utils::get_max_buyable_amount_and_price(&player, amount, HIGH_QUALITY_BITS_BASE, HIGH_QUALITY_BITS_EXP, player.upgrades.high_quality_bits),
					_ => (0, 0.0) // Can't get here
				};
				if player.cash < total_price || count == 0 {
					return Message::Content(format!("You need **${:.2}** more to buy that", total_price - player.cash));
				}
				player.cash -= total_price;
				let ret = match slot {
					1 => {
						player.upgrades.sharpening_books += count;
	
						Message::Content(format!("You bought **{}** Sharpening Books!", count))
					},
					2 => {
						player.upgrades.gym_pass += count;
	
						Message::Content(format!("You bought **{}** Gym Pass!", count))
					},
					3 => {
						player.upgrades.sharper_axes += count;
	
						Message::Content(format!("You bought **{}** Sharper Axes!", count))
					},
					4 => {
						player.upgrades.wider_axes += count;
	
						Message::Content(format!("You bought **{}** Wider Axes!", count))
					},
					5 => {
						player.upgrades.thermodynamics += count;
	
						Message::Content(format!("You bought **{}** Thermodynamics!", count))
					},
					6 => {
						player.upgrades.pull_carts += count;
	
						Message::Content(format!("You bought **{}** Pull Carts!", count))
					},
					7 => {
						player.upgrades.hotter_kilns += count;
	
						Message::Content(format!("You bought **{}** Hotter Kilns!", count))
					},
					8 => {
						player.upgrades.better_temperatures += count;
	
						Message::Content(format!("You bought **{}** Better Temperatures!", count))
					},
					9 => {
						player.upgrades.fast_drying_glue += count;
	
						Message::Content(format!("You bought **{}** Fast Drying Glue!", count))
					},
					10 => {
						player.upgrades.industrial_nails += count;
	
						Message::Content(format!("You bought **{}** Industrial Nails!", count))
					},
					11 => {
						player.upgrades.wd_40 += count;
	
						Message::Content(format!("You bought **{}** WD40!", count))
					},
					12 => {
						player.upgrades.high_quality_bits += count;
	
						Message::Content(format!("You bought **{}** High Quality Bits!", count))
					},
					_ => Message::how()
				};
				player.update().await;
	
				ret
			},
			_ => Message::how()
		},
		"sawdust" => match subaction.name.as_str() {
			"view" => {
				let mut ret = CreateEmbed::default();
				let mut desc = "Welcome to the sawdust upgrades! See something you like, buy it with **/upgrade sawdust buy**\n".to_string();
				desc.push_str(&format!("You have **{}** sawdust\n\n", player.sawdust));
				desc.push_str(&format!("**1:** Tree Fertilizer | Decreases chop time - {}SD\n", utils::get_price(1, TREE_FERTILIZER_BASE, TREE_FERTILIZER_EXP, player.sawdust_upgrades.tree_fertilizer)));
				desc.push_str(&format!("**2:** Less Bark | Increases logs per chop - {}SD\n", utils::get_price(1, LESS_BARK_BASE, LESS_BARK_EXP, player.sawdust_upgrades.less_bark)));
				desc.push_str(&format!("**3:** Double Swings | Loggers chop faster - {}SD\n", utils::get_price(1, DOUBLE_SWINGS_BASE, DOUBLE_SWINGS_EXP, player.sawdust_upgrades.double_swings)));
				desc.push_str(&format!("**4:** Dual Wielding | Loggers chop more per cycle - {}SD\n", utils::get_price(1, DUAL_WIELDING_BASE, DUAL_WIELDING_EXP, player.sawdust_upgrades.dual_wielding)));
				desc.push_str(&format!("**5:** Preheating | Decreases dry time - {}SD\n", utils::get_price(1, PREHEATING_BASE, PREHEATING_EXP, player.sawdust_upgrades.preheating)));
				desc.push_str(&format!("**6:** Efficient Packing | Increases lumber per dry - {}SD\n", utils::get_price(1, EFFICIENT_PACKING_BASE, EFFICIENT_PACKING_EXP, player.sawdust_upgrades.efficient_packing)));
				desc.push_str(&format!("**7:** Electric Heaters | Lumberers dry faster - {}SD\n", utils::get_price(1, ELECTRIC_HEATERS_BASE, ELECTRIC_HEATERS_EXP, player.sawdust_upgrades.electric_heaters)));
				desc.push_str(&format!("**8:** Reading Glasses | Lumberers dry more per cycle - {}SD\n", utils::get_price(1, READING_GLASSES_BASE, READING_GLASSES_EXP, player.sawdust_upgrades.reading_glasses)));
				desc.push_str(&format!("**9:** Longer Clamps | Decrease time to build - {}SD\n", utils::get_price(1, LONGER_CLAMPS_BASE, LONGER_CLAMPS_EXP, player.sawdust_upgrades.longer_clamps)));
				desc.push_str(&format!("**10:** Self Tapping Screws | Increase furniture per build - {}SD\n", utils::get_price(1, SELF_TAPPING_SCREWS_BASE, SELF_TAPPING_SCREWS_EXP, player.sawdust_upgrades.self_tapping_screws)));
				desc.push_str(&format!("**11:** Saved GCode | CNCs build faster - {}SD\n", utils::get_price(1, SAVED_GCODE_BASE, SAVED_GCODE_EXP, player.sawdust_upgrades.saved_gcode)));
				desc.push_str(&format!("**12:** Stronger Motors | CNCs build more per cycle - {}SD\n", utils::get_price(1, STRONGER_MOTORS_BASE, STRONGER_MOTORS_EXP, player.sawdust_upgrades.stronger_motors)));
				desc.push_str(&format!("**13:** Dust Collection | Increase sawdust collection amount - {}SD\n", utils::get_price(1, DUST_COLLECTION_BASE, DUST_COLLECTION_EXP, player.sawdust_upgrades.dust_collection)));
				desc.push_str(&format!("**14:** Fire Starter | Increases sawdust bonus - {}SD\n", utils::get_price(1, FIRE_STARTER_BASE, FIRE_STARTER_EXP, player.sawdust_upgrades.fire_starter)));
				desc.push_str(&format!("**15:** Multitasking | Increase the max number of queued actions - {}SD\n", utils::get_price(1, MULTITASKING_BASE, MULTITASKING_EXP, player.sawdust_upgrades.multitasking)));
				desc.push_str(&format!("**16:** Endurance Training | Increase the max number of tasks per action - {}SD\n", utils::get_price(1, ENDURANCE_TRAINING_BASE, ENDURANCE_TRAINING_EXP, player.sawdust_upgrades.endurance_training)));
				ret
					.title("Sawdust Upgrades")
					.description(&desc)
					.colour(utils::default_colour());

				Message::Embed(ret)
			},
			"buy" => {
				let slot = match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
					CommandDataOptionValue::Integer(i) => i.to_owned(),
					_ => 0,
				};
				if !(1..=16).contains(&slot) {
					return Message::Content("Invalid slot".to_string());
				}
				let amount = if &subaction.options.len() == &1usize {
					1
				} else {
					match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
						CommandDataOptionValue::Integer(i) => i.to_owned(),
						_ => 1
					}
				};
				let (count, total_price) = match slot {
					1 => utils::get_max_buyable_amount_and_price(&player, amount, TREE_FERTILIZER_BASE, TREE_FERTILIZER_EXP, player.sawdust_upgrades.tree_fertilizer),
					2 => utils::get_max_buyable_amount_and_price(&player, amount, LESS_BARK_BASE, LESS_BARK_EXP, player.sawdust_upgrades.less_bark),
					3 => utils::get_max_buyable_amount_and_price(&player, amount, DOUBLE_SWINGS_BASE, DOUBLE_SWINGS_EXP, player.sawdust_upgrades.double_swings),
					4 => utils::get_max_buyable_amount_and_price(&player, amount, DUAL_WIELDING_BASE, DUAL_WIELDING_EXP, player.sawdust_upgrades.dual_wielding),
					5 => utils::get_max_buyable_amount_and_price(&player, amount, PREHEATING_BASE, PREHEATING_EXP, player.sawdust_upgrades.preheating),
					6 => utils::get_max_buyable_amount_and_price(&player, amount, EFFICIENT_PACKING_BASE, EFFICIENT_PACKING_EXP, player.sawdust_upgrades.efficient_packing),
					7 => utils::get_max_buyable_amount_and_price(&player, amount, ELECTRIC_HEATERS_BASE, ELECTRIC_HEATERS_EXP, player.sawdust_upgrades.electric_heaters),
					8 => utils::get_max_buyable_amount_and_price(&player, amount, READING_GLASSES_BASE, READING_GLASSES_EXP, player.sawdust_upgrades.reading_glasses),
					9 => utils::get_max_buyable_amount_and_price(&player, amount, LONGER_CLAMPS_BASE, LONGER_CLAMPS_EXP, player.sawdust_upgrades.longer_clamps),
					10 => utils::get_max_buyable_amount_and_price(&player, amount, SELF_TAPPING_SCREWS_BASE, SELF_TAPPING_SCREWS_EXP, player.sawdust_upgrades.self_tapping_screws),
					11 => utils::get_max_buyable_amount_and_price(&player, amount, SAVED_GCODE_BASE, SAVED_GCODE_EXP, player.sawdust_upgrades.saved_gcode),
					12 => utils::get_max_buyable_amount_and_price(&player, amount, STRONGER_MOTORS_BASE, STRONGER_MOTORS_EXP, player.sawdust_upgrades.stronger_motors),
					13 => utils::get_max_buyable_amount_and_price(&player, amount, DUST_COLLECTION_BASE, DUST_COLLECTION_EXP, player.sawdust_upgrades.dust_collection),
					14 => utils::get_max_buyable_amount_and_price(&player, amount, FIRE_STARTER_BASE, FIRE_STARTER_EXP, player.sawdust_upgrades.fire_starter),
					15 => utils::get_max_buyable_amount_and_price(&player, amount, MULTITASKING_BASE, MULTITASKING_EXP, player.sawdust_upgrades.multitasking),
					16 => utils::get_max_buyable_amount_and_price(&player, amount, ENDURANCE_TRAINING_BASE, ENDURANCE_TRAINING_EXP, player.sawdust_upgrades.endurance_training),
					_ => (0, 0.0) // Can't get here
				};
				if player.cash < total_price || count == 0 {
					return Message::Content(format!("You need **${:.2}** more to buy that", total_price - player.cash));
				}
				player.cash -= total_price;
				let ret = match slot {
					1 => {
						player.sawdust_upgrades.tree_fertilizer += count;
	
						Message::Content(format!("You bought **{}** Tree Fertilizer!", count))
					},
					2 => {
						player.sawdust_upgrades.less_bark += count;
	
						Message::Content(format!("You bought **{}** Less Bark!", count))
					},
					3 => {
						player.sawdust_upgrades.double_swings += count;
	
						Message::Content(format!("You bought **{}** Double Swings!", count))
					},
					4 => {
						player.sawdust_upgrades.dual_wielding += count;
	
						Message::Content(format!("You bought **{}** Dual Wielding!", count))
					},
					5 => {
						player.sawdust_upgrades.preheating += count;
	
						Message::Content(format!("You bought **{}** Preheating!", count))
					},
					6 => {
						player.sawdust_upgrades.efficient_packing += count;
	
						Message::Content(format!("You bought **{}** Efficient Packing!", count))
					},
					7 => {
						player.sawdust_upgrades.electric_heaters += count;
	
						Message::Content(format!("You bought **{}** Electric Heaters!", count))
					},
					8 => {
						player.sawdust_upgrades.reading_glasses += count;
	
						Message::Content(format!("You bought **{}** Reading Glasses!", count))
					},
					9 => {
						player.sawdust_upgrades.longer_clamps += count;
	
						Message::Content(format!("You bought **{}** Longer Clamps!", count))
					},
					10 => {
						player.sawdust_upgrades.self_tapping_screws += count;
	
						Message::Content(format!("You bought **{}** Self Tapping Screws!", count))
					},
					11 => {
						player.sawdust_upgrades.saved_gcode += count;
	
						Message::Content(format!("You bought **{}** Saved GCode!", count))
					},
					12 => {
						player.sawdust_upgrades.stronger_motors += count;
	
						Message::Content(format!("You bought **{}** Stronger Motors!", count))
					},
					13 => {
						player.sawdust_upgrades.dust_collection += count;
	
						Message::Content(format!("You bought **{}** Dust Collection!", count))
					},
					14 => {
						player.sawdust_upgrades.fire_starter += count;
	
						Message::Content(format!("You bought **{}** Fire Starter!", count))
					},
					15 => {
						player.sawdust_upgrades.multitasking += count;
	
						Message::Content(format!("You bought **{}** Multitasking!", count))
					},
					16 => {
						player.sawdust_upgrades.endurance_training += count;
	
						Message::Content(format!("You bought **{}** Endurance Training!", count))
					},
					_ => Message::how()
				};
				// Stats maybe?
				player.update().await;
	
				ret
			},
			_ => Message::how()
		},
		// "seed" => match subaction.name.as_str() {
		// 	"view" => {
		// 		Message::under_construction()
		// 	},
		// 	"buy" => {
		// 		let slot = match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
		// 			CommandDataOptionValue::Integer(i) => i.to_owned(),
		// 			_ => 0,
		// 		};
		// 		if !(1..=6).contains(&slot) {
		// 			return Message::Content("Invalid slot".to_string());
		// 		}
		// 		let amount = if &subaction.options.len() == &1usize {
		// 			1
		// 		} else {
		// 			match subaction.options.get(0).expect("expected int").resolved.as_ref().expect("int") {
		// 				CommandDataOptionValue::Integer(i) => i.to_owned(),
		// 				_ => 1
		// 			}
		// 		};

		// 		Message::under_construction()
		// 	},
		// 	_ => Message::how()
		// },
		_ => Message::how()
	}	
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
	command.name("upgrade").description("Buy upgrades to improve yourself!")
		.create_option(|option| {
			option
				.name("normal")
				.description("buy normal upgrades!")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("view")
						.description("view the normal upgrades")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("buy")
						.description("buy normal upgrades")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("slot")
								.description("slot to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.max_int_value(12)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("amount to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.required(false)
						})
				})
		})
		.create_option(|option| {
			option
				.name("sawdust")
				.description("buy sawdust upgrades!")
				.kind(CommandOptionType::SubCommandGroup)
				.create_sub_option(|sub| {
					sub
						.name("view")
						.description("view the sawdust upgrades")
						.kind(CommandOptionType::SubCommand)
				})
				.create_sub_option(|sub| {
					sub
						.name("buy")
						.description("buy sawdust upgrades")
						.kind(CommandOptionType::SubCommand)
						.create_sub_option(|subsub| {
							subsub
								.name("slot")
								.description("slot to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.max_int_value(9)
								.required(true)
						})
						.create_sub_option(|subsub| {
							subsub
								.name("amount")
								.description("amount to purchase")
								.kind(CommandOptionType::Integer)
								.min_int_value(1)
								.required(false)
						})
				})
		})
		// .create_option(|option| {
		// 	option
		// 		.name("seed")
		// 		.description("buy seed upgrades")
		// 		.kind(CommandOptionType::SubCommandGroup)
		// 		.create_sub_option(|sub| {
		// 			sub
		// 				.name("view")
		// 				.description("view the seed upgrades")
		// 				.kind(CommandOptionType::SubCommand)
		// 		})
		// 		.create_sub_option(|sub| {
		// 			sub
		// 				.name("buy")
		// 				.description("buy seed upgrades")
		// 				.kind(CommandOptionType::SubCommand)
		// 				.create_sub_option(|subsub| {
		// 					subsub
		// 						.name("slot")
		// 						.description("slot to purchase")
		// 						.kind(CommandOptionType::Integer)
		// 						.min_int_value(1)
		// 						.max_int_value(9)
		// 						.required(true)
		// 				})
		// 				.create_sub_option(|subsub| {
		// 					subsub
		// 						.name("amount")
		// 						.description("amount to purchase")
		// 						.kind(CommandOptionType::Integer)
		// 						.min_int_value(1)
		// 						.required(false)
		// 				})
		// 		})
		// })
}