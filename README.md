# Rust Reverse Proxy

A lightweight reverse proxy server built with [Axum](https://docs.rs/axum) and [Tokio](https://tokio.rs/), designed to forward HTTP requests to upstream services based on configurable routing rules.

## 🚀 Features

* 🔁 Reverse proxy routing
* ⚙️ Configurable via `config.yaml`
* 🧪 Built with async Rust (Axum + Tokio)
* 🐳 Docker support via `compose.yaml`

## 🛠️ Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) (latest stable)
* [Docker](https://www.docker.com/) (optional, for containerized deployment)

### Clone the Repository

```bash
git clone https://github.com/Arjun259194/rust-reverse-proxy.git
cd rust-reverse-proxy
```

### Build and Run

```bash
cargo build --release
./target/release/rust-reverse-proxy
```

### Run with Docker Compose

```bash
docker-compose up --build
```

## ⚙️ Configuration

The proxy is configured using a `config.yaml` file located at the root of the project. Below is an example configuration:

```yaml
server:
  host: "127.0.0.1"
  port: 8080
  cors: "*"
  logging: "INFO"

records:
  "/users":
     target: "http://localhost:8081"
     methods: 
      - "GET"
     rewrite: "/user"
     remove_request_headers: ["x-secret", "user-agent"]
     add_response_headers:
       x-powered-by: "RustGateway"

  "/chat":
      target: "http://localhost:8082"
      rewrite: "/chats"
      methods:
        - "POST"
```

Each route specifies a `path` to match incoming requests and an `upstream` URL to which the requests will be forwarded.

## 📂 Project Structure

```
src
├── main.rs
└── proxy
    ├── config.rs
    ├── error.rs
    ├── mod.rs
    └── record.rs
```

* `src/`: Contains the Rust source code.
* `config.yaml`: Defines routing rules for the proxy.
* `compose.yaml`: Docker Compose configuration for containerized deployment.
* `Cargo.toml`: Rust project manifest.

## 🧪 Example Usage

Assuming you have services running on `localhost:3001` and `localhost:3002`, and your `config.yaml` is set up as shown above:

* Requests to `http://localhost:8080/api` will be proxied to `http://localhost:3001`.
* Requests to `http://localhost:8080/auth` will be proxied to `http://localhost:3002`.

## 📄 License

This project is licensed under the MIT License.
