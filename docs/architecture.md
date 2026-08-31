# Architecture

## Overview

shastack uses a flat monorepo structure with domain-specific top-level folders, unified by the `sha` CLI and `justfile` orchestration.

## Directory Structure

```
shastack/
├── .sha/                    # CLI configuration and feature manifest
│   └── config.json          # Workspace manifest (name, version, features)
├── .github/workflows/       # Global CI/CD triggers
├── cli/                     # Rust CLI (sha binary)
│   ├── src/
│   │   ├── main.rs          # Entry point
│   │   ├── commands.rs      # Command implementations
│   │   └── workspace/       # Workspace management
│   ├── Cargo.toml
│   └── scripts/             # Install scripts
├── frontend/                # Angular web application
├── landing/                 # Public landing page
├── docs/                    # Documentation site
├── shared/                  # Cross-module types and event bus
│   └── events/              # Event definitions
├── benchmarks/              # Performance benchmarks
└── justfile                 # Master task runner
```

## Module System

Each module is a self-contained unit with:
- Its own `justfile` for local automation
- Path-based CI triggers (`.github/workflows/`)
- Standard lifecycle: `deps` → `test` → `build` → `deploy`

### Supported Modules

| Module | Type | Directory |
|--------|------|-----------|
| Web Frontend | Angular | `web/client/` |
| Web Backend | Flask/Python | `web/server/` |
| Landing Page | Angular | `landing/` |
| Mobile App | Flutter | `mobile/app/` |
| Research | LaTeX | `research/` |
| ML | Python | `ml/` |
| Hardware | C++/MicroPython | `hardware/` |

## Cross-Module Communication

Modules communicate via a JSON-based event bus in `shared/events/`:

```json
{
  "event": "model.trained",
  "source": "ml",
  "timestamp": "2026-09-01T00:00:00Z",
  "data": {
    "model": "classifier-v1",
    "accuracy": 0.95
  }
}
```

## CI/CD Pipeline

1. **Path-Based Triggers**: Changes in `web/` only trigger web CI
2. **Global Coordinator**: `.github/workflows/release.yml` handles releases
3. **Conventional Commits**: Enforced via git hooks
4. **SemVer Automation**: Version bumped from commit history

## Environment Management

Secrets are stored in the system keychain via `envchain`:
- macOS: Keychain
- Linux: gnome-keyring / KWallet
- Windows: Credential Manager

No `.env` files are committed to the repository.
