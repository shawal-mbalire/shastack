# Contributing to shastack

Thank you for your interest in contributing! This guide will help you get started.

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Just](https://just.systems/) command runner
- [Git](https://git-scm.com/)
- [GitHub CLI](https://cli.github.com/) (`gh`)

## Getting Started

```bash
# Clone the repository
git clone https://github.com/shawal-mbalire/shastack.git
cd shastack

# Install dependencies
just setup

# Build the CLI
just build-cli

# Run tests
just test
```

## Development Workflow

### Issue-Driven Development (IDD)

1. **Never start code without an Issue**
2. Create a branch: `sha issue start <ID>`
3. Make changes following conventions below
4. Finish: `sha issue finish` (pushes branch, creates PR)

### Branch Naming

```
issue-<ID>-<short-description>
```

Example: `issue-42-add-dry-run-flag`

### Commit Messages

We enforce [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

feat(cli): add --dry-run flag
fix(workspace): handle missing config gracefully
docs(readme): update installation instructions
chore(deps): bump clap to 4.6.0
```

**Types:**
- `feat` - New feature (triggers minor version bump)
- `fix` - Bug fix (triggers patch version bump)
- `docs` - Documentation only
- `chore` - Maintenance tasks
- `refactor` - Code restructuring without behavior change
- `test` - Adding or fixing tests
- `ci` - CI/CD changes
- `style` - Code style/formatting
- `perf` - Performance improvements
- `build` - Build system changes
- `revert` - Reverting a previous commit

**Breaking changes:** Add `!` after type/scope:
```
feat(api)!: drop v1 endpoint support
```

## Code Style

### Rust (CLI)
- Use `cargo fmt` before committing
- Run `cargo clippy` and fix all warnings
- Prefer `anyhow::Result` for error handling
- Use `colored` crate for terminal output
- Follow existing patterns in `cli/src/`

### TypeScript (Frontend)
- Follow Angular style guide
- Use SCSS for styles
- Components should be self-contained

## Testing

```bash
# Run all tests
just test

# Run CLI tests only
cargo test --manifest-path cli/Cargo.toml

# Run with output
cargo test --manifest-path cli/Cargo.toml -- --nocapture
```

## Project Structure

```
shastack/
├── cli/            # Rust CLI (sha binary)
├── frontend/       # Angular web application
├── landing/        # Public landing page
├── docs/           # Documentation site
├── shared/         # Cross-module types and event bus
├── benchmarks/     # Performance benchmarks
├── .sha/           # Workspace configuration
├── .github/        # CI/CD workflows
└── justfile        # Task runner
```

## Pull Request Process

1. Ensure all tests pass: `just test`
2. Lint code: `just lint`
3. Format code: `just fmt`
4. Update documentation if needed
5. Fill out the PR template completely
6. Link the related Issue

## Questions?

Open a Discussion on GitHub or ask in the project's communication channels.
