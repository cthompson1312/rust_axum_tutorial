use axum::Json;
use rand::seq::SliceRandom;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct BingoRequest {
    entries: Vec<String>,
}

#[derive(Serialize)]
pub struct Item {
    name: String,
}

#[derive(Serialize)]
pub struct BingoResponse {
    squares: Vec<Item>,
}

pub async fn bingo_handler(
    Json(payload): Json<BingoRequest>,
) -> Result<Json<BingoResponse>, StatusCode> {
    if payload.entries.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let mut entries = payload.entries;
    let mut rng = rand::rng();
    entries.shuffle(&mut rng);

    let squares = entries
        .iter()
        .map(|entry| Item {
            name: entry.to_string(),
        })
        .collect();

    return Ok(Json(BingoResponse { squares }));
}
