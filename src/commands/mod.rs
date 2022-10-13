use mongodb::{
	Client,
	options::{
		ClientOptions,
	},
};
use std::error::Error;

pub mod ping;
pub mod chop;

async fn get_client() -> Result<Client, Box<dyn Error>> {
	let mon_client_uri = dotenv::var("MONGODB_URI").expect("No mongodb uri");
	let options = ClientOptions::parse(&mon_client_uri).await?;
	let client = Client::with_options(options)?;
	
	Ok(client)
}