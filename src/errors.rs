use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
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

impl IntoResponse for DirectionsError {
    fn into_response(self) -> Response {
        let code = match self {
            DirectionsError::NoRoute |
            DirectionsError::MapsNotOk(_, _) => { StatusCode::BAD_REQUEST }
            DirectionsError::MapsError(_) => { StatusCode::INTERNAL_SERVER_ERROR }
        };
        (code, self.to_string()).into_response()
    }
}
