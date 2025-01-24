use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use sqlx::postgres::PgPool;
use sqlx::Row;
use sqlx::{Connection, Pool, Postgres};

use chrono::{serde::ts_seconds::serialize, DateTime, Utc};
use std::sync::Arc;

use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::{serde_as, TimestampSeconds};
use tower_http::services::ServeDir;

mod database;
use database::database::get_database_connection_pool;

#[derive(Debug, Serialize)]
struct Location {
    location_id: String,
    city: String,
    state: String,
}

#[derive(Debug, Serialize)]
struct Measurement {}

#[derive(Debug, Serialize)]
struct Station {
    station_id: String,
    location_id: String,
    name: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    start_date: DateTime<Utc>,
}

#[tokio::main]
async fn main() {
    let connection_pool = get_database_connection_pool().await.unwrap();

    let routes_all = Router::new()
        .route("/get_cities_from_db", get(get_cities_from_db))
        .with_state(connection_pool)
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    println!("->> LISTENING on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes_static() -> Router {
    Router::new().nest_service(
        "/pages",
        ServeDir::new(std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/src/pages"),
    )
}

#[debug_handler]
async fn get_cities_from_db(
    State(connection_pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Location>>), (StatusCode, String)> {
    // Grab the connection pool from state
    let connection_pool = connection_pool;

    let res = match sqlx::query("SELECT location_id, city, state FROM location")
        .fetch_one(&connection_pool)
        .await
    {
        Ok(result) => result,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from(format!("Failed to run query: {}", e)),
            ))
        }
    };

    let cities = vec![Location {
        location_id: res.get("location_id"),
        city: res.get("city"),
        state: res.get("state"),
    }];

    return Ok((StatusCode::OK, Json(cities)));
}

#[debug_handler]
async fn get_readings_from_station(
    Station(station): Station,
    State(connection_pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Measurement>>), (StatusCode, String)> {
    todo!()
}
