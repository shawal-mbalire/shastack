# Ports and Adapters (Hexagonal Architecture)

Separate dependencies from business logic to ease swapping them out and enable blazing-fast testing.

## 1. Domain

Pure, isolated rules and decisions. Zero knowledge of external libraries, frameworks, or infrastructure.

- models: Pure data structures and objects that represent business concepts.
- errors: Custom exceptions specific to business rules (e.g., InsufficientFundsError).
- ports: Interfaces (Protocols or ABCs) defining the I/O the domain needs (Repositories for DBs, Gateways for APIs).
- use cases: Orchestrate the flow by taking in domain models and calling Ports.

```python
from typing import Protocol

# 1. Model
class Order: ... 

# 2. Port (Repository/Gateway)
class OrderPort(Protocol):
    def save(self, order: Order) -> None: ...

# 3. Use Case
def create_order(order_data: dict, repo: OrderPort):
    order = Order(**order_data)
    # ... pure business rules ...
    repo.save(order)
```

## 2. Adapters

Specific implementations of external libraries to fulfill the Ports, or to trigger the Use Cases.

- Driven Adapters (Implement Ports): SQLAlchemy (DB), Stripe (Gateway), Console/Sentry (Logger).
- Driving Adapters (Trigger the Domain): FastAPI or Flask routes mapping raw JSON to Domain Models.

```python
# Driven Adapter (SQLAlchemy implementing OrderPort)
class SqlOrderAdapter(OrderPort):
    def __init__(self, session):
        self.session = session
        
    def save(self, order: Order) -> None:
        self.session.add(DBOrder(id=order.id)) # Maps domain to DB row
        self.session.commit()
```

## 3. Main Entry (e.g., main.py or api.py)

The Composition Root. The wiring layer where everything comes together.
Has domain-level imports and external framework/adapter imports.
Reads environment variables.
Injects Dependencies: Instantiates the concrete Adapters and passes them into the pure Domain Use Cases.

```python
# Wiring it all together
db_session = get_session()
repo_adapter = SqlOrderAdapter(db_session)

@app.post("/orders")
def api_create_order(payload: dict):
    # Inject adapter into pure domain
    return create_order(payload, repo=repo_adapter)
```

## 4. The 6 Cross-Cutting Concerns

How to handle common utilities without polluting the pure domain:

### Logging

Treat it like a database. Create a LoggerPort in the domain. Implement it with a Console adapter for local dev, and a JSON/Sentry adapter for production.

### Configuration / Secrets

Resolved entirely in main.py. The domain receives configs as simple function arguments (e.g., max_retries=3), never via os.getenv().

### Caching

Handled via the Decorator Pattern in the Adapters layer. The domain doesn't know it exists.

```python
class RedisCacheAdapter(OrderPort):
    def __init__(self, fallback_sql_adapter: OrderPort):
        self.fallback = fallback_sql_adapter

    def get(self, id):
        if cached := redis.get(id): return cached
        return self.fallback.get(id) # Falls back to SQL
```

### Auth (Authentication & Authorization)

Mechanism: Handled by the API Adapter (e.g., decoding JWTs into a pure User model).
Rules: Handled by the Domain Use Case (e.g., if user.role != "admin": raise Error).

### Telemetry & Metrics

Treated exactly like logging. Define a MetricsPort and implement it with a Datadog or Prometheus Adapter.

### Event Publishing

Treated as a Gateway Port.
The domain calls an EventPublisherPort.publish(), and a Kafka/RabbitMQ adapter executes the actual message delivery.

## 5. Testing

Because the domain is isolated, testing becomes modular.

- Unit Tests (Domain): Pass "Fake Adapters" (in-memory data structures) into Use Cases. Runs in milliseconds.
- Integration Tests (Adapters): Test SQLAlchemy adapters against real test databases.
- End-to-End Tests (Main): Hit the API adapters with real HTTP requests to ensure main.py wired everything correctly.

```python
# Unit Testing with a Fake Adapter
class FakeOrderRepo(OrderPort):
    def __init__(self):
        self.db = {} # In-memory dict instead of SQL
        
    def save(self, order: Order):
        self.db[order.id] = order

def test_create_order():
    fake_repo = FakeOrderRepo()
    create_order({"id": "123"}, repo=fake_repo)
    assert "123" in fake_repo.db
```
