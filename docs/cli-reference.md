# CLI Reference

## Global Flags

```
--dry-run    Preview what would happen without making changes
--help       Show help information
--version    Show version
```

## Commands

### `sha new <name>`

Creates a new shastack workspace with interactive feature selection.

```bash
sha new my-project
```

### `sha add [feature]`

Adds a new module to the current workspace.

```bash
sha add "Web Frontend (Angular)"
sha add "ML (Python/Notebooks)"
sha add .  # Add current directory as a module
```

### `sha restore`

Restores workspace structure from `.sha/config.json`.

```bash
sha restore
```

### `sha version [component]`

Manages semantic versioning.

```bash
sha version           # Show current version
sha version patch     # Bump patch (0.1.0 → 0.1.1)
sha version minor     # Bump minor (0.1.0 → 0.2.0)
sha version major     # Bump major (0.1.0 → 1.0.0)
sha version auto      # Auto-detect from conventional commits
```

### `sha env <action>`

Manages environment variables via envchain (keychain-backed).

```bash
sha env set API_KEY sk-xxx
sha env get API_KEY
sha env list
```

### `sha deps`

Installs system-wide and project dependencies.

```bash
sha deps
```

### `sha pulse`

Checks health and heartbeats of workspace modules.

```bash
sha pulse
```

### `sha registry <action>`

Manages the ML model registry.

```bash
sha registry pin my-model ./weights/model.bin
sha registry list
```

### `sha sync-api [url]`

Generates clients from API definitions.

```bash
sha sync-api
sha sync-api https://api.example.com/schema.json
```

### `sha docs [options]`

Opens documentation.

```bash
sha docs                    # Open rustup docs
sha docs --feature web      # Open feature-specific docs
sha docs --std              # Open standard library docs
```

### `sha issue <action>`

Enforces Issue-Driven Development.

```bash
sha issue start 42           # Create branch for issue #42
sha issue status             # Check IDD compliance
sha issue finish             # Push and create PR
```

### `sha upgrade`

Self-updates the CLI from GitHub releases.

```bash
sha upgrade
sha upgrade --url https://example.com/sha
```

## Dry-Run Mode

Every command supports `--dry-run` to preview changes without executing:

```bash
sha --dry-run new my-project     # Show what would be created
sha --dry-run add "web"          # Show what would be added
sha --dry-run version patch      # Show what version would be set
sha --dry-run env set KEY val    # Show what would be stored
```

Dry-run mode:
- Prints all operations that would be performed
- Does not create files, modify configs, or make network requests
- Exits with code 0 on success
