# Frontend: Angular

Hexagonal architecture for Angular frontend with dependency injection.

## Port (Pure TypeScript)

```typescript
export interface UserRepository {
  getUser(id: string): Promise<User>;
}
```

## Driven Adapter (Angular HttpClient)

```typescript
import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { firstValueFrom } from 'rxjs';

@Injectable()
export class HttpUserAdapter implements UserRepository {
  constructor(private http: HttpClient) {}

  async getUser(id: string): Promise<User> {
    const dto = await firstValueFrom(this.http.get<UserDto>(`/api/users/${id}`));
    return mapDtoToDomain(dto);
  }
}
```

## Driving Adapter (Angular Component)

```typescript
import { Component } from '@angular/core';

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
```

## Composition Root (app.config.ts)

```typescript
import { ApplicationConfig } from '@angular/core';

export const appConfig: ApplicationConfig = {
  providers: [{ provide: UserRepository, useClass: HttpUserAdapter }]
};
```

## Cross-Cutting Concerns

### Logging

```typescript
// Port (Domain)
export interface Logger {
  info(msg: string): void;
  error(msg: string): void;
}

// Adapter (Console)
@Injectable()
export class ConsoleLogger implements Logger {
  info(msg: string) { console.log(`[INFO] ${msg}`); }
  error(msg: string) { console.error(`[ERROR] ${msg}`); }
}

// Adapter (Sentry)
@Injectable()
export class SentryLogger implements Logger {
  constructor(private sentry: SentryService) {}
  info(msg: string) { this.sentry.captureMessage(msg); }
  error(msg: string) { this.sentry.captureException(new Error(msg)); }
}
```

### Configuration / Secrets

```typescript
// Adapter resolves config, domain receives as arguments
@Injectable({ providedIn: 'root' })
export class ConfigService {
  readonly apiUrl = environment.apiUrl;
  readonly maxRetries = environment.maxRetries;
}

// Domain use case receives config as params
export class GetUserUseCase {
  constructor(
    private userRepo: UserRepository,
    private maxRetries: number // Injected, never reads env
  ) {}
}
```

### Caching (Decorator Pattern)

```typescript
@Injectable()
export class CacheUserAdapter implements UserRepository {
  constructor(
    @Inject('UserRepository') private fallback: UserRepository,
    private cache: Map<string, User>
  ) {}

  async getUser(id: string): Promise<User> {
    if (this.cache.has(id)) return this.cache.get(id)!;
    const user = await this.fallback.getUser(id);
    this.cache.set(id, user);
    return user;
  }
}
```

### Auth

```typescript
// Driving Adapter decodes JWT
@Injectable()
export class AuthGuard implements CanActivate {
  constructor(private authAdapter: AuthAdapter) {}

  canActivate(): boolean {
    const user = this.authAdapter.getUser();
    return user?.role === 'admin';
  }
}

// Domain enforces rules
export function deleteUser(user: User, currentUser: User): void {
  if (currentUser.role !== 'admin') throw new Error('Admin only');
  if (user.id === currentUser.id) throw new Error('Cannot delete self');
}
```

### Telemetry & Metrics

```typescript
// Port (Domain)
export interface Metrics {
  increment(name: string, tags?: Record<string, string>): void;
  histogram(name: string, value: number): void;
}

// Adapter (Datadog)
@Injectable()
export class DatadogMetrics implements Metrics {
  constructor(private dd: DatadogService) {}
  increment(name, tags) { this.dd.increment(name, tags); }
  histogram(name, value) { this.dd.histogram(name, value); }
}
```

### Event Publishing

```typescript
// Port (Domain)
export interface EventPublisher {
  publish(event: DomainEvent): void;
}

// Adapter (GTM)
@Injectable()
export class GTMEventPublisher implements EventPublisher {
  publish(event: DomainEvent): void {
    window.dataLayer?.push({ event: event.type, ...event.data });
  }
}
```

## Localized Concerns (Browser)

### Service Worker (Offline Support)

```typescript
// ngsw-config.json
{
  "index": "/index.html",
  "assetGroups": [{
    "name": "app-shell",
    "installMode": "prefetch",
    "resources": { "files": ["/**/*.css", "/**/*.js"] }
  }]
}

// Adapter (Angular Service Worker)
@Injectable()
export class OfflineAdapter implements CartRepository {
  constructor(private ngsw: SwPush, private storage: StorageMap) {}

  async saveCart(cart: Cart): Promise<void> {
    await this.storage.set('pending_cart', cart);
    if (navigator.onLine) {
      await this.syncCart(cart);
    }
  }
}
```

### IndexedDB (Persistent Storage)

```typescript
// Adapter (Angular - idb via ngx-indexed-db)
@Injectable()
export class IndexedDBAdapter implements CartRepository {
  constructor(private db: NgxIndexedDBService) {}

  async saveCart(cart: Cart): Promise<void> {
    await this.db.update('carts', cart);
  }

  async getCart(id: string): Promise<Cart | null> {
    return firstValueFrom(this.db.getByID('carts', id));
  }
}
```

### Cache API (HTTP Caching)

```typescript
// Adapter (Angular HTTP Interceptor)
@Injectable()
export class HttpCacheInterceptor implements HttpInterceptor {
  private cache = new Map<string, HttpResponse<any>>();

  intercept(req: HttpRequest<any>, next: HttpHandler): Observable<HttpEvent<any>> {
    if (req.method !== 'GET') return next.handle(req);

    const cached = this.cache.get(req.url);
    if (cached) return of(cached);

    return next.handle(req).pipe(
      tap(event => {
        if (event instanceof HttpResponse) {
          this.cache.set(req.url, event);
        }
      })
    );
  }
}
```

### Web Push Notifications

```typescript
// Adapter (Angular - SwPush)
@Injectable()
export class PushNotificationAdapter {
  constructor(private swPush: SwPush) {}

  async requestPermission(): Promise<boolean> {
    const permission = await Notification.requestPermission();
    return permission === 'granted';
  }

  async subscribe(): Promise<PushSubscription> {
    return this.swPush.requestSubscription({ serverPublicKey: VAPID_KEY });
  }
}
```

### Internationalization (i18n)

```typescript
// Adapter (Angular - @ngx-translate)
import { TranslateService } from '@ngx-translate/core';

@Injectable()
export class I18nAdapter implements Translator {
  constructor(private translate: TranslateService) {}

  translate(key: string, params?: Record<string, string>): string {
    return this.translate.instant(key, params);
  }
}
```

### Clipboard API

```typescript
// Adapter (Clipboard)
@Injectable()
export class ClipboardAdapter {
  async copy(text: string): Promise<void> {
    await navigator.clipboard.writeText(text);
  }

  async paste(): Promise<string> {
    return navigator.clipboard.readText();
  }
}
```
