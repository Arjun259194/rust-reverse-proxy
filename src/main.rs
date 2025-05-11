use axum::{
    Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    routing::any,
};
use serde::Deserialize;
use std::{collections::HashMap, env, fs::File, io::BufReader, process, sync::Arc};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub server: Server,
    pub records: Vec<Record>,
}

impl Config {
    pub fn new(path: Option<&str>) -> Config {
        let path = match path {
            None => "config.yaml",
            Some(path) => path,
        };
        let file = match File::open(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!(
                    "Can't open the config.yaml file\nCheck if the file exits or not\n\nIf file exists move it to the config directory or pass the path to config file as argument."
                );
                eprintln!("{err}");
                process::exit(1)
            }
        };
        let reader = BufReader::new(file);

        match serde_yaml::from_reader::<_, Config>(reader) {
            Err(err) => {
                eprintln!(
                    "Can't parse the config.yaml file\nCheck for syntax error and double check the content of config file"
                );
                eprintln!("{err}");
                process::exit(1);
            }
            Ok(config) => config,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub port: String,
    pub host: String,
    pub cors: Option<String>,
    pub logging: Option<LogginLevel>,
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
    TRACE,
    DEBUG,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    pub path: String,
    pub target: String,
    pub methods: Vec<HttpMethods>,
    pub rewrite: Option<String>,
    pub remove_request_headers: Option<Vec<String>>,
    pub add_response_headers: Option<HashMap<String, String>>,
}

#[tokio::main]
async fn main() {
    let arguments: Vec<String> = env::args().collect();
    let path: Option<&str> = if arguments.len() > 1 {
        Some(&arguments[1])
    } else {
        None
    };
    let config = Arc::new(Config::new(path));

    let router = Router::new()
        .route("/{*rest}", any(handler))
        .with_state(config.clone());

    let addr = format!("{0}:{1}", config.server.host, config.server.port);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    println!("Proxy running on {0}", &addr);
    axum::serve(listener, router).await.unwrap();
}

async fn handler(State(config): State<Arc<Config>>, Path(rest): Path<String>) -> impl IntoResponse {
    for record in config.records.iter() {
        if format!("/{rest}") == record.path {
            return (StatusCode::OK, HeaderMap::new(), format!("Redirecting...")).into_response();
        }
    }
    (
        StatusCode::NOT_FOUND,
        HeaderMap::new(),
        format!("You hit path: {}\n\nconfig:{:?}", rest, config),
    )
        .into_response()
}
