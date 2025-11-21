use axum::{extract::Query, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct WeatherQuery {
    pub city: String,
}

#[derive(Serialize)]
pub struct WeatherResponse {}

pub struct LatLong {
    pub lat: u32,
    pub lon: u32,
}

async fn fetch_weather(lat_long: LatLong) -> impl IntoResponse {}

async fn fetch_lat_long(city: &str) -> impl IntoResponse {}

pub async fn weather_handler(Query(params): Query<WeatherQuery>) -> impl IntoResponse {
    return "weather";
}
