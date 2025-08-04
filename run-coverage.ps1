$ErrorActionPreference = "Stop"

if (!(Get-Command cargo-llvm-cov -ErrorAction SilentlyContinue)) {
    cargo install cargo-llvm-cov
}
else {
    Write-Host "cargo-llvm-cov already installed"
}

$COMMON_ARGS = "--ignore-filename-regex=main\.rs", "--all-features", "--workspace", "--show-missing-lines"

cargo llvm-cov @COMMON_ARGS --codecov --output-path codecov.json --fail-under-lines 90
cargo llvm-cov @COMMON_ARGS --html
