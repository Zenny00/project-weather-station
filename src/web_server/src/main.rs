use axum::{
    extract::Query,
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Json, Router,
};
use sqlx::postgres::{PgPool, PgRow};
use sqlx::Row;
use sqlx::{Connection, Pool, Postgres};

use chrono::{serde::ts_seconds::serialize, DateTime, NaiveDateTime, Utc};
use std::{os::linux::raw::stat, sync::Arc};

use axum_macros::debug_handler;
use bigdecimal::{BigDecimal, ToPrimitive};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_with::{serde_as, TimestampSeconds};
use tower_http::services::ServeDir;

mod database;
use database::database::{
    get_database_connection_pool, Location, LocationQuery, Measurement, Station, StationQuery,
};

#[tokio::main]
async fn main() {
    let connection_pool = get_database_connection_pool().await.unwrap();

    let routes_all = Router::new()
        .route("/get_cities_from_db", get(get_cities_from_db))
        .route("/get_stations_from_db", get(get_stations_from_location))
        .route(
            "/get_measurements_from_station",
            get(get_measurements_from_station),
        )
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
async fn get_stations_from_location(
    Query(params): Query<LocationQuery>,
    State(connection_pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Station>>), (StatusCode, String)> {
    // Extract the location ID from the input params
    let location_id = params.location_id;

    // Grab the connection pool from state
    let connection_pool = connection_pool;

    // Run a query against the DB to get all stations at the given location
    let res = match sqlx::query(
        "SELECT station_id, location_id, description, start_date FROM station WHERE location_id = $1",
    )
    .bind(location_id)
    .fetch_all(&connection_pool)
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

    // Format the output
    let mut stations = Vec::new();
    for station in res.into_iter() {
        //        let date: NaiveDateTime = station.get("start_date");

        stations.push(Station {
            station_id: station.get("station_id"),
            location_id: station.get("location_id"),
            description: station.get("description"),
            start_date: station.get("start_date"),
        })
    }

    return Ok((StatusCode::OK, Json(stations)));
}

#[debug_handler]
async fn get_measurements_from_station(
    Query(params): Query<StationQuery>,
    State(connection_pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Measurement>>), (StatusCode, String)> {
    // Get the station ID passed in via params
    let station_id = params.station_id;

    // Grab the connection pool from state
    let connection_pool = connection_pool;

    // Run a query against the DB to get the measurements at the given location
    let res = match sqlx::query(
        "SELECT measurement_id, station_id, timestamp, temperature, humidity, precipitation, pressure, wind_speed, wind_direction, light_level, description FROM measurement WHERE station_id = $1",
    )
    .bind(station_id)
    .fetch_all(&connection_pool)
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

    // Format the output
    let mut measurements: Vec<Measurement> = Vec::new();
    for measurement in res.into_iter() {
        measurements.push(Measurement {
            measurement_id: measurement.get("measurement_id"),
            station_id: measurement.get("station_id"),
            timestamp: measurement.get("timestamp"),
            temperature: get_pg_value_as_float(&measurement, "temperature"),
            humidity: get_pg_value_as_float(&measurement, "humidity"),
            precipitation: get_pg_value_as_float(&measurement, "precipitation"),
            pressure: get_pg_value_as_float(&measurement, "pressure"),
            wind_speed: get_pg_value_as_float(&measurement, "wind_speed"),
            wind_direction: measurement.get("wind_direction"),
            light_level: measurement.get("light_level"),
            description: measurement.get("description"),
        });
    }

    return Ok((StatusCode::OK, Json(measurements)));
}

fn get_pg_value_as_float(row: &PgRow, column: &str) -> f64 {
    row.try_get::<BigDecimal, _>(column)
        .ok()
        .and_then(|v| v.to_f64())
        .unwrap_or_default()
}
