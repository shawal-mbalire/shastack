set shell := ["bash", "-uc"]

# --- Global Commands ---

# Setup system tools and restore workspace structure
setup:
    sha deps
    sha restore

# Install all dependencies across the workspace
deps:
    sha deps

# Run tests across all modules
test:
    @for dir in cli frontend hardware; do \
        if [ -d "$dir" ] && [ -f "$dir/justfile" ]; then \
            echo "Testing $dir..."; \
            just -f "$dir/justfile" test; \
        fi; \
    done

# Run development server for the frontend
dev:
    (cd frontend && just dev)

# Flash firmware to hardware
flash:
    (cd hardware && just flash)

# --- CLI Development ---

# Build the sha CLI
build-cli:
    cargo build -p sha

# Install the sha CLI locally
install:
    cargo install --path cli

# --- Deployment ---

# Build and deploy the frontend to Firebase
deploy:
    (cd frontend && just build)
    firebase deploy --only hosting
