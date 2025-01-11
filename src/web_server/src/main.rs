use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use sqlx::Connection;
use sqlx::Row;

use std::error::Error;

use axum_macros::debug_handler;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::services::ServeDir;

mod database;
use database::database::{get_database_credentials, DatabaseCredentials};

#[derive(Debug, Serialize)]
struct City {
    name: String,
    state: String,
}

#[tokio::main]
async fn main() {
    let routes_all = Router::new()
        .merge(Router::new().route("/get_cities", get(get_cities)))
        .merge(Router::new().route("/get_cities_from_db", get(get_cities_from_db)))
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

async fn get_cities_from_db() -> Result<(StatusCode, Json<Vec<City>>), (StatusCode, String)> {
    // Get the credentials for the database from a file on the system
    let credentials: DatabaseCredentials = match get_database_credentials() {
        Ok(credentials) => credentials,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from(format!("Failed to get database credentials {}.", e)),
            ))
        }
    };

    println!("{:?}", credentials);
    let url = format!(
        "postgres://{}:{}@{}:5432/weather_data",
        credentials.username, credentials.password, credentials.ip_address
    );
    let mut connection = match sqlx::postgres::PgConnection::connect(&url).await {
        Ok(connection) => connection,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                String::from(format!("Failed to connect to DB: {}", e)),
            ))
        }
    };

    let res = match sqlx::query("SELECT city, state FROM location")
        .fetch_one(&mut connection)
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
