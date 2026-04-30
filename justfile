set shell := ["bash", "-uc"]

# Install all dependencies
deps:
    @echo "Installing all dependencies via shastack..."

# Run the sha CLI (requires Rust)
sha *args:
    @cargo run --manifest-path cli/Cargo.toml -- {{args}}

# Run the sha CLI in dry-run mode
sha-dry *args:
    @cargo run --manifest-path cli/Cargo.toml -- --dry-run {{args}}

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
