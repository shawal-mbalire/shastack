set shell := ["bash", "-uc"]

# --- Global Commands ---

# Setup system tools and restore workspace structure
setup:
    sha deps
    sha restore

# Install project-wide and module dependencies
deps:
    @for dir in web/client web/server ml mobile/app landing research hardware; do \
        if [ -d "$dir" ]; then \
            echo "Installing dependencies in $dir..."; \
            (cd "$dir" && just deps); \
        fi; \
    done

# Run tests across all modules
test module="all":
    @for dir in web/client web/server ml mobile/app landing research hardware; do \
        if [ -d "$dir" ] && ([ "{{module}}" = "all" ] || [ "{{module}}" = "$dir" ]); then \
            echo "Testing $dir..."; \
            (cd "$dir" && just test); \
        fi; \
    done

# Synchronize all APIs
sync-api:
    sha sync-api

# Check workspace health
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
