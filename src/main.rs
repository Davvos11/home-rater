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
use axum::http::Method;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let _ = dotenv();
    let _ = env::var("MAPS_API_KEY")
        .expect("env variable MAPS_API_KEY not set");

    let app = Router::new()
        .route("/", get(get_lengths))
        .layer(CorsLayer::new()
            .allow_origin(vec!["https://dovatvis.nl".parse().unwrap(), "http://0.0.0.0:8080".parse().unwrap()])
            .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        )
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    eprintln!("Listening on: http://{}", &listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct QueryParams {
    from: String,
}

#[derive(Serialize)]
struct Response {
    walk: i64,
    cycle: i64,
}

async fn get_lengths(Query(params): Query<QueryParams>) -> Result<Json<Response>, DirectionsError> {
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
