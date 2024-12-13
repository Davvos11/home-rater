mod errors;

use std::env;
use dotenv::dotenv;
use google_maps::directions::response::status::Status;
use google_maps::GoogleMapsClient;
use google_maps::prelude::{Location, TravelMode};
use crate::errors::DirectionsError;

#[tokio::main]
async fn main() -> Result<(), DirectionsError> {
    dotenv().expect("Could not load .env file");
    let api_key = env::var("MAPS_API_KEY")
        .expect("env variable MAPS_API_KEY not set");

    let maps_client = GoogleMapsClient::try_new(api_key)
        .expect("Could not create Google Maps Client");

    let walk = get_length_minutes(&maps_client, TravelMode::Walking, "Kerkstraat 44, Leerdam")
        .await?;
    let cycle = get_length_minutes(&maps_client, TravelMode::Bicycling, "Kerkstraat 44, Leerdam")
        .await?;

    println!("Walk {walk} minutes, cycle {cycle} minutes");

    Ok(())
}

async fn get_length_minutes(maps_client: &GoogleMapsClient, travel_mode: TravelMode, from: &str) -> Result<i64, DirectionsError> {
    let directions = maps_client.directions(
        Location::from_address(from),
        Location::from_address("Café de Rat, Utrecht"),
    )
        .with_travel_mode(travel_mode)
        .execute().await?
        ;

    if directions.status != Status::Ok {
        return Err(
            DirectionsError::MapsNotOk(directions.status, directions.error_message.unwrap_or("".to_string()))
        );
    }

    directions.routes.iter()
        .map(|r| r.legs.iter().map(|l| l.duration.value.num_minutes()).sum())
        .min()
        .ok_or(DirectionsError::NoRoute)
}
