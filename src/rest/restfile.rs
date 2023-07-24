use std::fmt::Display;

use serde::Serialize;

use super::RestRequest;

#[derive(Debug, Clone, Serialize)]
pub struct RestFile {
    pub requests: Vec<RestRequest>,
}

impl Display for RestFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.requests.is_empty() {
            writeln!(f, "No requests found")?;
            return Ok(());
        }
        for (i, req) in self.requests.iter().enumerate() {
            write!(f, "### {}\n{}\n", i + 1, req)?;
        }
        Ok(())
    }
}
