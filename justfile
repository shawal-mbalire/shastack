set shell := ["bash", "-uc"]

# --- Global Commands ---

# Setup system tools, hooks, and restore workspace structure
setup:
    sha deps
    just setup-hooks
    sha restore

# Install git hooks to enforce conventional commits
setup-hooks:
    git config core.hooksPath .githooks
    @echo "Git hooks installed from .githooks/"

# Install all dependencies across the workspace
deps:
    sha deps

# Run tests across all modules
test:
    @for dir in cli frontend; do \
        if [ -d "$dir" ] && [ -f "$dir/justfile" ]; then \
            echo "Testing $dir..."; \
            just -f "$dir/justfile" test; \
        fi; \
    done

# Run development server for the frontend
dev:
    (cd frontend && just dev)

# --- CLI Development ---

# Build the sha CLI
build-cli:
    (cd cli && cargo build)

# Check the CLI for errors
check-cli:
    (cd cli && cargo check)

# Install the sha CLI locally
install:
    cargo install --path cli

# Run clippy for the CLI
lint:
    (cd cli && cargo clippy)

# Format CLI code
fmt:
    (cd cli && cargo fmt)

# --- Deployment ---

# Build and deploy the frontend to Firebase
deploy:
    (cd frontend && just build)
    firebase deploy --only hosting
