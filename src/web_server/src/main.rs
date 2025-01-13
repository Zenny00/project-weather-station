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

use std::sync::Arc;

use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::services::ServeDir;

mod database;
use database::database::get_database_connection_pool;

#[derive(Debug, Serialize)]
struct City {
    name: String,
    state: String,
}

#[tokio::main]
async fn main() {
    let connection_pool = get_database_connection_pool().await.unwrap();

    let routes_all = Router::new()
        .route("/get_cities", get(get_cities))
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

#[debug_handler]
async fn get_cities_from_db(
    State(connection_pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<City>>), (StatusCode, String)> {
    // Grab the connection pool from state
    let connection_pool = connection_pool;

    let res = match sqlx::query("SELECT city, state FROM location")
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

    let cities = vec![City {
        name: res.get("city"),
        state: res.get("state"),
    }];

    return Ok((StatusCode::OK, Json(cities)));
}
