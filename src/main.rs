use axum::extract::Path;
use axum::response::{IntoResponse, Response, Json};
use axum::http::{Uri, StatusCode};
use axum::{routing::get, routing::post, Router};
use serde::{Deserialize, Serialize};
// use serde_json::Result;
// Day 1
// async fn exclusive_cube(
//     Path((num1, num2)): Path<(i32, i32)>,
// ) -> Result<Json<i32>, StatusCode> {

//     let xor_result = num1 ^ num2;
//     let pow_result = xor_result.pow(3);
//     Ok(Json(pow_result))
// }

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
    Err(_) => {
            return Err(StatusCode::BAD_REQUEST)
        }
    }
        
}

async fn unknown() -> Response {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        "Something went wrong...",
    ).into_response()
}

async fn handle(uri: Uri) -> Response {
    if uri.path() == "/" {
        "Hello, world!".into_response()
    } else {
        format!("This part from {}", uri.path()).into_response()
    }
}

// Day 4
#[derive(Serialize, Deserialize, Debug)]
struct Reindeer {
    name: String,
    strength: u32,
}

async fn day_four(data: Json<T>) -> Response {
    let R: Person = serde_json::from_str(data)?;
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
