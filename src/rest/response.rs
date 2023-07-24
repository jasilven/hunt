use std::{collections::HashMap, fmt::Display};

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct RestResponse {
    pub elapsed_ms: u128,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub status: String,
    pub version: String,
}

impl Display for RestResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("moikka")
    }
}
