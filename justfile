set shell := ["bash", "-uc"]

# --- Global Commands ---

# Install dependencies based on .sha/config.json
deps:
    @echo "Installing project-wide dependencies..."
    @cargo build --manifest-path cli/Cargo.toml
    @cargo build --manifest-path landing-page/Cargo.toml

# Run the sha CLI (requires Rust)
sha *args:
    @cargo run --manifest-path cli/Cargo.toml -- {{args}}

# Run the sha CLI in dry-run mode
sha-dry *args:
    @cargo run --manifest-path cli/Cargo.toml -- --dry-run {{args}}

# Run tests per module
test module="all":
    {{ if module == "all" || module == "cli" }} cd cli && cargo test {{ endif }}
    {{ if module == "all" || module == "web" }} cd web && just test {{ endif }}
    {{ if module == "all" || module == "ml" }} cd ml && just test {{ endif }}

# --- Module Dev ---

dev-web:
    just --parallel client server

# Open ML Notebooks
notebooks:
    cd ml && uv run jupyter lab

# --- Global Sync ---
sync-all:
    @just sha release v1
    @just build

# Initialize a new module
new module:
    @just sha new {{module}}

# Setup the project
setup:
    @just deps
    @echo "shastack ready."

# Dry-run semantic release
release-dry:
    @npx semantic-release --dry-run
