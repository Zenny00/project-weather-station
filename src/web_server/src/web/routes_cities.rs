use crate::{Error, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct City {
    name: String,
    state: String,
}
