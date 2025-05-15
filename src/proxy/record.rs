use std::{collections::HashMap, str::FromStr};

use axum::http::{HeaderMap, HeaderName, HeaderValue};
use reqwest::{Client, Method, Request, Url};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
pub struct Record {
    #[serde(deserialize_with = "target_from_str")]
    pub target: Url,
    #[serde(deserialize_with = "methods_from_strs")]
    pub methods: Vec<reqwest::Method>,
    pub rewrite: Option<String>,
    pub remove_request_headers: Option<Vec<String>>,
    pub add_response_headers: Option<HashMap<String, String>>,
}

fn target_from_str<'de, D>(deserializer: D) -> Result<Url, D::Error>
where
    D: Deserializer<'de>,
{
    let url_str = String::deserialize(deserializer)?;

    Url::from_str(&url_str).map_err(|_| {
        serde::de::Error::custom("Not a valid host/url\nCheck config `target` field again")
    })
}

impl Record {
    pub async fn get_request(
        &self,
        path: &str,
        method: reqwest::Method,
        incoming_header: &HeaderMap,
    ) -> Result<Request, Box<dyn std::error::Error>> {
        let rewrite = self.rewrite.as_deref().unwrap_or(path);
        let mut url = self.target.clone();
        url.set_path(&rewrite);

        let mut proxy_headers = incoming_header.clone();

        if let Some(to_remove) = &self.remove_request_headers {
            for key in to_remove {
                proxy_headers.remove(key);
            }
        }

        Ok(Client::new()
            .request(method, url)
            .headers(proxy_headers)
            .build()?)
    }
}

fn methods_from_strs<'de, D>(deserializer: D) -> Result<Vec<Method>, D::Error>
where
    D: Deserializer<'de>,
{
    let strs: Vec<String> = Vec::deserialize(deserializer)?;
    strs.into_iter()
        .map(|s| Method::from_str(&s).map_err(serde::de::Error::custom))
        .collect()
}
