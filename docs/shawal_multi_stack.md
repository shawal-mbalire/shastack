# Hexagonal Architecture Across Tech Stacks

The core rule remains universal across all languages and platforms: The Domain must be pure. UI, HTTP clients, hardware APIs, and databases are all just Adapters.

---

## 1. Backend: Python & Google Cloud

The classic environment. The backend acts as the authoritative domain. We separate the HTTP/Event triggers from the business logic and the Cloud infrastructure.

- **Domain:** Pure Python dataclasses and pure functions.
- **Ports:** Python `typing.Protocol` or `abc.ABC`.
- **Driving Adapters (Triggering the app):** FastAPI/Flask (HTTP triggers). GCloud Specific: Cloud Functions (functions-framework), Pub/Sub event handlers.
- **Driven Adapters (Outbound I/O):** SQLAlchemy (Cloud SQL). GCloud Specific: Firestore SDK, Cloud Storage SDK, Google Secret Manager.
- **Composition Root:** `main.py` reading `os.environ` and wiring adapters.

### Example: FastAPI & Firestore

```python
# 1. Port (Pure Python)
from typing import Protocol
from dataclasses import dataclass

@dataclass
class Document:
    id: str
    content: str

class DocumentRepository(Protocol):
    def save(self, doc: Document) -> None: ...

# 2. Driven Adapter (GCloud Firestore)
from google.cloud import firestore

class FirestoreDocAdapter(DocumentRepository):
    def __init__(self, collection_name: str):
        self.db = firestore.Client()
        self.collection = self.db.collection(collection_name)

    def save(self, doc: Document) -> None:
        self.collection.document(doc.id).set({"content": doc.content})

# 3. Use Case (Pure Python)
def create_document(doc_id: str, content: str, repo: DocumentRepository) -> Document:
    # ... pure business logic, validation, etc.
    doc = Document(id=doc_id, content=content)
    repo.save(doc)
    return doc

# 4. Driving Adapter & Composition Root (FastAPI)
from fastapi import FastAPI
app = FastAPI()

# Wiring
firestore_repo = FirestoreDocAdapter("documents")

@app.post("/docs/{doc_id}")
def api_create_document(doc_id: str, content: dict):
    # The route acts as a controller, delegating to the pure use case
    return create_document(doc_id, content["text"], repo=firestore_repo)
```

---

## 2. Frontend: UI & State Management

In the frontend, the UI itself is an external framework. The "Database" is often just an HTTP API or LocalStorage. UI components act as Driving Adapters, capturing user intent and delegating it to the core.

- **Domain:** Pure TypeScript class, interface, and plain TS files. No framework-specific imports (e.g., no React, no `@Injectable()`) in the pure domain models/logic.
- **Ports:** TypeScript `interface` or `type` definitions.
- **Driving Adapters (UI & State):** UI components collect user input, invoke Use Cases, and render the resulting state. State management hooks/services act as thin coordinators between UI and Domain.
- **Driven Adapters:** `fetch` API, Axios, Angular `HttpClient`, or `localStorage`.
- **Composition Root:** Framework dependency injection (Angular's `app.config.ts`) or explicit object wiring/React Context at the application root.

### Example: React

```typescript
// 1. Port (Pure TS)
export interface CartRepository {
  saveCart(cart: Cart): Promise<void>;
}

// 2. Driven Adapter (Infrastructure)
export class LocalStorageCartAdapter implements CartRepository {
  async saveCart(cart: Cart): Promise<void> {
    localStorage.setItem('cart', JSON.stringify(cart));
  }
}

// 3. Application Use Case (Pure TS)
export class AddToCartUseCase {
  constructor(private cartRepo: CartRepository) {}

  async execute(productId: string, quantity: number) {
    // ... pure business logic
    await this.cartRepo.saveCart(cart);
    return cart;
  }
}

// 4. Driving Adapter (React Component & Hook)
function useCartController(addToCartUseCase: AddToCartUseCase) {
  const [cartState, setCartState] = useState<Cart | null>(null);

  const handleAdd = async (productId: string) => {
    const updatedCart = await addToCartUseCase.execute(productId, 1);
    setCartState(updatedCart);
  };
  return { cartState, handleAdd };
}

export const ProductView = ({ productId, useCase }: { productId: string, useCase: AddToCartUseCase }) => {
  const { handleAdd } = useCartController(useCase);
  return <button onClick={() => handleAdd(productId)}>Add to Cart</button>;
};
```

### Example: Angular

```typescript
// 1. Driven Adapter (Angular specific)
@Injectable()
export class HttpUserAdapter implements UserRepository {
  constructor(private http: HttpClient) {}

  async getUser(id: string): Promise<User> {
    const dto = await firstValueFrom(this.http.get<UserDto>(`/api/users/${id}`));
    return mapDtoToDomain(dto);
  }
}

// 2. Driving Adapter (Angular Component)
@Component({
  selector: 'app-user-profile',
  template: `<button (click)="loadUser()">Load User</button>`
})
export class UserProfileComponent {
  constructor(private userRepo: UserRepository) {}

  async loadUser() {
     const user = await this.userRepo.getUser('123');
  }
}

// 3. Composition Root (app.config.ts)
export const appConfig: ApplicationConfig = {
  providers: [{ provide: UserRepository, useClass: HttpUserAdapter }]
};
```

---

## 3. Mobile: Flutter

Very similar to the frontend, but dealing with mobile hardware constraints, local SQLite, and state managers like Riverpod or BLoC.

- **Domain:** Pure Dart classes and methods. Zero `flutter/material.dart` imports.
- **Ports:** Dart `abstract class`.
- **Driving Adapters:** State Managers (Riverpod Notifier, BLoC Cubit). They listen to UI events and call pure Use Cases. Flutter Widgets just display state and send events to the state manager.
- **Driven Adapters:** `http`/`dio` (API Gateway), `sqflite` (Local DB), Hardware APIs (Camera, GPS).
- **Composition Root:** `main.dart` using a DI package like `get_it` or overriding Riverpod Providers.

### Example: Flutter with Riverpod

```dart
// 1. Port & Domain (Pure Dart)
abstract class LocationPort {
  Future<Coordinates> getCurrentLocation();
}

class TrackLocationUseCase {
  final LocationPort locationPort;
  TrackLocationUseCase(this.locationPort);

  Future<Coordinates> execute() async {
    return await locationPort.getCurrentLocation(); // + Business rules
  }
}

// 2. Driven Adapter (Hardware specific)
import 'package:geolocator/geolocator.dart';

class GeolocatorAdapter implements LocationPort {
  @override
  Future<Coordinates> getCurrentLocation() async {
    final pos = await Geolocator.getCurrentPosition();
    return Coordinates(lat: pos.latitude, lng: pos.longitude);
  }
}

// 3. Driving Adapter (Riverpod State Controller)
final locationProvider = AsyncNotifierProvider<LocationNotifier, Coordinates>(LocationNotifier.new);

class LocationNotifier extends AsyncNotifier<Coordinates> {
  late final TrackLocationUseCase _useCase;

  @override
  Future<Coordinates> build() async {
    _useCase = TrackLocationUseCase(GetIt.I<LocationPort>());
    return await _useCase.execute();
  }

  Future<void> refresh() async {
    state = const AsyncLoading();
    state = await AsyncValue.guard(() => _useCase.execute());
  }
}

// 4. Driving Adapter (Flutter Widget)
import 'package:flutter/material.dart';

class LocationView extends ConsumerWidget {
  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final locState = ref.watch(locationProvider);
    return locState.when(
      data: (coords) => Text('Lat: ${coords.lat}, Lng: ${coords.lng}'),
      loading: () => CircularProgressIndicator(),
      error: (err, stack) => Text('Error: $err'),
    );
  }
}
```

---

## 4. Desktop: Tauri + Angular

Tauri forces a hard boundary between the Frontend (UI) and Backend (OS access). In Tauri, you actually have two hexagons talking to each other via Inter-Process Communication (IPC).

### Part A: The Frontend Hexagon (UI)

**Driven Adapter:** Instead of an `HttpClient`, you write a Tauri IPC Adapter.

```typescript
// Driven Adapter in Frontend calling Rust
import { invoke } from '@tauri-apps/api/core';

export class TauriFileStorageAdapter implements FileStoragePort {
  async saveFile(content: string): Promise<void> {
    await invoke('save_file_command', { payload: content });
  }
}
```

### Part B: The Rust Hexagon (System Core)

- **Domain:** Pure Rust `struct` and `impl`.
- **Ports:** Rust `trait`.
- **Driving Adapters:** Tauri `#[tauri::command]` functions listening to the Frontend.
- **Driven Adapters:** Rust `std::fs` (File system), `rusqlite` (Local DB).

```rust
// Driving Adapter (Tauri Command listens to Frontend)
#[tauri::command]
fn save_file_command(
    payload: String,
    state: tauri::State<AppDIState>
) -> Result<(), String> {
    core_domain::save_document(&payload, &state.file_repository)
}
```

---

## 5. Embedded Systems: C++ & MicroPython

Hardware abstraction is the ultimate use case. You can compile and test your pure control loops on your laptop without needing the physical microcontroller.

- **Domain:**
  - C++: Pure standard C++ (No `<Arduino.h>`, no ESP-IDF).
  - MicroPython: Pure Python logic.
- **Ports:** Abstract classes in C++, or `typing.Protocol` / Duck Typing in Python.
- **Driving Adapters:** Hardware Interrupts (ISR), RTOS tasks (FreeRTOS), or `loop()`.
- **Driven Adapters:** I2C/SPI drivers, GPIO, PWM generators.

### Example: MicroPython (ESP32/Raspberry Pi Pico)

```python
# 1. Port (Protocol / Duck Typing)
class RelayPort:
    def turn_on(self) -> None: pass
    def turn_off(self) -> None: pass

# 2. Pure Domain Use Case (Testable on your PC)
class PumpController:
    def __init__(self, pump_relay: RelayPort):
        self.pump = pump_relay
        self.is_active = False

    def toggle_irrigation(self):
        if self.is_active:
            self.pump.turn_off()
        else:
            self.pump.turn_on()
        self.is_active = not self.is_active

# 3. Driven Adapter (Hardware Specific MicroPython)
from machine import Pin

class ESP32RelayAdapter(RelayPort):
    def __init__(self, pin_number: int):
        self.pin = Pin(pin_number, Pin.OUT)

    def turn_on(self) -> None:
        self.pin.value(1)

    def turn_off(self) -> None:
        self.pin.value(0)

# 4. Driving Adapter (Hardware Interrupt / Button Press)
# Composition Root
water_pump = ESP32RelayAdapter(pin_number=14)
controller = PumpController(pump_relay=water_pump)

# Hardware driving the use case
button = Pin(0, Pin.IN, Pin.PULL_UP)
button.irq(trigger=Pin.IRQ_FALLING, handler=lambda p: controller.toggle_irrigation())
```

---

## Universal Golden Rules for Multi-Platform

1. **The DTO Boundary:** Adapters must translate external formats (JSON, SQL Rows, raw byte buffers) into Pure Domain Models before passing them inward.
2. **Never import external frameworks in Domain:** No `import { Component }` in Angular, no `package:flutter` in Dart, no `#include <Arduino.h>` in C++.
3. **Mocking is Universal:** Because all platforms use Interfaces for Ports, you can write pure in-memory Mocks in TypeScript, Dart, Rust, Python, and C++ equally well for lightning-fast unit tests.
