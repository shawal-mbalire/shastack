# Frontend: React

Hexagonal architecture for React frontend with state management hooks.

## Port (Pure TypeScript)

```typescript
export interface CartRepository {
  saveCart(cart: Cart): Promise<void>;
}
```

## Driven Adapter (Infrastructure)

```typescript
export class LocalStorageCartAdapter implements CartRepository {
  async saveCart(cart: Cart): Promise<void> {
    localStorage.setItem('cart', JSON.stringify(cart));
  }
}
```

## Use Case (Pure TypeScript)

```typescript
export class AddToCartUseCase {
  constructor(private cartRepo: CartRepository) {}

  async execute(productId: string, quantity: number) {
    // pure business logic
    await this.cartRepo.saveCart(cart);
    return cart;
  }
}
```

## Driving Adapter (React Component & Hook)

```typescript
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

## Cross-Cutting Concerns

### Logging

```typescript
// Port (Domain)
interface Logger {
  info(msg: string): void;
  error(msg: string): void;
}

// Adapter (Console)
const consoleLogger: Logger = {
  info: (msg) => console.log(`[INFO] ${msg}`),
  error: (msg) => console.error(`[ERROR] ${msg}`),
};

// Adapter (Sentry)
const sentryLogger: Logger = {
  info: (msg) => Sentry.captureMessage(msg),
  error: (msg) => Sentry.captureException(new Error(msg)),
};
```

### Configuration / Secrets

```typescript
// Adapter resolves config, domain receives as arguments
const API_URL = import.meta.env.VITE_API_URL;

// Domain use case receives config as params
class AddToCartUseCase {
  constructor(
    private cartRepo: CartRepository,
    private maxItems: number // Config passed in, never read env directly
  ) {}
}
```

### Caching (Decorator Pattern)

```typescript
class CacheCartAdapter implements CartRepository {
  constructor(
    private fallback: CartRepository,
    private cache: Map<string, Cart>
  ) {}

  async getCart(id: string): Promise<Cart> {
    if (this.cache.has(id)) return this.cache.get(id)!;
    const cart = await this.fallback.getCart(id);
    this.cache.set(id, cart);
    return cart;
  }
}
```

### Auth

```typescript
// Driving Adapter decodes token
const authAdapter = {
  getUser: (): User | null => {
    const token = localStorage.getItem('token');
    if (!token) return null;
    return jwtDecode<User>(token);
  },
};

// Domain enforces rules
function addToCart(productId: string, user: User, cart: Cart): Cart {
  if (!user.isAuthenticated) throw new Error('Not authenticated');
  if (cart.items.length >= MAX_ITEMS) throw new Error('Cart full');
  return { ...cart, items: [...cart.items, productId] };
}
```

### Telemetry & Metrics

```typescript
// Port (Domain)
interface Metrics {
  increment(name: string, tags?: Record<string, string>): void;
  histogram(name: string, value: number): void;
}

// Adapter (PostHog / Datadog)
const postHogMetrics: Metrics = {
  increment: (name, tags) => posthog.capture(name, tags),
  histogram: (name, value) => posthog.capture(name, { value }),
};
```

### Event Publishing

```typescript
// Port (Domain)
interface EventPublisher {
  publish(event: DomainEvent): void;
}

// Adapter (Analytics / GTM)
class GTMEventPublisher implements EventPublisher {
  publish(event: DomainEvent): void {
    window.dataLayer?.push({ event: event.type, ...event.data });
  }
}
```

## Localized Concerns (Browser)

### Service Worker (Offline Support)

```typescript
// sw.ts - Workbox
import { precacheAndRoute } from 'workbox-precaching';

precacheAndRoute(self.__WB_MANIFEST);

// Adapter for offline-first
class OfflineAdapter implements CartRepository {
  async saveCart(cart: Cart): Promise<void> {
    localStorage.setItem('pending_cart', JSON.stringify(cart));
    // Sync when online
    if (navigator.onLine) {
      await fetch('/api/cart', { method: 'POST', body: JSON.stringify(cart) });
    }
  }
}
```

### IndexedDB (Persistent Storage)

```typescript
// Adapter (IndexedDB via idb)
import { openDB } from 'idb';

const dbPromise = openDB('app-store', 1, {
  upgrade(db) { db.createObjectStore('carts'); },
});

class IndexedDBAdapter implements CartRepository {
  async saveCart(cart: Cart): Promise<void> {
    const db = await dbPromise;
    await db.put('carts', cart, cart.id);
  }
  async getCart(id: string): Promise<Cart | null> {
    const db = await dbPromise;
    return db.get('carts', id);
  }
}
```

### Cache API (HTTP Caching)

```typescript
// Adapter (Cache API)
class HttpCacheAdapter {
  private cacheName = 'api-cache';

  async fetchWithCache(url: string): Promise<Response> {
    const cache = await caches.open(this.cacheName);
    const cached = await cache.match(url);
    if (cached) return cached;

    const response = await fetch(url);
    cache.put(url, response.clone());
    return response;
  }
}
```

### Web Push Notifications

```typescript
// Adapter (Push API)
class PushNotificationAdapter {
  async requestPermission(): Promise<boolean> {
    const permission = await Notification.requestPermission();
    return permission === 'granted';
  }

  async subscribe(): Promise<PushSubscription> {
    const registration = await navigator.serviceWorker.ready;
    return registration.pushManager.subscribe({
      userVisibleOnly: true,
      applicationServerKey: VAPID_KEY,
    });
  }
}
```

### Internationalization (i18n)

```typescript
// Port (Domain)
interface Translator {
  translate(key: string, params?: Record<string, string>): string;
}

// Adapter (i18next)
import i18n from 'i18next';

class I18NextAdapter implements Translator {
  translate(key: string, params?: Record<string, string>): string {
    return i18n.t(key, params);
  }
}
```

### Clipboard API

```typescript
// Adapter (Clipboard)
class ClipboardAdapter {
  async copy(text: string): Promise<void> {
    await navigator.clipboard.writeText(text);
  }

  async paste(): Promise<string> {
    return navigator.clipboard.readText();
  }
}
```
