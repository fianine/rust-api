# Rust Instagram API

This project is a simple Instagram-style API built with [Axum](https://github.com/tokio-rs/axum) and Tokio in Rust. It exposes health and resource endpoints and is designed as a starter backend service.

## Prerequisites

- Rust toolchain (stable) and Cargo installed
  - If you do not have Rust installed, install it using `rustup` from the official Rust website.

## Getting Started

### 1. Clone the repository

```bash path=null start=null
git clone <your-repo-url>
cd rust-api
```

### 2. Build the project

```bash path=null start=null
cargo build
```

### 3. Run the application

The application reads the port from the `PORT` environment variable. If it is not set, it defaults to `3000`.

Run in debug mode:

```bash path=null start=null
# Optional: set a custom port
# On PowerShell
$env:PORT = 4000

# On bash
# export PORT=4000

# Start the server
cargo run
```

Run in release mode:

```bash path=null start=null
cargo run --release
```

When the server starts, you should see output similar to:

```text path=null start=null
Starting server on 0.0.0.0:3000
```

### 4. Verify the server is running

Call the health check endpoint from your browser or using a tool like `curl` or `Invoke-WebRequest`:

- URL: `http://localhost:3000/health`

Example using `curl`:

```bash path=null start=null
curl http://localhost:3000/health
```

Example using PowerShell:

```powershell path=null start=null
Invoke-WebRequest http://localhost:3000/health | Select-Object -ExpandProperty Content
```

You should receive:

```text path=null start=null
OK
```

## Project Structure

- `src/main.rs` – application entry point, router setup, and HTTP server
- `src/config.rs` – application configuration (currently, the HTTP port)
- `src/state.rs` – shared application state
- `src/routes/` – route handlers grouped by feature (auth, posts, social, etc.)
- `src/models/` – data models and DTOs used by the API

## Useful Commands

- Build the project:

  ```bash path=null start=null
  cargo build
  ```

- Run tests (if defined):

  ```bash path=null start=null
  cargo test
  ```

- Format the code:

  ```bash path=null start=null
  cargo fmt
  ```

- Lint with Clippy (if installed):

  ```bash path=null start=null
  cargo clippy
  ```
