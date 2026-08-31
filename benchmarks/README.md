# Performance Benchmarks

Benchmarks for measuring shastack CLI and module performance.

## Running Benchmarks

```bash
# Run all benchmarks
just bench

# Run specific benchmark
cargo bench --manifest-path cli/Cargo.toml --bench workspace_ops
```

## Benchmarked Operations

| Operation | Target | Description |
|-----------|--------|-------------|
| `sha new` | < 500ms | Workspace initialization |
| `sha add` | < 200ms | Adding a module |
| `sha restore` | < 1s | Workspace restoration |
| `sha deps` | < 30s | Dependency installation |
| `sha version auto` | < 500ms | Version calculation |
| `sha pulse` | < 2s | Health check |
| Event bus emit | < 1ms | Cross-module event |
| Event bus list | < 10ms | Event listing |

## CI Integration

Benchmarks run on every PR via `.github/workflows/benchmarks.yml`.

Results are compared against the baseline on `main` and regressions are flagged.
