$ErrorActionPreference = "Stop"

Write-Host "=== Tool Versions ===" -ForegroundColor Green
Write-Host "Rust: $(rustc --version)"
Write-Host "Cargo: $(cargo --version)"
Write-Host "Clippy: $(cargo clippy --version)"
Write-Host "Rustfmt: $(rustfmt --version)"
Write-Host "=== End Tool Versions ===" -ForegroundColor Green
Write-Host

cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings -A dead_code -A clippy::module-inception