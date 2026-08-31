# shastack

The Unified Universal Stack Specification CLI.

Built with **Hexagonal Architecture** (Ports & Adapters) for clean separation of business logic from infrastructure.

## Architecture

```
sha (CLI)
├── domain/           # Pure business logic (zero dependencies)
│   ├── models.rs     # Workspace, Feature, Version, etc.
│   ├── errors.rs     # Custom error types
│   ├── ports.rs      # Interfaces for I/O (FileSystem, Git, Env, etc.)
│   └── use_cases.rs  # Business logic orchestration
├── adapters/         # Infrastructure implementations
│   ├── fs.rs         # File system adapter
│   ├── git.rs        # Git operations adapter
│   ├── env.rs        # Environment management adapter
│   ├── scaffold.rs   # Project scaffolding adapter
│   ├── http.rs       # HTTP/network adapter
│   ├── prompt.rs     # Interactive prompts adapter
│   ├── display.rs    # Output/display adapter
│   └── command.rs    # Command execution adapter
├── commands/         # CLI command definitions (driving adapter)
└── main.rs           # Composition root - wires everything together
```

See [docs/architecture.md](docs/architecture.md) for the full hexagonal architecture guide.

## Installation

### One-Liner Install (Recommended)

**macOS / Linux:**
```bash
curl -sSfL https://raw.githubusercontent.com/shawal-mbalire/shastack/main/cli/scripts/install.sh | bash
```

**Windows:**
```powershell
powershell -c "irm https://raw.githubusercontent.com/shawal-mbalire/shastack/main/cli/scripts/install.ps1 | iex"
```

### Manual Install

Download the latest binary from [GitHub Releases](https://github.com/shawal-mbalire/shastack/releases).

## Usage

```bash
sha --help

# Create a new workspace
sha new my-project

# Add features
sha add "Web Frontend (Angular)"
sha add "ML (Python/Notebooks)"

# Preview without making changes
sha --dry-run new my-project
```

## Project Structure

```
shastack/
├── cli/              # Rust CLI (sha binary) - Hexagonal Architecture
│   └── src/
│       ├── domain/   # Pure business logic
│       ├── adapters/ # Infrastructure implementations
│       ├── commands/ # CLI definitions
│       └── main.rs   # Composition root
├── frontend/         # Angular web application
├── landing/          # Public landing page
├── docs/             # Documentation
│   ├── architecture.md
│   ├── hexagonal_architecture.md
│   ├── stacks/       # Stack-specific guides
│   └── ...
├── shared/           # Cross-module event bus
├── benchmarks/       # Performance benchmarks
└── justfile          # Task runner
```

## Documentation

- [Architecture](docs/architecture.md) - Hexagonal architecture overview
- [Hexagonal Architecture Guide](docs/hexagonal_architecture.md) - Full guide
- [Multi-Stack Architecture](docs/shawal_multi_stack.md) - Cross-platform patterns
- [Getting Started](docs/getting-started.md)
- [CLI Reference](docs/cli-reference.md)
- [Contributing](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)

### Stack-Specific Guides

- [Backend (Python/FastAPI)](docs/stacks/backend-python.md)
- [Frontend (Angular)](docs/stacks/frontend-angular.md)
- [Frontend (React)](docs/stacks/frontend-react.md)
- [Mobile (Flutter)](docs/stacks/mobile-flutter.md)
- [Desktop (Tauri)](docs/stacks/desktop-tauri.md)
- [Embedded (MicroPython)](docs/stacks/embedded-micropython.md)
- [Embedded (C++)](docs/stacks/embedded-cpp.md)

## Testing

```bash
# Run all tests
cargo test

# Run with verbose output
cargo test -- --nocapture
```

## License

MIT
