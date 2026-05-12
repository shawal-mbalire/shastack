set shell := ["bash", "-uc"]

# --- Global Commands ---

# Install all system and project dependencies
deps:
    sha deps

# Synchronize all APIs and rebuild artifacts
sync-all:
    sha sync-api
    just build

# Check the health of all modules
pulse:
    sha pulse

# --- Development ---

# Build the sha CLI
build:
    cargo build -p sha

# Run unit tests
test:
    cargo test -p sha

# Install the sha CLI locally
install:
    cargo install --path cli

# Run the CLI with arguments (e.g., just run -- help)
run *args:
    cargo run -p sha -- {{args}}

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
