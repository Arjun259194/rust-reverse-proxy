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
routes:
  - path: /api
    upstream: http://localhost:3001
  - path: /auth
    upstream: http://localhost:3002
```

Each route specifies a `path` to match incoming requests and an `upstream` URL to which the requests will be forwarded.

## 📂 Project Structure

```
rust-reverse-proxy/
├── src/
│   ├── main.rs
│   └── ...
├── config.yaml
├── compose.yaml
├── Cargo.toml
└── ...
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
