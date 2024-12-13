use google_maps::directions::response::status::Status;
use google_maps::GoogleMapsError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DirectionsError {
    #[error("No route found")]
    NoRoute,
    
    #[error("Failed to get directions: {0}")]
    MapsError(#[from] GoogleMapsError),

    #[error("Error from Google Maps API. Status: {0:?}. Error: {1}")]
    MapsNotOk(Status, String),
}
