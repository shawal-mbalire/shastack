# Backend: Python & Google Cloud

Hexagonal architecture for Python backend with FastAPI and GCloud services.

## Port (Pure Python)

```python
from typing import Protocol
from dataclasses import dataclass

@dataclass
class Document:
    id: str
    content: str

class DocumentRepository(Protocol):
    def save(self, doc: Document) -> None: ...
```

## Driven Adapter (GCloud Firestore)

```python
from google.cloud import firestore

class FirestoreDocAdapter(DocumentRepository):
    def __init__(self, collection_name: str):
        self.db = firestore.Client()
        self.collection = self.db.collection(collection_name)

    def save(self, doc: Document) -> None:
        self.collection.document(doc.id).set({"content": doc.content})
```

## Use Case (Pure Python)

```python
def create_document(doc_id: str, content: str, repo: DocumentRepository) -> Document:
    doc = Document(id=doc_id, content=content)
    repo.save(doc)
    return doc
```

## Driving Adapter & Composition Root (FastAPI)

```python
from fastapi import FastAPI

app = FastAPI()
firestore_repo = FirestoreDocAdapter("documents")

@app.post("/docs/{doc_id}")
def api_create_document(doc_id: str, content: dict):
    return create_document(doc_id, content["text"], repo=firestore_repo)
```

## Cross-Cutting Concerns

### Logging

```python
# Port (Domain)
class LoggerPort(Protocol):
    def info(self, msg: str) -> None: ...
    def error(self, msg: str) -> None: ...

# Adapter (Console for dev)
class ConsoleLogger:
    def info(self, msg: str) -> None: print(f"[INFO] {msg}")
    def error(self, msg: str) -> None: print(f"[ERROR] {msg}")

# Adapter (Sentry for prod)
class SentryLogger:
    def __init__(self, dsn: str): sentry_sdk.init(dsn)
    def info(self, msg: str) -> None: sentry_sdk.capture_message(msg)
    def error(self, msg: str) -> None: sentry_sdk.capture_exception(Exception(msg))
```

### Configuration / Secrets

```python
# main.py - Resolved here, never in domain
import os
from google.cloud import secretmanager

def get_secret(name: str) -> str:
    client = secretmanager.SecretManagerServiceClient()
    return client.access_secret_version(name).payload.data.decode()

# Domain receives as arguments
firestore_repo = FirestoreDocAdapter(collection_name=get_secret("COLLECTION_NAME"))
```

### Caching (Decorator Pattern)

```python
class CacheAdapter(DocumentRepository):
    def __init__(self, fallback: DocumentRepository, cache_client):
        self.fallback = fallback
        self.cache = cache_client

    def get(self, doc_id: str) -> Document:
        if cached := self.cache.get(doc_id):
            return cached
        doc = self.fallback.get(doc_id)
        self.cache.set(doc_id, doc)
        return doc
```

### Auth

```python
# Driving Adapter decodes JWT into pure User model
from fastapi import Depends, HTTPException
from fastapi.security import HTTPBearer

security = HTTPBearer()

def get_current_user(token: str = Depends(security)) -> User:
    payload = jwt.decode(token.credentials, SECRET_KEY, algorithms=["HS256"])
    return User(id=payload["sub"], role=payload["role"])

# Domain enforces rules
def create_document(doc_id: str, content: str, user: User, repo: DocumentRepository):
    if user.role != "admin":
        raise PermissionError("Admin only")
    ...
```

### Telemetry & Metrics

```python
# Port (Domain)
class MetricsPort(Protocol):
    def increment(self, name: str, tags: dict = None) -> None: ...
    def histogram(self, name: str, value: float) -> None: ...

# Adapter (Prometheus)
from prometheus_client import Counter, Histogram

class PrometheusMetrics:
    def increment(self, name: str, tags: dict = None) -> None:
        Counter(name, labelnames=tags or {}).inc()
    def histogram(self, name: str, value: float) -> None:
        Histogram(name).observe(value)
```

### Event Publishing

```python
# Port (Domain)
class EventPublisherPort(Protocol):
    def publish(self, event: DomainEvent) -> None: ...

# Adapter (Pub/Sub)
from google.cloud import pubsub_v1

class PubSubAdapter:
    def __init__(self, topic: str):
        self.publisher = pubsub_v1.PublisherClient()
        self.topic = topic

    def publish(self, event: DomainEvent) -> None:
        self.publisher.publish(self.topic, json.dumps(event.__dict__).encode())
```

## Localized Concerns (GCloud)

### Cloud SQL (PostgreSQL)

```python
# Driven Adapter (Cloud SQL via SQLAlchemy)
from sqlalchemy import create_engine
from sqlalchemy.orm import sessionmaker

class CloudSQLAdapter(DocumentRepository):
    def __init__(self, connection_name: str):
        # Cloud SQL Auth Proxy connection
        engine = create_engine(f"postgresql+psycopg2://user:pass@/{db_name}",
                               connect_args={"host": connection_name})
        self.Session = sessionmaker(bind=engine)

    def save(self, doc: Document) -> None:
        with self.Session() as session:
            session.add(DBDocument(id=doc.id, content=doc.content))
            session.commit()
```

### Cloud Tasks (Async Jobs)

```python
# Driven Adapter (Cloud Tasks)
from google.cloud import tasks_v2

class CloudTasksAdapter:
    def __init__(self, queue: str, location: str):
        self.client = tasks_v2.CloudTasksClient()
        self.queue = self.client.queue_path("project", location, queue)

    def enqueue(self, task_name: str, payload: dict) -> None:
        task = {
            "http_request": {
                "http_method": "POST",
                "url": f"https://worker.run.app/{task_name}",
                "body": json.dumps(payload).encode(),
            }
        }
        self.client.create_task(parent=self.queue, task=task)
```

### Cloud Run (Serverless Deployment)

```yaml
# Dockerfile for Cloud Run
FROM python:3.11-slim
WORKDIR /app
COPY requirements.txt .
RUN pip install -r requirements.txt
COPY . .
CMD ["uvicorn", "main:app", "--host", "0.0.0.0", "--port", "8080"]
```

### IAM (Service Accounts)

```python
# Adapter (IAM - Service Account impersonation)
from google.auth import impersonated_credentials

def get_impersonated_token(target_sa: str):
    creds, _ = google.auth.default()
    target_scopes = ["https://www.googleapis.com/auth/cloud-platform"]
    return impersonated_credentials.Credentials(
        source_credentials=creds,
        target_principal=target_sa,
        target_scopes=target_scopes,
    )
```

### Memorystore (Redis)

```python
# Driven Adapter (Redis Cache)
import redis

class RedisCacheAdapter:
    def __init__(self, host: str, port: int = 6379):
        self.client = redis.Redis(host=host, port=port, decode_responses=True)

    def get(self, key: str):
        return self.client.get(key)

    def set(self, key: str, value, expiry: int = 3600):
        self.client.setex(key, expiry, value)
```

### Cloud Storage (GCS)

```python
# Driven Adapter (Cloud Storage)
from google.cloud import storage

class GCSAdapter:
    def __init__(self, bucket: str):
        self.client = storage.Client()
        self.bucket = self.client.bucket(bucket)

    def upload(self, blob_name: str, data: bytes):
        blob = self.bucket.blob(blob_name)
        blob.upload_from_string(data)

    def download(self, blob_name: str) -> bytes:
        blob = self.bucket.blob(blob_name)
        return blob.download_as_bytes()
```
