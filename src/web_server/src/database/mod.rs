pub mod database {
    use std::fs;

    #[derive(Debug)]
    pub struct DatabaseCredentials {
        pub username: String,
        pub password: String,
        pub ip_address: String,
    }

    pub fn get_database_credentials() -> Result<DatabaseCredentials, String> {
        // Read in the credential file from the expected location and store in a vec of strings
        let db_credentials: Vec<String> = fs::read_to_string("../db_credentials")
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
}
