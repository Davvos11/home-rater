mod errors;

use crate::errors::DirectionsError;
use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use dotenv::dotenv;
use google_maps::directions::response::status::Status;
use google_maps::prelude::{Location, TravelMode};
use google_maps::GoogleMapsClient;
use serde::{Deserialize, Serialize};
use std::env;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(get_lengths));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    eprintln!("Listening on: http://{}", &listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct QueryParams {
    from: String
}

#[derive(Serialize)]
struct Response {
    walk: i64,
    cycle: i64,
}

async fn get_lengths(Query(params): Query<QueryParams>) -> Result<Json<Response>, DirectionsError> {
    dotenv().expect("Could not load .env file");
    let api_key = env::var("MAPS_API_KEY")
        .expect("env variable MAPS_API_KEY not set");

    let maps_client = GoogleMapsClient::try_new(api_key)
        .expect("Could not create Google Maps Client");

    let from = params.from;

    let walk = get_length_minutes(&maps_client, TravelMode::Walking, &from)
        .await?;
    let cycle = get_length_minutes(&maps_client, TravelMode::Bicycling, &from)
        .await?;

    Ok(Json(Response { walk, cycle }))
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
