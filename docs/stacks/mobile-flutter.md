# Mobile: Flutter

Hexagonal architecture for Flutter mobile with Riverpod state management.

## Port (Pure Dart)

```dart
abstract class LocationPort {
  Future<Coordinates> getCurrentLocation();
}

class TrackLocationUseCase {
  final LocationPort locationPort;
  TrackLocationUseCase(this.locationPort);

  Future<Coordinates> execute() async {
    return await locationPort.getCurrentLocation();
  }
}
```

## Driven Adapter (Hardware Specific)

```dart
import 'package:geolocator/geolocator.dart';

class GeolocatorAdapter implements LocationPort {
  @override
  Future<Coordinates> getCurrentLocation() async {
    final pos = await Geolocator.getCurrentPosition();
    return Coordinates(lat: pos.latitude, lng: pos.longitude);
  }
}
```

## Driving Adapter (Riverpod State Controller)

```dart
import 'package:flutter_riverpod/flutter_riverpod.dart';

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
```

## Driving Adapter (Flutter Widget)

```dart
import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

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

## Cross-Cutting Concerns

### Logging

```dart
// Port (Domain)
abstract class Logger {
  void info(String msg);
  void error(String msg);
}

// Adapter (Console)
class ConsoleLogger implements Logger {
  @override
  void info(String msg) => print('[INFO] $msg');
  @override
  void error(String msg) => print('[ERROR] $msg');
}

// Adapter (Sentry)
class SentryLogger implements Logger {
  @override
  void info(String msg) => Sentry.captureMessage(msg);
  @override
  void error(String msg) => Sentry.captureException(Exception(msg));
}
```

### Configuration / Secrets

```dart
// Adapter resolves config, domain receives as arguments
class ConfigService {
  final String apiUrl;
  final int maxRetries;
  ConfigService({required this.apiUrl, required this.maxRetries});
}

// Domain use case receives config as params
class GetUserUseCase {
  final UserRepository userRepo;
  final int maxRetries;
  GetUserUseCase(this.userRepo, this.maxRetries);
}
```

### Caching (Decorator Pattern)

```dart
class CacheUserAdapter implements UserRepository {
  final UserRepository fallback;
  final Map<String, User> cache;
  CacheUserAdapter(this.fallback, this.cache);

  @override
  Future<User> getUser(String id) async {
    if (cache.containsKey(id)) return cache[id]!;
    final user = await fallback.getUser(id);
    cache[id] = user;
    return user;
  }
}
```

### Auth

```dart
// Driving Adapter decodes token
class AuthAdapter {
  User? getUser() {
    final token = localStorage.getItem('token');
    if (token == null) return null;
    return JwtDecoder.decode(token);
  }
}

// Domain enforces rules
void deleteUser(User user, User currentUser) {
  if (currentUser.role != 'admin') throw Exception('Admin only');
  if (user.id == currentUser.id) throw Exception('Cannot delete self');
}
```

### Telemetry & Metrics

```dart
// Port (Domain)
abstract class Metrics {
  void increment(String name, {Map<String, String>? tags});
  void histogram(String name, double value);
}

// Adapter (Firebase Analytics)
class FirebaseMetrics implements Metrics {
  @override
  void increment(String name, {Map<String, String>? tags}) {
    FirebaseAnalytics.instance.logEvent(name: name, parameters: tags);
  }
  @override
  void histogram(String name, double value) {
    FirebaseAnalytics.instance.logEvent(name: name, parameters: {'value': value});
  }
}
```

### Event Publishing

```dart
// Port (Domain)
abstract class EventPublisher {
  void publish(DomainEvent event);
}

// Adapter (Firebase Analytics)
class FirebaseEventPublisher implements EventPublisher {
  @override
  void publish(DomainEvent event) {
    FirebaseAnalytics.instance.logEvent(
      name: event.type,
      parameters: event.data,
    );
  }
}
```

## Localized Concerns (Mobile)

### Push Notifications

```dart
// Adapter (Firebase Cloud Messaging)
import 'package:firebase_messaging/firebase_messaging.dart';

class PushNotificationAdapter {
  final FirebaseMessaging _messaging = FirebaseMessaging.instance;

  Future<void> initialize() async {
    final settings = await _messaging.requestPermission();
    if (settings.authorizationStatus == AuthorizationStatus.authorized) {
      final token = await _messaging.getToken();
      // Register token with backend
    }
  }

  void onMessage(void Function(RemoteMessage) handler) {
    FirebaseMessaging.onMessage.listen(handler);
  }
}
```

### Biometrics (Face ID / Fingerprint)

```dart
// Adapter (local_auth)
import 'package:local_auth/local_auth.dart';

class BiometricsAdapter {
  final LocalAuthentication _auth = LocalAuthentication();

  Future<bool> isAvailable() async {
    return await _auth.canCheckBiometrics;
  }

  Future<bool> authenticate() async {
    return await _auth.authenticate(
      localizedReason: 'Authenticate to continue',
      options: const AuthenticationOptions(biometricOnly: true),
    );
  }
}
```

### Deep Links

```dart
// Adapter (uni_links)
import 'package:uni_links/uni_links.dart';

class DeepLinkAdapter {
  Stream<Uri> get linkStream => uriLinkStream;

  Future<void> initialize() async {
    final initialLink = await getInitialUri();
    if (initialLink != null) {
      _handleLink(initialLink);
    }
  }

  void _handleLink(Uri uri) {
    // Route based on uri.path
  }
}
```

### App Lifecycle

```dart
// Adapter (WidgetsBindingObserver)
class AppLifecycleAdapter extends WidgetsBindingObserver {
  void Function(AppLifecycleState)? onStateChanged;

  @override
  void didChangeAppLifecycleState(AppLifecycleState state) {
    onStateChanged?.call(state);
  }

  void attach() => WidgetsBinding.instance.addObserver(this);
  void detach() => WidgetsBinding.instance.removeObserver(this);
}
```

### Offline Storage (Hive/Drift)

```dart
// Adapter (Hive)
import 'package:hive_flutter/hive_flutter.dart';

class HiveAdapter implements CartRepository {
  late Box<Cart> _box;

  Future<void> init() async {
    await Hive.initFlutter();
    _box = await Hive.openBox<Cart>('carts');
  }

  @override
  Future<void> saveCart(Cart cart) async {
    await _box.put(cart.id, cart);
  }

  @override
  Future<Cart?> getCart(String id) async {
    return _box.get(id);
  }
}
```

### Connectivity

```dart
// Adapter (connectivity_plus)
import 'package:connectivity_plus/connectivity_plus.dart';

class ConnectivityAdapter {
  final Connectivity _connectivity = Connectivity();

  Stream<ConnectivityResult> get onChanged => _connectivity.onConnectivityChanged;

  Future<bool> isConnected() async {
    final result = await _connectivity.checkConnectivity();
    return result != ConnectivityResult.none;
  }
}
```
