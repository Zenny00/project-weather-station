use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};

use axum_macros::debug_handler;
use serde::{de::Error, Deserialize, Serialize};
use serde_json::json;
use tower_http::services::ServeDir;

#[derive(Debug, Serialize)]
struct City {
    name: String,
    state: String,
}

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(Router::new().route("/get_cities", get(get_cities)))
        .merge(routes_weather())
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("->> LISTENING on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service("/pages", ServeDir::new("pages"))
}

fn routes_weather() -> Router {
    Router::new().route(
        "/weather",
        get(|| async { Html("This is the weather for today <strong>81 degrees </strong>") }),
    )
}

#[debug_handler]
async fn get_cities() -> Result<(StatusCode, Json<Vec<City>>), (StatusCode, String)> {
    let cities = vec![
        City {
            name: String::from("Silver Spring"),
            state: String::from("MD"),
        },
        City {
            name: String::from("Chicago"),
            state: String::from("IL"),
        },
    ];

    return Ok((StatusCode::OK, Json(cities)));
}
