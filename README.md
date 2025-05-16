# http.rs

> A simple CLI tool to serve the current directory over HTTP.

## Build & Run

```bash
# build release
cargo build --release

# change to output directory
cd ./target/release/rhttp

# serve current directory on default port 8080
rhttp

# serve on port 3000
rhttp --port 3000
```

## CLI Options

- `-p, --port <PORT>`, port to listen on (default: 8080)
