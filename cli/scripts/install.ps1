Write-Host "Installing shastack (sha) CLI..."

if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "Error: Rust/Cargo is not installed. Please install it from https://rustup.rs/" -ForegroundColor Red
    exit 1
}

Write-Host "Running cargo install --git https://github.com/shawal-mbalire/shastack..."
cargo install --git https://github.com/shawal-mbalire/shastack

Write-Host "shastack (sha) has been installed successfully!" -ForegroundColor Green
Write-Host "Make sure ~/.cargo/bin is in your PATH."
