mod proxy;
use crate::proxy::{Config, HandlerErr};

use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::any,
};
use reqwest::{self, Client, Method, header::CONTENT_TYPE};
use serde_json::json;
use std::{env, str::FromStr, sync::Arc};

#[derive(Debug, Clone)]
struct AppState {
    config: Config,
    client: Client,
}

impl AppState {
    fn new() -> AppState {
        let arguments: Vec<String> = env::args().collect();
        let path: Option<&str> = if arguments.len() > 1 {
            Some(&arguments[1])
        } else {
            None
        };
        let config = Config::new(path);
        AppState {
            config,
            client: Client::new(),
        }
    }
}

#[tokio::main]
async fn main() {
    // v1 ready, test it and check it then commit it
    let state = Arc::new(AppState::new());
    let router = Router::new()
        .route("/{*path}", any(handler))
        .with_state(state.clone());

    let addr = format!(
        "{0}:{1}",
        state.config.server.host, state.config.server.port
    );
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Proxy running on {0}", &addr);
    axum::serve(listener, router).await.unwrap();
}

fn format_path(path: &str) -> String {
    if path.ends_with("/") {
        let mut x = format!("/{path}");
        let _ = &x.pop().unwrap();
        x
    } else {
        format!("/{path}")
    }
}

async fn handler(
    method: Method,
    Path(path): Path<String>,
    header: HeaderMap,
    State(state): State<Arc<AppState>>,
) -> Result<Response, HandlerErr> {
    let fmt_path = format_path(&path);

    let Some(record) = state.config.records.get(&fmt_path) else {
        eprintln!("Can't find a record");
        return Err(HandlerErr::NOTFOUND);
    };

    if !record.methods.contains(&method) {
        eprintln!("Method not supporting incoming method!");
        return Err(HandlerErr::BADREQUEST("Method not supported".to_string()));
    }

    let request = match record.get_request(&fmt_path, method, &header).await {
        Ok(r) => r,
        Err(err) => {
            eprintln!("Error while building proxy request: {:?}", err);
            return Err(HandlerErr::INTERNALERROR(
                "Error while building proxy request".to_string(),
            ));
        }
    };

    let proxy_response = match state.client.execute(request).await {
        Ok(response) => response,
        Err(err) => {
            eprintln!("{:?}", err);
            return Err(HandlerErr::INTERNALERROR(err.to_string()));
        }
    };

    let status = proxy_response.status();
    let mut proxy_res_headers = proxy_response.headers().clone();
    let bytes = match proxy_response.bytes().await {
        Ok(b) => b,
        Err(err) => {
            eprintln!("{err}");
            return Err(HandlerErr::INTERNALERROR(err.to_string()));
        }
    };

    if let Some(add_on) = &record.add_response_headers {
        for (key, val) in add_on {
            if let (Ok(key), Ok(val)) = (HeaderName::from_str(key), HeaderValue::from_str(val)) {
                proxy_res_headers.insert(key, val);
            }
        }
    }

    let mut response = Response::builder()
        .status(status)
        .body(Body::from(bytes))
        .unwrap();

    *response.headers_mut() = proxy_res_headers;

    Ok(response)
}
