set shell := ["bash", "-uc"]

# --- Development ---

# Build the sha CLI
build:
    cargo build

# Run unit tests
test:
    cargo test

# Install the sha CLI locally
install:
    cargo install --path .

# Run the CLI with arguments (e.g., just run -- help)
run *args:
    cargo run -- {{args}}

# --- Linting & Formatting ---

# Check code for errors
check:
    cargo check

# Format code
fmt:
    cargo fmt

# Run clippy
lint:
    cargo clippy
