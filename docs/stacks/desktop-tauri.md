# Desktop: Tauri + Angular

Hexagonal architecture for Tauri desktop app with Angular frontend and Rust backend.

## Part A: Frontend Hexagon (Angular)

### Driven Adapter (Tauri IPC)

```typescript
import { invoke } from '@tauri-apps/api/core';

export class TauriFileStorageAdapter implements FileStoragePort {
  async saveFile(content: string): Promise<void> {
    await invoke('save_file_command', { payload: content });
  }
}
```

## Part B: Rust Hexagon (System Core)

### Domain (Pure Rust)

```rust
// Pure Rust structs and traits - no Tauri imports in domain
pub trait FileRepository {
    fn save(&self, content: &str) -> Result<(), String>;
}

pub fn save_document(content: &str, repo: &dyn FileRepository) -> Result<(), String> {
    // pure business logic
    repo.save(content)
}
```

### Driven Adapter (File System)

```rust
use std::fs;

pub struct LocalFileAdapter {
    path: String,
}

impl FileRepository for LocalFileAdapter {
    fn save(&self, content: &str) -> Result<(), String> {
        fs::write(&self.path, content).map_err(|e| e.to_string())
    }
}
```

### Driving Adapter (Tauri Command)

```rust
#[tauri::command]
fn save_file_command(
    payload: String,
    state: tauri::State<AppDIState>,
) -> Result<(), String> {
    core_domain::save_document(&payload, &state.file_repository)
}
```

## Cross-Cutting Concerns (Rust Backend)

### Logging

```rust
// Port (Domain)
pub trait Logger {
    fn info(&self, msg: &str);
    fn error(&self, msg: &str);
}

// Adapter (tracing)
use tracing::{info, error};

struct TracingLogger;
impl Logger for TracingLogger {
    fn info(&self, msg: &str) { info!("{}", msg); }
    fn error(&self, msg: &str) { error!("{}", msg); }
}
```

### Configuration / Secrets

```rust
// Adapter resolves config, domain receives as arguments
use std::env;

fn get_config() -> AppConfig {
    AppConfig {
        max_retries: env::var("MAX_RETRIES").unwrap_or("3".to_string()).parse().unwrap(),
    }
}

// Domain receives as params
pub fn save_document(content: &str, repo: &dyn FileRepository, max_retries: u32) -> Result<(), String> {
    // pure logic with config
}
```

### Caching (Decorator Pattern)

```rust
pub struct CacheAdapter {
    cache: std::collections::HashMap<String, Vec<u8>>,
    fallback: Box<dyn FileRepository>,
}

impl FileRepository for CacheAdapter {
    fn read(&self, path: &str) -> Result<Vec<u8>, String> {
        if let Some(cached) = self.cache.get(path) {
            return Ok(cached.clone());
        }
        let data = self.fallback.read(path)?;
        // self.cache.insert(path.to_string(), data.clone());
        Ok(data)
    }
}
```

### Auth

```rust
// Domain enforces rules
pub fn delete_file(path: &str, user: &User) -> Result<(), String> {
    if user.role != "admin" {
        return Err("Admin only".to_string());
    }
    Ok(())
}
```

### Telemetry & Metrics

```rust
// Port (Domain)
pub trait Metrics {
    fn increment(&self, name: &str);
    fn histogram(&self, name: &str, value: f64);
}

// Adapter (sentry)
struct SentryMetrics;
impl Metrics for SentryMetrics {
    fn increment(&self, name: &str) {
        sentry::capture_message(name, sentry::Level::Info);
    }
    fn histogram(&self, name: &str, value: f64) {
        sentry::configure_scope(|scope| {
            scope.set_extra(name, value.into());
        });
    }
}
```

### Event Publishing

```rust
// Port (Domain)
pub trait EventPublisher {
    fn publish(&self, event: DomainEvent);
}

// Adapter (IPC to frontend)
struct TauriEventPublisher {
    app_handle: tauri::AppHandle,
}

impl EventPublisher for TauriEventPublisher {
    fn publish(&self, event: DomainEvent) {
        self.app_handle.emit_all("domain-event", &event).ok();
    }
}
```

## Cross-Cutting Concerns (Angular Frontend)

### Logging

```typescript
@Injectable()
export class ConsoleLogger implements Logger {
  info(msg: string) { console.log(`[INFO] ${msg}`); }
  error(msg: string) { console.error(`[ERROR] ${msg}`); }
}
```

### Auth

```typescript
@Injectable()
export class AuthAdapter {
  getUser(): User | null {
    const token = localStorage.getItem('token');
    return token ? jwtDecode<User>(token) : null;
  }
}
```

## Localized Concerns (Desktop)

### File System Access

```rust
// Adapter (Tauri fs)
use tauri::api::path::app_data_dir;

pub struct DesktopFileAdapter {
    base_path: PathBuf,
}

impl DesktopFileAdapter {
    pub fn new(app_handle: &tauri::AppHandle) -> Self {
        Self {
            base_path: app_data_dir(app_handle.config()).unwrap(),
        }
    }

    pub fn read_config(&self, name: &str) -> Result<String, String> {
        let path = self.base_path.join(name);
        fs::read_to_string(path).map_err(|e| e.to_string())
    }

    pub fn write_config(&self, name: &str, content: &str) -> Result<(), String> {
        let path = self.base_path.join(name);
        fs::write(path, content).map_err(|e| e.to_string())
    }
}
```

### System Tray

```rust
// Adapter (Tauri tray)
use tauri::{CustomMenuItem, Menu, SystemTray, SystemTrayEvent};

pub fn create_system_tray() -> SystemTray {
    let menu = Menu::new()
        .add_item(CustomMenuItem::new("show", "Show Window"))
        .add_item(CustomMenuItem::new("quit", "Quit"));

    SystemTray::new().with_menu(menu)
}

pub fn handle_tray_event(event: SystemTrayEvent) {
    match event {
        SystemTrayEvent::MenuItemClick { id, .. } => {
            match id.as_str() {
                "quit" => std::process::exit(0),
                "show" => { /* show window */ }
                _ => {}
            }
        }
        _ => {}
    }
}
```

### Auto-Updater

```rust
// Adapter (Tauri updater)
use tauri::updater;

pub fn check_for_updates(app_handle: &tauri::AppHandle) {
    updater::builder(app_handle.clone())
        .header("Authorization", "Bearer token")
        .check()
        .unwrap();
}
```

### Window Management

```rust
// Adapter (Tauri window)
use tauri::Window;

pub struct WindowManager {
    window: Window,
}

impl WindowManager {
    pub fn minimize(&self) { self.window.minimize().unwrap(); }
    pub fn maximize(&self) { self.window.maximize().unwrap(); }
    pub fn close(&self) { self.window.close().unwrap(); }
    pub fn set_title(&self, title: &str) { self.window.set_title(title).unwrap(); }
}
```

### Global Shortcut

```rust
// Adapter (Tauri global shortcut)
use tauri::GlobalShortcutManager;

pub fn register_shortcut(app_handle: &tauri::AppHandle, shortcut: &str) {
    let mut shortcuts = app_handle.global_shortcut_manager();
    shortcuts.register(shortcut, move || {
        // Handle shortcut
    }).unwrap();
}
```

### Notification (Desktop)

```rust
// Adapter (tauri-notification)
use tauri_notification::Notification;

pub fn send_notification(title: &str, body: &str) {
    Notification::new()
        .title(title)
        .body(body)
        .show()
        .unwrap();
}
```
