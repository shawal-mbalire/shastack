# Getting Started with shastack

## Installation

### One-Liner Install (Recommended)

**macOS / Linux:**
```bash
curl -sSfL https://raw.githubusercontent.com/shawal-mbalire/shastack/main/scripts/install.sh | bash
```

**Windows:**
```powershell
powershell -c "irm https://raw.githubusercontent.com/shawal-mbalire/shastack/main/scripts/install.ps1 | iex"
```

### Manual Install

Download the latest binary from [GitHub Releases](https://github.com/shawal-mbalire/shastack/releases).

## Create Your First Workspace

```bash
# Create a new workspace
sha new my-project

# Navigate into it
cd my-project

# Check status
sha pulse
```

## Add Features

```bash
# Add a web frontend
sha add "Web Frontend (Angular)"

# Add ML capabilities
sha add "ML (Python/Notebooks)"

# Check what's installed
sha pulse
```

## Common Commands

| Command | Description |
|---------|-------------|
| `sha new <name>` | Create new workspace |
| `sha add <feature>` | Add a module |
| `sha deps` | Install dependencies |
| `sha test` | Run all tests |
| `sha build <feature>` | Build artifacts |
| `sha deploy <feature>` | Deploy to target |
| `sha version auto` | Auto-bump version |
| `sha env set KEY val` | Store secret |
| `sha pulse` | Check health |
| `sha --dry-run <cmd>` | Preview without changes |

## Next Steps

- Read the [CLI Reference](./cli-reference.md)
- Learn about [Architecture](./architecture.md)
- Check the [Contributing Guide](../CONTRIBUTING.md)
