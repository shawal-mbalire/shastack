# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.0] - 2026-09-01

### Added
- Core CLI (`sha`) with workspace lifecycle management
- `sha new` - Interactive workspace initialization with feature selection
- `sha add` - Add standalone modules to existing workspaces
- `sha restore` - Restore workspace structure from config
- `sha version` - Semantic versioning with conventional commit detection (`auto`)
- `sha env` - Keychain-backed environment management via envchain
- `sha deps` - Cross-platform dependency installation
- `sha sync-api` - API client generation from Zod/Pydantic definitions
- `sha pulse` - Workspace health monitoring
- `sha registry` - ML model registry with git-pinned weights
- `sha docs` - Documentation access
- `sha issue` - Issue-Driven Development workflow enforcement
- `sha upgrade` - Self-update mechanism
- Global `--dry-run` flag for safe preview of all operations
- Cross-platform install scripts (bash/PowerShell)
- Modular CI/CD with path-based triggers
- Conventional commits enforcement via git hooks
- SemVer automation with GitHub Actions release workflow
- Landing page (Angular) for public-facing site
- Documentation site structure
- Cross-module event bus for inter-module communication
- Performance benchmarking infrastructure
- Security policy and audit workflow

### Changed
- Repository restructured with dedicated landing/ and docs/ folders
- CLI registered as formal shastack module in config

## [0.2.1] - 2026-08-15

### Added
- Basic CLI with workspace init, add, restore commands
- Version management with conventional commit parsing
- Envchain integration for secret management
- Self-update via GitHub releases
- Angular frontend scaffolding

## [0.1.0] - 2026-08-01

### Added
- Initial project scaffolding
- Cargo workspace setup
- Basic justfile orchestration
