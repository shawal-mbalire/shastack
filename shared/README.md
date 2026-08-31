# Shared Module

Cross-module types, constants, and event bus for shastack.

## Event Bus

The event bus enables modules to communicate via JSON events stored in `events/`.

### Emitting Events

```rust
use shared::events::{Event, EventBus};

let bus = EventBus::new(&workspace_root);
let event = Event::new("model.trained", "ml")
    .with_data("accuracy", serde_json::json!(0.95));
bus.emit(event)?;
```

### Consuming Events

```rust
// Register handler
bus.on("model.trained", |event| {
    println!("Model trained: {:?}", event.data);
});

// Or list events
let events = bus.get_by_type("model.trained")?;
```

### Event Types

| Event | Source | Description |
|-------|--------|-------------|
| `model.trained` | ml | ML model training completed |
| `model.deployed` | ml | Model deployed to registry |
| `web.deployed` | web | Web app deployed |
| `api.synced` | cli | API clients regenerated |
| `research.built` | research | PDF paper built |
| `hardware.flashed` | hardware | Firmware flashed |
| `version.bumped` | cli | Version incremented |
| `deps.installed` | cli | Dependencies installed |
| `tests.passed` | cli | Tests passed |
| `security.audit` | cli | Security audit completed |
