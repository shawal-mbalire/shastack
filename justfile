set shell := ["bash", "-uc"]

# Install all dependencies
deps:
    @echo "Installing all dependencies via shastack..."

# Run the sha CLI (requires Bun)
sha *args:
    @bun run cli/main.ts {{args}}

# Initialize a new module
new module:
    @just sha new {{module}}

# Setup the project
setup:
    @just deps
    @echo "shastack ready."
