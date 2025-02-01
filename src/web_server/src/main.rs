use axum::{http::StatusCode, routing::get, Json, Router};

mod database;
use database::database::{
    get_database_connection_pool, get_pg_value_as_float, Location, LocationQuery, Measurement,
    Station, StationQuery,
};

mod server;
use server::server::{
    get_cities_from_db, get_measurements_from_station, get_stations_from_location, routes_static,
};

#[tokio::main]
async fn main() {
    let connection_pool = get_database_connection_pool().await.unwrap();

    // Setup routes with handler functions
    let routes_all = Router::new()
        .route("/get_cities_from_db", get(get_cities_from_db))
        .route("/get_stations_from_db", get(get_stations_from_location))
        .route(
            "/get_measurements_from_station",
            get(get_measurements_from_station),
        )
        .with_state(connection_pool)
        .fallback_service(routes_static());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("->> LISTENING on {}\n", listener.local_addr().unwrap());
    axum::serve(listener, routes_all).await.unwrap();
}
