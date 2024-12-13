use std::env;
use dotenv::dotenv;
use google_maps::directions::response::status::Status;
use google_maps::GoogleMapsClient;
use google_maps::prelude::{Location, TravelMode};

#[tokio::main]
async fn main() {
    dotenv().expect("Could not load .env file");
    let api_key = env::var("MAPS_API_KEY")
        .expect("env variable MAPS_API_KEY not set");

    let maps_client = GoogleMapsClient::try_new(api_key)
        .expect("Could not create Google Maps Client");

    let walk = get_length_minutes(&maps_client, TravelMode::Walking, "Kerkstraat 44, Leerdam").await;
    let cycle = get_length_minutes(&maps_client, TravelMode::Bicycling, "Kerkstraat 44, Leerdam").await;

    println!("Walk {walk} minutes, cycle {cycle} minutes");
}

async fn get_length_minutes(maps_client: &GoogleMapsClient, travel_mode: TravelMode, from: &str) -> i64 {
    let directions = maps_client.directions(
        Location::from_address(from),
        Location::from_address("Café de Rat, Utrecht"),
    )
        .with_travel_mode(travel_mode)
        .execute().await
        .expect("Could not get directions")
        ;

    if directions.status != Status::Ok {
        panic!("Could not get directions, status: {:?}, error: {}",
               directions.status,
               directions.error_message.unwrap_or("".to_string()));
    }
    
    directions.routes.iter()
        .map(|r| r.legs.iter().map(|l|l.duration.value.num_minutes()).sum())
        .min()
        .expect("Could not get directions")
}
