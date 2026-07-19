$ErrorActionPreference = "Stop"
$env:RUST_BACKTRACE = "1"
cargo run -p acme-gallery
