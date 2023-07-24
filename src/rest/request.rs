use std::{collections::HashMap, fmt::Display, time::SystemTime};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{RestMethod, RestResponse};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RestRequest {
    pub method: RestMethod,
    pub url: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl RestRequest {
    pub async fn get_response(&self) -> Result<RestResponse, String> {
        let mut client = match self.method {
            RestMethod::Get => reqwest::Client::new().get(&self.url),
            RestMethod::Post => reqwest::Client::new().post(&self.url),
            RestMethod::Put => reqwest::Client::new().put(&self.url),
            RestMethod::Delete => reqwest::Client::new().delete(&self.url),
        };

        // build self headers
        for (key, val) in &self.headers {
            client = client.header(key, val);
        }

        // build self headers
        for (key, val) in &self.headers {
            client = client.header(key, val);
        }

        // execute request
        let now = SystemTime::now();
        let res = client
            .body(self.body.clone())
            .send()
            .await
            .map_err(|e| e.to_string())?;
        let elapsed_ms = now.elapsed().map_err(|e| e.to_string())?.as_millis();

        // collect response headers
        let mut headers = HashMap::<String, String>::new();
        for (key, val) in res.headers().iter() {
            headers.insert(
                key.to_string(),
                val.to_str().map_err(|e| e.to_string())?.to_string(),
            );
        }

        // create response
        let response = RestResponse {
            elapsed_ms,
            headers,
            status: res.status().to_string(),
            version: format!("{:?}", res.version()),
            body: res.text().await.map_err(|e| e.to_string()).and_then(|s| {
                tracing::info!("got response: {}", &s);
                let val: Value = serde_json::from_str(&s).unwrap_or(Value::String(s.clone()));
                let s = serde_json::to_string_pretty(&val).unwrap_or(s);
                Ok(s)
            })?,
        };
        Ok(response)
    }
}

impl Display for RestRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} HTTP/{}\n", self.method, self.url, self.version)?;
        if !self.headers.is_empty() {
            for (k, v) in self.headers.iter() {
                write!(f, "{}: {}\n", k, v)?;
            }
        }
        if !self.body.is_empty() {
            write!(f, "\n{}\n", &self.body)?;
        }
        Ok(())
    }
}
