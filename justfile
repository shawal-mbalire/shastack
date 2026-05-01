set shell := ["bash", "-uc"]

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
