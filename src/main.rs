use axum::{Json, Router, routing::get};
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub records: Vec<Record>,
    pub cors: Option<String>,
    pub loggin: Option<LogginLevel>,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: String,
    pub host: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethods {
    GET,
    POST,
    PUT,
    DELETE,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogginLevel {
    INFO,
    MINIMAL,
    TRACE,
    DEBUG,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub path: String,
    pub target: String,
    pub method: Vec<HttpMethods>,
    pub rewrite: Option<String>,
    pub remove_request_headers: Option<Vec<String>>,
    pub add_response_headers: Option<HashMap<String, String>>,
}

#[tokio::main]
async fn main() {
    let router = Router::new().route("/check", get(root));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

async fn root() -> Json<Value> {
    Json(json!({"status": "OK", "message": "Server working"}))
}
