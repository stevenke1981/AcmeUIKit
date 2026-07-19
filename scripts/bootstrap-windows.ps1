$ErrorActionPreference = "Stop"

Write-Host "[Acme] Checking Rust toolchain..."
if (-not (Get-Command rustup -ErrorAction SilentlyContinue)) {
    throw "Rustup not found. Install it from rustup.rs, then reopen PowerShell."
}

rustup toolchain install stable --profile minimal --component rustfmt --component clippy
rustup default stable

Write-Host "[Acme] Fetching dependencies..."
cargo fetch

Write-Host "[Acme] Bootstrap completed. Run ./scripts/check.ps1"
