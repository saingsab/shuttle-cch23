use axum::extract::Path;
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Json, Response};
use axum::{routing::get, routing::post, Router};
use serde::{Deserialize, Serialize};

// Day Bonus 1
async fn day1(Path(ids): Path<String>) -> Result<Json<i32>, StatusCode> {
    // Parse the path into a vector of u32
    let parsed_ids: Result<Vec<i32>, _> = ids
        .split('/')
        .filter(|s| !s.is_empty())
        .map(|s| s.parse())
        .collect();

    match parsed_ids {
        Ok(ids) => {
            // Perform the XOR and power of 3 operations
            let result = ids.iter().fold(0, |acc, &id| acc ^ id).pow(3);

            Ok(Json(result))
        }
        Err(_) => return Err(StatusCode::BAD_REQUEST),
    }
}

async fn unknown() -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong...").into_response()
}

async fn handle(uri: Uri) -> Response {
    if uri.path() == "/" {
        "Hello, world!".into_response()
    } else {
        format!("This part from {}", uri.path()).into_response()
    }
}

// Day 4
#[derive(Deserialize)]
struct Reindeer {
    name: String,
    strength: i32,
}

async fn day_four(data: Json<Vec<Reindeer>>) -> Result<Json<i32>, StatusCode> {
    let mut result = 0;
    for i in data.iter() {
        result += i.strength;
    }

    Ok(Json(result))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/4/strength", post(day_four))
        .route("/1/*ids", get(day1))
        .route("/", get(handle))
        .route("/-1/error", get(unknown));

    Ok(router.into())
}
