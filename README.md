# Rust Backend Starter

A simple Rust Axum backend starter template with frontend demo showing health checks and ping-pong communication.

## File Structure

This template contains the files needed to deploy a Rust backend with frontend on Bunetic:

- **`src/main.rs`** - Axum HTTP server with health and ping endpoints
- **`Cargo.toml`** - Rust dependencies and project config (auto-detects Rust buildpack)
- **`Cargo.lock`** - Dependency lockfile for reproducible builds (required by buildpack)
- **`index.html`** - Frontend demo with health check and ping-pong buttons
- **`RustConfig`** - Rust version specification for buildpack (VERSION=1.80.0)
- **`.buildpacks`** - Explicitly specifies the Rust buildpack URL for reliable detection
- **`Procfile`** - Tells Bunetic how to start the Rust application (`web: ./target/release/bunetic-rust-backend`)

## Features

- **Health Check Endpoint** (`/health`) - Shows server status, uptime, and timestamp
- **Ping Endpoint** (`/ping`) - Returns "pong" with server info and timestamp
- **Frontend Demo** - Simple HTML interface to test both endpoints
- **Response Time Measurement** - Shows exact milliseconds for each request
- **CORS Enabled** - Ready for cross-origin requests
- **High Performance** - Compiled Rust binary for maximum speed
- **Async/Await** - Modern Rust async runtime with Tokio

## API Endpoints

### GET /health
Returns server health status:
```json
{
  "status": "healthy",
  "timestamp": "2025-09-29T12:00:00Z",
  "uptime": "5h30m15s"
}
```

### GET /ping
Returns pong response:
```json
{
  "message": "pong",
  "timestamp": "2025-09-29T12:00:00Z",
  "server_id": "rust-axum-v1"
}
```

## Why These Files Are Needed

- **Rust requires Cargo** - `Cargo.toml` and `Cargo.lock` auto-detect Rust buildpack
- **Reproducible builds** - `Cargo.lock` ensures consistent dependency versions  
- **Version specification** - `RustConfig` tells buildpack which Rust version to use (1.80.0+)
- **Buildpack detection** - `.buildpacks` ensures reliable Rust buildpack selection
- **Source code structure** - `src/main.rs` contains the Axum server implementation
- **Binary compilation** - Rust compiles to optimized native binary for deployment
- **Frontend included** - `index.html` demonstrates the backend functionality
- **Procfile defines startup** - Bunetic reads this to launch the compiled binary


