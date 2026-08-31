use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use anyhow::Result;

/// Event types for cross-module communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event type identifier (e.g., "model.trained", "web.deployed")
    pub event: String,
    /// Source module that emitted the event
    pub source: String,
    /// ISO 8601 timestamp
    pub timestamp: String,
    /// Event payload as key-value pairs
    pub data: HashMap<String, serde_json::Value>,
}

impl Event {
    pub fn new(event: &str, source: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        Self {
            event: event.to_string(),
            source: source.to_string(),
            timestamp: format!("{:?}", timestamp),
            data: HashMap::new(),
        }
    }

    pub fn with_data(mut self, key: &str, value: serde_json::Value) -> Self {
        self.data.insert(key.to_string(), value);
        self
    }
}

/// Event bus for cross-module communication
pub struct EventBus {
    events_dir: PathBuf,
    handlers: Arc<Mutex<HashMap<String, Vec<Box<dyn Fn(&Event) + Send>>>>>,
}

impl EventBus {
    pub fn new(root: &Path) -> Self {
        let events_dir = root.join("shared/events");
        Self {
            events_dir,
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Emit an event to the event bus
    pub fn emit(&self, event: Event) -> Result<()> {
        fs::create_dir_all(&self.events_dir)?;

        let filename = format!(
            "{}-{}.json",
            event.timestamp,
            event.event.replace('.', "-")
        );
        let path = self.events_dir.join(&filename);

        let json = serde_json::to_string_pretty(&event)?;
        fs::write(&path, json)?;

        // Notify registered handlers
        if let Ok(handlers) = self.handlers.lock() {
            if let Some(callbacks) = handlers.get(&event.event) {
                for callback in callbacks {
                    callback(&event);
                }
            }
        }

        Ok(())
    }

    /// Register a handler for a specific event type
    pub fn on<F>(&self, event_type: &str, handler: F)
    where
        F: Fn(&Event) + Send + 'static,
    {
        if let Ok(mut handlers) = self.handlers.lock() {
            handlers
                .entry(event_type.to_string())
                .or_insert_with(Vec::new)
                .push(Box::new(handler));
        }
    }

    /// List all events in the bus
    pub fn list(&self) -> Result<Vec<Event>> {
        let mut events = Vec::new();

        if !self.events_dir.exists() {
            return Ok(events);
        }

        for entry in fs::read_dir(&self.events_dir)? {
            let entry = entry?;
            if entry.path().extension().map_or(false, |e| e == "json") {
                let content = fs::read_to_string(entry.path())?;
                let event: Event = serde_json::from_str(&content)?;
                events.push(event);
            }
        }

        events.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(events)
    }

    /// Get events of a specific type
    pub fn get_by_type(&self, event_type: &str) -> Result<Vec<Event>> {
        let all = self.list()?;
        Ok(all.into_iter()
            .filter(|e| e.event == event_type)
            .collect())
    }

    /// Get events from a specific source module
    pub fn get_by_source(&self, source: &str) -> Result<Vec<Event>> {
        let all = self.list()?;
        Ok(all.into_iter()
            .filter(|e| e.source == source)
            .collect())
    }

    /// Clear all events
    pub fn clear(&self) -> Result<()> {
        if self.events_dir.exists() {
            fs::remove_dir_all(&self.events_dir)?;
            fs::create_dir_all(&self.events_dir)?;
        }
        Ok(())
    }
}

/// Predefined event types for common cross-module operations
pub mod events {
    pub const MODEL_TRAINED: &str = "model.trained";
    pub const MODEL_DEPLOYED: &str = "model.deployed";
    pub const WEB_DEPLOYED: &str = "web.deployed";
    pub const API_SYNCED: &str = "api.synced";
    pub const RESEARCH_BUILT: &str = "research.built";
    pub const HARDWARE_FLASHED: &str = "hardware.flashed";
    pub const VERSION_BUMPED: &str = "version.bumped";
    pub const DEPS_INSTALLED: &str = "deps.installed";
    pub const TESTS_PASSED: &str = "tests.passed";
    pub const SECURITY_AUDIT: &str = "security.audit";
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_event_bus_emit_and_list() -> Result<()> {
        let dir = tempdir()?;
        let bus = EventBus::new(dir.path());

        let event = Event::new("test.event", "test")
            .with_data("key", serde_json::json!("value"));

        bus.emit(event)?;

        let events = bus.list()?;
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event, "test.event");
        assert_eq!(events[0].source, "test");

        Ok(())
    }

    #[test]
    fn test_event_bus_filter_by_type() -> Result<()> {
        let dir = tempdir()?;
        let bus = EventBus::new(dir.path());

        bus.emit(Event::new("model.trained", "ml"))?;
        bus.emit(Event::new("web.deployed", "web"))?;
        bus.emit(Event::new("model.trained", "ml"))?;

        let model_events = bus.get_by_type("model.trained")?;
        assert_eq!(model_events.len(), 2);

        let web_events = bus.get_by_type("web.deployed")?;
        assert_eq!(web_events.len(), 1);

        Ok(())
    }

    #[test]
    fn test_event_bus_handler() -> Result<()> {
        let dir = tempdir()?;
        let bus = EventBus::new(dir.path());

        let received = Arc::new(Mutex::new(Vec::new()));
        let received_clone = received.clone();

        bus.on("test.event", move |event| {
            received_clone.lock().unwrap().push(event.event.clone());
        });

        bus.emit(Event::new("test.event", "test"))?;

        let events = received.lock().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0], "test.event");

        Ok(())
    }
}
