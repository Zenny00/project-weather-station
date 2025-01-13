pub mod database {
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
    pub async fn get_database_connection_pool() -> Result<sqlx::postgres::PgPool, String> {
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
}
