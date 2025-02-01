pub mod database {
    use bigdecimal::{BigDecimal, ToPrimitive};
    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use sqlx::{
        postgres::{PgPool, PgRow},
        Row,
    };

    ///
    /// A struct representing the schema of the Location table
    /// in the postgres database
    ///
    /// location_id: A unique identifier for the location
    /// city: The city in which this location resides
    /// state: The US state in which this location resides
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Location {
        pub location_id: String,
        pub city: String,
        pub state: String,
    }

    ///
    /// A struct representing the schema of the measurement table in the
    /// postgres database
    ///
    /// measurement_id: A unique indentifier for the measurement
    /// station_id: A unique identifier for the station this measurement was taken at
    /// timestamp: The time at which this measurement was taken (UTC)
    /// temperature: The temperature present during the reading (degrees C)
    /// humidity: The relative humidty present during the reading (%)
    /// precipitation: The precipitation amount recorded since the last reading
    /// pressure: The present pressure when the reading was taken
    /// wind_speed: The speed at which the wind was traveling when the measurement was taken (m/s)
    /// wind_direction: The direction in which the wind was blowing when the measurement was taken
    /// (degrees)
    /// light_level: The present light level at the time of the reading (TBD)
    /// description: A plain text explaination of the conditions
    #[derive(Debug, Serialize)]
    pub struct Measurement {
        pub measurement_id: String,
        pub station_id: String,
        #[serde(with = "chrono::serde::ts_seconds")]
        pub timestamp: DateTime<Utc>,
        pub temperature: f64,
        pub humidity: f64,
        pub precipitation: f64,
        pub pressure: f64,
        pub wind_speed: f64,
        pub wind_direction: f64,
        pub light_level: f64,
        pub description: String,
    }

    ///
    /// A struct representing the schema of the Station table in the postgres database
    ///
    /// station_id: A unique identifier representing the station
    /// location_id: A unique identifier representing the location this station was present at
    /// description: A description of the station in plain text
    /// start_date: The time the station began recording data
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Station {
        pub station_id: String,
        pub location_id: String,
        pub description: String,
        #[serde(with = "chrono::serde::ts_seconds")]
        pub start_date: DateTime<Utc>,
    }

    ///
    /// A struct that can be used to make a param query based on location_id
    ///
    /// location_id: A unique identifier for the location
    #[derive(Debug, Serialize, Deserialize)]
    pub struct LocationQuery {
        pub location_id: String,
    }

    ///
    /// A struct that can be used to make a param query based on a station_id
    ///
    /// station_id: A unique identifier for the station
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StationQuery {
        pub station_id: String,
    }

    /// A stuct for holding information used to connect to the database server. Has the following
    /// stucture
    /// {
    ///     username: String,
    ///     password: String,
    ///     ip_address: String,
    /// }
    #[derive(Debug)]
    pub struct DatabaseCredentials {
        pub username: String,
        pub password: String,
        pub ip_address: String,
    }

    /// Returns the values stored inside the database credentials file as a DatabaseCredentials
    /// struct of the format
    /// {
    ///     username: String,
    ///     password: String,
    ///     ip_address: String,
    /// }
    ///
    pub fn get_database_credentials() -> Result<DatabaseCredentials, String> {
        // Read in the credential file from the expected location and store in a vec of strings
        let db_credentials: Vec<String> = std::fs::read_to_string(
            std::env::var("CARGO_MANIFEST_DIR").unwrap() + "/db_credentials",
        )
        .expect("Could not read credential file")
        .lines()
        .map(|line| line.to_string())
        .collect();

        // Return an error to the user if an invalid number of values are present in the input file
        if db_credentials.len() != 3 {
            return Err(String::from("Invalid number of values in credential file"));
        }

        // Return the result in the form of a database credentials object
        return Ok(DatabaseCredentials {
            username: db_credentials[0].clone(),
            password: db_credentials[1].clone(),
            ip_address: db_credentials[2].clone(),
        });
    }

    /// Returns a pool of database connections that can be used to make queries in the application
    /// without starting new connections.
    pub async fn get_database_connection_pool() -> Result<PgPool, String> {
        // Get the credentials for the database from a file on the system
        let credentials: DatabaseCredentials = match get_database_credentials() {
            Ok(credentials) => credentials,
            Err(e) => return Err(String::from("Failed to get database credentials")),
        };

        let url = format!(
            "postgres://{}:{}@{}:5432/weather_data",
            credentials.username, credentials.password, credentials.ip_address
        );

        let pool = match sqlx::postgres::PgPool::connect(&url).await {
            Ok(pool) => pool,
            Err(e) => return Err(String::from("Failed to create pool.")),
        };

        return Ok(pool);
    }

    ///
    /// Takes in a PgRow and string and returns the Numeric value present in the specified row
    /// column as a f64 data type
    pub fn get_pg_value_as_float(row: &PgRow, column: &str) -> f64 {
        row.try_get::<BigDecimal, _>(column)
            .ok()
            .and_then(|v| v.to_f64())
            .unwrap_or_default()
    }
}
