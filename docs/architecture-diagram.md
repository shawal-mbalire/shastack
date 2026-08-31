# Hexagonal Architecture - Visual Overview

## Core Architecture

```mermaid
graph TB
    subgraph "DRIVING ADAPTERS (Trigger the Domain)"
        DA1[FastAPI / Flask]
        DA2[React Hook / Angular]
        DA3[Flutter Widget / Riverpod]
        DA4[Tauri Command]
        DA5[ISR / Button / RTOS]
    end

    subgraph "PORTS (Interfaces)"
        P1[RepositoryPort]
        P2[EventPublisherPort]
        P3[LoggerPort]
        P4[MetricsPort]
    end

    subgraph "DOMAIN (Pure Business Logic)"
        D1[Models]
        D2[Use Cases]
        D3[Errors]
        D4[Rules]
    end

    subgraph "DRIVEN ADAPTERS (Implement Ports)"
        DRV1[Firestore / SQL]
        DRV2[Redis / IndexedDB]
        DRV3[Sentry / Console]
        DRV4[Prometheus / MQTT]
    end

    subgraph "EXTERNAL SERVICES"
        E1[Cloud SQL / Browser Storage]
        E2[Cache / Local DB]
        E3[Logs / Serial]
        E4[Metrics / Events]
    end

    DA1 --> P1
    DA2 --> P1
    DA3 --> P1
    DA4 --> P1
    DA5 --> P1

    P1 --> D1
    P2 --> D1
    P3 --> D1
    P4 --> D1

    D1 --> D2
    D2 --> D3
    D2 --> D4

    D1 --> P1
    D1 --> P2
    D1 --> P3
    D1 --> P4

    P1 --> DRV1
    P2 --> DRV2
    P3 --> DRV3
    P4 --> DRV4

    DRV1 --> E1
    DRV2 --> E2
    DRV3 --> E3
    DRV4 --> E4

    style D1 fill:#4a90d9,stroke:#333,stroke-width:2px
    style D2 fill:#4a90d9,stroke:#333,stroke-width:2px
    style D3 fill:#4a90d9,stroke:#333,stroke-width:2px
    style D4 fill:#4a90d9,stroke:#333,stroke-width:2px
```

## Request Flow

```mermaid
sequenceDiagram
    participant User
    participant DA as Driving Adapter
    participant Port
    participant Domain
    participant DRV as Driven Adapter
    participant Ext as External Service

    User->>DA: Action (HTTP/Click/ISR)
    DA->>DA: Convert DTO to Domain Model
    DA->>Port: Call Use Case
    Port->>Domain: Execute Business Logic
    Domain->>Domain: Apply Rules & Validate
    Domain->>Port: Call Repository/Gateway
    Port->>DRV: Execute via Adapter
    DRV->>Ext: API Call / DB Query
    Ext-->>DRV: Response
    DRV-->>Port: Return Domain Model
    Port-->>Domain: Return Result
    Domain-->>DA: Return Domain Model
    DA-->>User: Convert to DTO/Response
```

## Cross-Cutting Concerns

```mermaid
graph LR
    subgraph "DOMAIN"
        D[Business Logic]
    end

    subgraph "CONCERNS"
        L[Logging]
        C[Configuration]
        Ca[Caching]
        A[Auth]
        T[Telemetry]
        E[Events]
    end

    subgraph "ADAPTERS"
        LA[Console/Sentry]
        CA[SecretManager/Env]
        CaA[Redis/Decorator]
        AA[JWT/Guard]
        TA[Prometheus/Datadog]
        EA[PubSub/MQTT]
    end

    subgraph "EXTERNAL"
        LE[Logs/Metrics]
        CE[Config Store]
        CaE[Cache Store]
        AE[Identity Provider]
        TE[Monitoring]
        EE[Message Broker]
    end

    D -->|LoggerPort| L
    D -->|ConfigPort| C
    D -->|CachePort| Ca
    D -->|User Model| A
    D -->|MetricsPort| T
    D -->|EventPublisherPort| E

    L --> LA
    C --> CA
    Ca --> CaA
    A --> AA
    T --> TA
    E --> EA

    LA --> LE
    CA --> CE
    CaA --> CaE
    AA --> AE
    TA --> TE
    EA --> EE

    style D fill:#4a90d9,stroke:#333,stroke-width:2px
```

## Testing Layers

```mermaid
graph TB
    subgraph "UNIT TESTS (Domain Only)"
        UT[Use Case]
        FA[Fake Adapter / Mock]
        UT -->|Test| FA
    end

    subgraph "INTEGRATION TESTS (Adapters)"
        IT[Adapter]
        RDB[Real DB / API]
        IT -->|Test| RDB
    end

    subgraph "E2E TESTS (Full Stack)"
        ET[API Endpoint]
        FL[All Layers]
        ET -->|Test| FL
    end

    UT -.->|Fast, Isolated| IT
    IT -.->|Real Dependencies| ET

    style UT fill:#4a90d9,stroke:#333
    style IT fill:#7b68ee,stroke:#333
    style ET fill:#9370db,stroke:#333
```

## Stack-Specific Architecture

```mermaid
graph TB
    subgraph "BACKEND (Python/GCloud)"
        B1[FastAPI] --> B2[Domain]
        B2 --> B3[Firestore]
        B2 --> B4[Cloud Tasks]
        B2 --> B5[Cloud SQL]
    end

    subgraph "FRONTEND (React/Angular)"
        F1[Component/Hook] --> F2[Domain]
        F2 --> F3[localStorage]
        F2 --> F4[IndexedDB]
        F2 --> F5[fetch API]
    end

    subgraph "MOBILE (Flutter)"
        M1[Widget/Riverpod] --> M2[Domain]
        M2 --> M3[Geolocator]
        M2 --> M4[Hive]
        M2 --> M5[HTTP Client]
    end

    subgraph "DESKTOP (Tauri)"
        DT1[Tauri Command] --> DT2[Rust Domain]
        DT2 --> DT3[File System]
        DT2 --> DT4[System Tray]
        DT2 --> DT5[IPC to Angular]
    end

    subgraph "EMBEDDED (ESP32)"
        E1[ISR/Button] --> E2[Domain]
        E2 --> E3[GPIO/I2C]
        E2 --> E4[MQTT]
        E2 --> E5[WiFi]
    end

    style B2 fill:#4a90d9,stroke:#333
    style F2 fill:#4a90d9,stroke:#333
    style M2 fill:#4a90d9,stroke:#333
    style DT2 fill:#4a90d9,stroke:#333
    style E2 fill:#4a90d9,stroke:#333
```

## Dependency Injection Flow

```mermaid
graph TB
    subgraph "COMPOSITION ROOT (main.py / app.config.ts)"
        CR[Read Config]
        CR --> CA[Create Adapters]
        CR --> IN[Inject into Use Cases]
    end

    subgraph "INSTANTIATION"
        A1[FirestoreAdapter]
        A2[RedisCacheAdapter]
        A3[SentryLogger]
        A4[PrometheusMetrics]
    end

    subgraph "INJECTION"
        UC1[Use Case 1]
        UC2[Use Case 2]
        UC3[Use Case 3]
    end

    CA --> A1
    CA --> A2
    CA --> A3
    CA --> A4

    IN --> UC1
    IN --> UC2
    IN --> UC3

    A1 -.->|Implements| UC1
    A2 -.->|Wraps| UC1
    A3 -.->|Logs| UC2
    A4 -.->|Metrics| UC3

    style CR fill:#4a90d9,stroke:#333
    style A1 fill:#7b68ee,stroke:#333
    style A2 fill:#7b68ee,stroke:#333
    style A3 fill:#7b68ee,stroke:#333
    style A4 fill:#7b68ee,stroke:#333
```

## ESP32 Hexagonal Flow

```mermaid
graph TB
    subgraph "DRIVING (Hardware)"
        ISR[Interrupt - Button]
        RTOS[RTOS Task]
        LOOP[Main Loop]
    end

    subgraph "PORTS"
        RP[RelayPort]
        SP[SensorPort]
        WP[WiFiPort]
    end

    subgraph "DOMAIN (Pure)"
        PC[PumpController]
        SC[SensorController]
        UC[Use Cases]
    end

    subgraph "DRIVEN (Hardware)"
        GPIO[GPIO Adapter]
        I2C[I2C Adapter]
        WIFI[WiFi Adapter]
        MQTT[MQTT Adapter]
    end

    subgraph "HARDWARE"
        PIN[Pin 14 - Relay]
        SENS[DHT22 / BMP280]
        NET[WiFi Module]
        BROKER[MQTT Broker]
    end

    ISR --> RP
    RTOS --> SP
    LOOP --> WP

    RP --> PC
    SP --> SC
    WP --> UC

    PC --> GPIO
    SC --> I2C
    UC --> WIFI
    UC --> MQTT

    GPIO --> PIN
    I2C --> SENS
    WIFI --> NET
    MQTT --> BROKER

    style PC fill:#4a90d9,stroke:#333
    style SC fill:#4a90d9,stroke:#333
    style UC fill:#4a90d9,stroke:#333
```
