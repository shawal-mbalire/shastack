use anyhow::Result;
use std::fs;
use std::path::Path;

const WEB_CLIENT_PACKAGE_JSON: &str = r##"{
  "name": "client",
  "version": "0.0.0",
  "scripts": {
    "ng": "ng",
    "start": "ng serve",
    "build": "ng build",
    "test": "ng test",
    "lint": "ng lint"
  },
  "dependencies": {
    "@angular/animations": "^18.0.0",
    "@angular/common": "^18.0.0",
    "@angular/compiler": "^18.0.0",
    "@angular/core": "^18.0.0",
    "@angular/forms": "^18.0.0",
    "@angular/platform-browser": "^18.0.0",
    "@angular/platform-browser-dynamic": "^18.0.0",
    "@angular/router": "^18.0.0",
    "rxjs": "~7.8.0",
    "tslib": "^2.3.0",
    "zone.js": "~0.14.3"
  },
  "devDependencies": {
    "@angular-devkit/build-angular": "^18.0.0",
    "@angular/cli": "^18.0.0",
    "@angular/compiler-cli": "^18.0.0",
    "typescript": "~5.4.0"
  }
}
"##;

const WEB_CLIENT_TSCONFIG_JSON: &str = r##"{
  "compileOnSave": false,
  "compilerOptions": {
    "baseUrl": "./",
    "outDir": "./dist/out-tsc",
    "forceConsistentCasingInFileNames": true,
    "strict": true,
    "noImplicitOverride": true,
    "noPropertyAccessFromIndexSignature": true,
    "noImplicitReturns": true,
    "noFallthroughCasesInSwitch": true,
    "esModuleInterop": true,
    "sourceMap": true,
    "declaration": false,
    "experimentalDecorators": true,
    "moduleResolution": "bundler",
    "importHelpers": true,
    "target": "ES2022",
    "module": "ES2022",
    "useDefineForClassFields": false,
    "lib": ["ES2022", "dom"]
  },
  "angularCompilerOptions": {
    "enableI18nLegacyMessageIdFormat": false,
    "strictInjectionParameters": true,
    "strictInputAccessModifiers": true,
    "strictTemplates": true
  }
}
"##;

const WEB_CLIENT_INDEX_HTML: &str = r##"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>App</title>
  <base href="/">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <link rel="icon" type="image/x-icon" href="favicon.ico">
</head>
<body>
  <app-root></app-root>
</body>
</html>
"##;

const WEB_CLIENT_MAIN_TS: &str = r##"import { bootstrapApplication } from '@angular/platform-browser';
import { appConfig } from './app/app.config';
import { AppComponent } from './app/app.component';

bootstrapApplication(AppComponent, appConfig)
  .catch((err) => console.error(err));
"##;

const WEB_CLIENT_STYLES_SCSS: &str = r##"*, *::before, *::after {
  box-sizing: border-box;
}

body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}
"##;

const WEB_CLIENT_APP_COMPONENT_TS: &str = r##"import { Component } from '@angular/core';
import { RouterOutlet } from '@angular/router';

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [RouterOutlet],
  template: '<router-outlet></router-outlet>',
})
export class AppComponent {}
"##;

const WEB_CLIENT_APP_CONFIG_TS: &str = r##"import { ApplicationConfig } from '@angular/core';
import { provideRouter } from '@angular/router';
import { provideHttpClient, withInterceptors } from '@angular/common/http';
import { routes } from './app.routes';
import { authInterceptor } from './core/interceptors/auth.interceptor';

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(routes),
    provideHttpClient(withInterceptors([authInterceptor])),
  ],
};
"##;

const WEB_CLIENT_APP_ROUTES_TS: &str = r##"import { Routes } from '@angular/router';
import { authGuard } from './core/auth/auth.guard';

export const routes: Routes = [
  {
    path: '',
    redirectTo: 'dashboard',
    pathMatch: 'full',
  },
  {
    path: 'login',
    loadComponent: () =>
      import('./features/auth/login.component').then((m) => m.LoginComponent),
  },
  {
    path: 'dashboard',
    canActivate: [authGuard],
    loadComponent: () =>
      import('./features/shell/shell.component').then((m) => m.ShellComponent),
    loadChildren: () =>
      import('./features/shell/shell.routes').then((m) => m.shellRoutes),
  },
  { path: '**', redirectTo: 'login' },
];
"##;

const WEB_CLIENT_AUTH_SERVICE_TS: &str = r##"import { Injectable, signal } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { tap } from 'rxjs/operators';
import { environment } from '../../../environments/environment';

export interface AuthUser {
  id: string;
  email: string;
  roles: string[];
}

@Injectable({ providedIn: 'root' })
export class AuthService {
  private readonly TOKEN_KEY = 'jwt_token';
  readonly currentUser = signal<AuthUser | null>(null);

  constructor(private http: HttpClient) {
    const token = this.getToken();
    if (token) {
      this.loadUser();
    }
  }

  login(email: string, password: string) {
    return this.http
      .post<{ token: string; user: AuthUser }>(`${environment.apiUrl}/auth/login`, { email, password })
      .pipe(
        tap(({ token, user }) => {
          localStorage.setItem(this.TOKEN_KEY, token);
          this.currentUser.set(user);
        })
      );
  }

  logout() {
    localStorage.removeItem(this.TOKEN_KEY);
    this.currentUser.set(null);
  }

  getToken(): string | null {
    return localStorage.getItem(this.TOKEN_KEY);
  }

  hasRole(role: string): boolean {
    return this.currentUser()?.roles.includes(role) ?? false;
  }

  private loadUser() {
    this.http.get<AuthUser>(`${environment.apiUrl}/auth/me`).subscribe({
      next: (user) => this.currentUser.set(user),
      error: () => this.logout(),
    });
  }
}
"##;

const WEB_CLIENT_AUTH_GUARD_TS: &str = r##"import { inject } from '@angular/core';
import { CanActivateFn, Router } from '@angular/router';
import { AuthService } from './auth.service';

export const authGuard: CanActivateFn = () => {
  const auth = inject(AuthService);
  const router = inject(Router);

  if (auth.getToken()) {
    return true;
  }

  return router.parseUrl('/login');
};
"##;

const WEB_CLIENT_AUTH_INTERCEPTOR_TS: &str = r##"import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import { AuthService } from '../auth/auth.service';

export const authInterceptor: HttpInterceptorFn = (req, next) => {
  const auth = inject(AuthService);
  const token = auth.getToken();

  if (token) {
    const authReq = req.clone({
      setHeaders: { Authorization: `Bearer ${token}` },
    });
    return next(authReq);
  }

  return next(req);
};
"##;

const WEB_CLIENT_SHELL_COMPONENT_TS: &str = r##"import { Component } from '@angular/core';
import { RouterLink, RouterOutlet } from '@angular/router';
import { AuthService } from '../../core/auth/auth.service';

@Component({
  selector: 'app-shell',
  standalone: true,
  imports: [RouterOutlet, RouterLink],
  template: `
    <nav>
      <span>Dashboard</span>
      <button (click)="auth.logout()">Logout</button>
    </nav>
    <main>
      <router-outlet></router-outlet>
    </main>
  `,
})
export class ShellComponent {
  constructor(public auth: AuthService) {}
}
"##;

const WEB_CLIENT_SHELL_ROUTES_TS: &str = r##"import { Routes } from '@angular/router';

export const shellRoutes: Routes = [
  {
    path: '',
    redirectTo: 'home',
    pathMatch: 'full',
  },
  {
    path: 'home',
    loadComponent: () =>
      import('../home/home.component').then((m) => m.HomeComponent),
  },
];
"##;

const WEB_CLIENT_LOGIN_COMPONENT_TS: &str = r##"import { NgIf } from '@angular/common';
import { Component } from '@angular/core';
import { FormBuilder, ReactiveFormsModule, Validators } from '@angular/forms';
import { Router } from '@angular/router';
import { AuthService } from '../../core/auth/auth.service';

@Component({
  selector: 'app-login',
  standalone: true,
  imports: [ReactiveFormsModule, NgIf],
  template: `
    <form [formGroup]="form" (ngSubmit)="submit()">
      <input formControlName="email" type="email" placeholder="Email" />
      <input formControlName="password" type="password" placeholder="Password" />
      <button type="submit" [disabled]="form.invalid">Sign In</button>
      <p *ngIf="error" style="color:red">{{ error }}</p>
    </form>
  `,
})
export class LoginComponent {
  form = this.fb.group({
    email: ['', [Validators.required, Validators.email]],
    password: ['', Validators.required],
  });
  error = '';

  constructor(
    private fb: FormBuilder,
    private auth: AuthService,
    private router: Router,
  ) {}

  submit() {
    if (this.form.invalid) return;
    const { email, password } = this.form.value;
    this.auth.login(email!, password!).subscribe({
      next: () => this.router.navigate(['/dashboard']),
      error: () => (this.error = 'Invalid credentials'),
    });
  }
}
"##;

const WEB_CLIENT_HOME_COMPONENT_TS: &str = r##"import { Component } from '@angular/core';

@Component({
  selector: 'app-home',
  standalone: true,
  template: `
    <section>
      <h1>Welcome</h1>
      <p>Your web workspace is ready.</p>
    </section>
  `,
})
export class HomeComponent {}
"##;

const WEB_CLIENT_ENVIRONMENT_TS: &str = r##"export const environment = {
  production: false,
  apiUrl: 'http://localhost:3000',
};
"##;

const WEB_CLIENT_ENVIRONMENT_PROD_TS: &str = r##"export const environment = {
  production: true,
  apiUrl: '/api',
};
"##;

const WEB_SERVER_PACKAGE_JSON: &str = r##"{
  "name": "server",
  "version": "0.1.0",
  "type": "module",
  "scripts": {
    "dev": "bun run --watch src/index.ts",
    "start": "bun run src/index.ts",
    "test": "bun test"
  },
  "dependencies": {
    "hono": "^4.4.0",
    "@hono/zod-validator": "^0.2.0",
    "zod": "^3.23.0",
    "jose": "^5.6.0"
  },
  "devDependencies": {
    "@types/bun": "latest"
  }
}
"##;

const WEB_SERVER_TSCONFIG_JSON: &str = r##"{
  "compilerOptions": {
    "target": "ESNext",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "strict": true,
    "skipLibCheck": true,
    "types": ["bun-types"]
  }
}
"##;

const WEB_SERVER_ENV_EXAMPLE: &str = r##"JWT_SECRET=change_me_in_production
PORT=3000
DATABASE_URL=./data/app.sqlite
"##;

const WEB_SERVER_LOGGER_TS: &str = r##"type LogLevel = 'info' | 'warn' | 'error' | 'debug';

interface LogEntry {
  level: LogLevel;
  message: string;
  timestamp: string;
  [key: string]: unknown;
}

function log(level: LogLevel, message: string, meta: Record<string, unknown> = {}) {
  const entry: LogEntry = {
    level,
    message,
    timestamp: new Date().toISOString(),
    ...meta,
  };
  console.log(JSON.stringify(entry));
}

export const logger = {
  info: (message: string, meta?: Record<string, unknown>) => log('info', message, meta),
  warn: (message: string, meta?: Record<string, unknown>) => log('warn', message, meta),
  error: (message: string, meta?: Record<string, unknown>) => log('error', message, meta),
  debug: (message: string, meta?: Record<string, unknown>) => log('debug', message, meta),
};
"##;

const WEB_SERVER_AUTH_MIDDLEWARE_TS: &str = r##"import { createMiddleware } from 'hono/factory';
import { jwtVerify } from 'jose';
import { logger } from '../lib/logger';

const JWT_SECRET = new TextEncoder().encode(process.env.JWT_SECRET ?? 'dev_secret');

export interface JwtPayload {
  sub: string;
  email: string;
  roles: string[];
  exp: number;
}

declare module 'hono' {
  interface ContextVariableMap {
    user: JwtPayload;
  }
}

export const authMiddleware = createMiddleware(async (c, next) => {
  const authHeader = c.req.header('Authorization');
  if (!authHeader?.startsWith('Bearer ')) {
    return c.json({ error: 'Unauthorized' }, 401);
  }

  const token = authHeader.slice(7);
  try {
    const { payload } = await jwtVerify(token, JWT_SECRET);
    c.set('user', payload as unknown as JwtPayload);
    await next();
  } catch (err) {
    logger.warn('JWT verification failed', { error: String(err) });
    return c.json({ error: 'Invalid token' }, 401);
  }
});
"##;

const WEB_SERVER_RBAC_MIDDLEWARE_TS: &str = r##"import { createMiddleware } from 'hono/factory';

export function requireRole(...roles: string[]) {
  return createMiddleware(async (c, next) => {
    const user = c.get('user');
    if (!user) {
      return c.json({ error: 'Unauthorized' }, 401);
    }

    const hasRole = roles.some((role) => user.roles.includes(role));
    if (!hasRole) {
      return c.json({ error: 'Forbidden' }, 403);
    }

    await next();
  });
}
"##;

const WEB_SERVER_HEALTH_ROUTE_TS: &str = r##"import { Hono } from 'hono';

const health = new Hono();

health.get('/', (c) => {
  return c.json({
    status: 'ok',
    timestamp: new Date().toISOString(),
    uptime: process.uptime(),
  });
});

export default health;
"##;

const WEB_SERVER_INDEX_TS: &str = r##"import { Hono } from 'hono';
import { logger as honoLogger } from 'hono/logger';
import { logger } from './lib/logger';
import health from './routes/health';

const app = new Hono();
const PORT = parseInt(process.env.PORT ?? '3000');

app.use('*', honoLogger((message) => logger.info(message)));

app.route('/health', health);

app.notFound((c) => c.json({ error: 'Not Found' }, 404));

app.onError((err, c) => {
  logger.error('Unhandled error', { error: err.message });
  return c.json({ error: 'Internal Server Error' }, 500);
});

logger.info(`Server starting on port ${PORT}`);

export default {
  port: PORT,
  fetch: app.fetch,
};
"##;

const WEB_JUSTFILE: &str = r##"set shell := ["bash", "-uc"]

# Run web client dev server
run-client:
    cd client && npm start

# Run web server
run-server:
    cd server && bun run dev

# Run both in parallel
run:
    just --parallel run-client run-server

# Build web client
build:
    cd client && npm run build

# Test all
test:
    cd client && npm test
    cd server && bun test

# Install all dependencies
deps:
    cd client && npm install
    cd server && bun install

# Deploy to firebase (client)
deploy target="firebase":
    cd client && npm run build
    firebase deploy --only hosting
"##;

const MOBILE_PUBSPEC_YAML: &str = r##"name: mobile_app
description: A production-grade Flutter application.
publish_to: none
version: 0.1.0+1

environment:
  sdk: ">=3.3.0 <4.0.0"

dependencies:
  flutter:
    sdk: flutter

  # Offline-first local database
  drift: ^2.18.0
  sqlite3_flutter_libs: ^0.5.0
  path_provider: ^2.1.0
  path: ^1.9.0

  # Secure token storage (biometric-backed)
  flutter_secure_storage: ^9.0.0

  # Networking
  dio: ^5.4.0

  # State management
  flutter_riverpod: ^2.5.0
  riverpod_annotation: ^2.3.0

  # Navigation
  go_router: ^13.0.0

dev_dependencies:
  flutter_test:
    sdk: flutter
  flutter_lints: ^4.0.0
  build_runner: ^2.4.0
  drift_dev: ^2.18.0
  riverpod_generator: ^2.4.0

flutter:
  uses-material-design: true
"##;

const MOBILE_ANALYSIS_OPTIONS_YAML: &str = r##"include: package:flutter_lints/flutter.yaml

linter:
  rules:
    - prefer_const_constructors
    - prefer_const_literals_to_create_immutables
    - avoid_print
    - use_key_in_widget_constructors
"##;

const MOBILE_MAIN_DART: &str = r##"import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'core/router/app_router.dart';

void main() {
  runApp(const ProviderScope(child: App()));
}

class App extends ConsumerWidget {
  const App({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final router = ref.watch(routerProvider);
    return MaterialApp.router(
      title: 'App',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      routerConfig: router,
    );
  }
}
"##;

const MOBILE_APP_DATABASE_DART: &str = r##"import 'dart:io';
import 'package:drift/drift.dart';
import 'package:drift/native.dart';
import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

part 'app_database.g.dart';

// Add your Drift table definitions here, e.g.:
// class Users extends Table { ... }

@DriftDatabase(tables: [])
class AppDatabase extends _$AppDatabase {
  AppDatabase() : super(_openConnection());

  @override
  int get schemaVersion => 1;
}

LazyDatabase _openConnection() {
  return LazyDatabase(() async {
    final dbFolder = await getApplicationDocumentsDirectory();
    final file = File(p.join(dbFolder.path, 'app.sqlite'));
    return NativeDatabase.createInBackground(file);
  });
}
"##;

const MOBILE_DIO_CLIENT_DART: &str = r##"import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'auth_interceptor.dart';

const String kBaseUrl = 'http://localhost:3000';

final dioProvider = Provider<Dio>((ref) {
  final dio = Dio(BaseOptions(
    baseUrl: kBaseUrl,
    connectTimeout: const Duration(seconds: 10),
    receiveTimeout: const Duration(seconds: 30),
    headers: {'Content-Type': 'application/json'},
  ));

  dio.interceptors.add(AuthInterceptor());

  return dio;
});
"##;

const MOBILE_AUTH_INTERCEPTOR_DART: &str = r##"import 'package:dio/dio.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';

class AuthInterceptor extends Interceptor {
  final FlutterSecureStorage _storage = const FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
  );

  @override
  void onRequest(
      RequestOptions options, RequestInterceptorHandler handler) async {
    final token = await _storage.read(key: 'jwt_token');
    if (token != null) {
      options.headers['Authorization'] = 'Bearer $token';
    }
    handler.next(options);
  }

  @override
  void onError(DioException err, ErrorInterceptorHandler handler) async {
    if (err.response?.statusCode == 401) {
      await _storage.delete(key: 'jwt_token');
      // TODO: redirect to login via router
    }
    handler.next(err);
  }
}
"##;

const MOBILE_AUTH_SERVICE_DART: &str = r##"import 'package:dio/dio.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import '../network/dio_client.dart';

class AuthState {
  final String? token;
  final bool isLoading;
  final String? error;

  const AuthState({this.token, this.isLoading = false, this.error});

  bool get isAuthenticated => token != null;
}

class AuthNotifier extends AsyncNotifier<AuthState> {
  final _storage = const FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
  );

  @override
  Future<AuthState> build() async {
    final token = await _storage.read(key: 'jwt_token');
    return AuthState(token: token);
  }

  Future<void> login(String email, String password) async {
    state = const AsyncLoading();
    try {
      final dio = ref.read(dioProvider);
      final response = await dio.post('/auth/login',
          data: {'email': email, 'password': password});
      final token = response.data['token'] as String;
      await _storage.write(key: 'jwt_token', value: token);
      state = AsyncData(AuthState(token: token));
    } on DioException catch (e) {
      state = AsyncData(AuthState(error: e.message));
    }
  }

  Future<void> logout() async {
    await _storage.delete(key: 'jwt_token');
    state = const AsyncData(AuthState());
  }
}

final authProvider = AsyncNotifierProvider<AuthNotifier, AuthState>(AuthNotifier.new);
"##;

const MOBILE_RBAC_SERVICE_DART: &str = r##"import 'package:flutter_riverpod/flutter_riverpod.dart';

// Roles match the JWT claims from the server
enum AppRole { admin, user, viewer }

final userRolesProvider = StateProvider<List<AppRole>>((ref) => []);

bool hasRole(WidgetRef ref, AppRole role) {
  return ref.read(userRolesProvider).contains(role);
}

// Use this in your widgets to conditionally show UI:
// if (hasRole(ref, AppRole.admin)) { ... }
"##;

const MOBILE_APP_ROUTER_DART: &str = r##"import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../../features/auth/login_screen.dart';
import '../auth/auth_service.dart';

final routerProvider = Provider<GoRouter>((ref) {
  final authState = ref.watch(authProvider);

  return GoRouter(
    initialLocation: '/login',
    redirect: (context, state) {
      final isLoggedIn = authState.valueOrNull?.isAuthenticated ?? false;
      final isLoginPage = state.matchedLocation == '/login';

      if (!isLoggedIn && !isLoginPage) return '/login';
      if (isLoggedIn && isLoginPage) return '/home';
      return null;
    },
    routes: [
      GoRoute(
        path: '/login',
        builder: (context, state) => const LoginScreen(),
      ),
      GoRoute(
        path: '/home',
        builder: (context, state) => const Scaffold(
          body: Center(child: Text('Home')),
        ),
      ),
    ],
  );
});
"##;

const MOBILE_LOGIN_SCREEN_DART: &str = r##"import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import '../../core/auth/auth_service.dart';

class LoginScreen extends ConsumerStatefulWidget {
  const LoginScreen({super.key});

  @override
  ConsumerState<LoginScreen> createState() => _LoginScreenState();
}

class _LoginScreenState extends ConsumerState<LoginScreen> {
  final _emailCtrl = TextEditingController();
  final _passwordCtrl = TextEditingController();

  @override
  void dispose() {
    _emailCtrl.dispose();
    _passwordCtrl.dispose();
    super.dispose();
  }

  Future<void> _submit() async {
    await ref.read(authProvider.notifier).login(
          _emailCtrl.text.trim(),
          _passwordCtrl.text,
        );
  }

  @override
  Widget build(BuildContext context) {
    final authState = ref.watch(authProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Sign In')),
      body: Padding(
        padding: const EdgeInsets.all(24),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            TextField(
                controller: _emailCtrl,
                decoration: const InputDecoration(labelText: 'Email')),
            const SizedBox(height: 12),
            TextField(
                controller: _passwordCtrl,
                obscureText: true,
                decoration: const InputDecoration(labelText: 'Password')),
            const SizedBox(height: 24),
            if (authState.isLoading)
              const CircularProgressIndicator()
            else
              ElevatedButton(onPressed: _submit, child: const Text('Sign In')),
            if (authState.valueOrNull?.error != null)
              Text(authState.valueOrNull!.error!,
                  style: const TextStyle(color: Colors.red)),
          ],
        ),
      ),
    );
  }
}
"##;

const MOBILE_WIDGET_TEST_DART: &str = r##"import 'package:flutter_test/flutter_test.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_app/main.dart';

void main() {
  testWidgets('renders app shell', (tester) async {
    await tester.pumpWidget(const ProviderScope(child: App()));
    expect(find.text('Sign In'), findsOneWidget);
  });
}
"##;

const MOBILE_JUSTFILE: &str = r##"set shell := ["bash", "-uc"]

# Run the Flutter app
run:
    cd app && flutter run

# Build Android APK
build:
    cd app && flutter build apk --release

# Run tests
test:
    cd app && flutter test

# Install dependencies
deps:
    cd app && flutter pub get
    cd app && dart run build_runner build --delete-conflicting-outputs

# Deploy to Firebase App Distribution
deploy target="firebase":
    cd app && flutter build apk --release
    firebase appdistribution:distribute app/build/app/outputs/flutter-apk/app-release.apk \
      --app $FIREBASE_APP_ID
"##;

const RESEARCH_MAIN_TEX: &str = r##"\documentclass[12pt,a4paper]{article}

% --- Packages ---
\usepackage[utf8]{inputenc}
\usepackage[T1]{fontenc}
\usepackage{amsmath, amssymb, amsthm}
\usepackage{graphicx}
\usepackage{hyperref}
\usepackage{booktabs}
\usepackage{cleveref}
\usepackage[style=ieee]{biblatex}
\usepackage{geometry}

\geometry{margin=2.5cm}

% --- Metadata ---
\title{Research Title}
\author{Author Name}
\date{\today}

\addbibresource{references.bib}

\begin{document}

\maketitle
\tableofcontents
\newpage

\input{src/01_introduction}
\input{src/02_methodology}
\input{src/03_results}
\input{src/04_conclusion}

\printbibliography

\end{document}
"##;

const RESEARCH_INTRODUCTION_TEX: &str = r##"\section{Introduction}
\label{sec:introduction}

Provide the research context and motivation here.

\subsection{Problem Statement}
Clearly define the problem being addressed.

\subsection{Contributions}
\begin{itemize}
    \item Contribution 1
    \item Contribution 2
\end{itemize}
"##;

const RESEARCH_METHODOLOGY_TEX: &str = r##"\section{Methodology}
\label{sec:methodology}

Describe the research approach, datasets, and experimental setup.

\subsection{Data}
Describe datasets used.

\subsection{Approach}
Describe the method or algorithm.
"##;

const RESEARCH_RESULTS_TEX: &str = r##"\section{Results}
\label{sec:results}

Present experimental results with tables and figures.

\subsection{Quantitative Results}

\begin{table}[h]
\centering
\begin{tabular}{lcc}
\toprule
Method & Metric 1 & Metric 2 \\
\midrule
Baseline & 0.00 & 0.00 \\
Ours & 0.00 & 0.00 \\
\bottomrule
\end{tabular}
\caption{Comparison of methods.}
\label{tab:results}
\end{table}
"##;

const RESEARCH_CONCLUSION_TEX: &str = r##"\section{Conclusion}
\label{sec:conclusion}

Summarize findings and future work directions.

\subsection{Future Work}
Describe planned extensions and open problems.
"##;

const RESEARCH_REFERENCES_BIB: &str = r##"@article{example2024,
  author  = {Author, First and Author, Second},
  title   = {Example Reference Title},
  journal = {Journal Name},
  year    = {2024},
  volume  = {1},
  pages   = {1--10},
}
"##;

const RESEARCH_JUSTFILE: &str = r##"set shell := ["bash", "-uc"]

# Compile the PDF
build:
    pdflatex -interaction=nonstopmode main.tex
    biber main
    pdflatex -interaction=nonstopmode main.tex
    pdflatex -interaction=nonstopmode main.tex

# Compile once (faster for previewing)
preview:
    pdflatex -interaction=nonstopmode main.tex

# Clean build artifacts
clean:
    rm -f *.aux *.bbl *.bcf *.blg *.log *.out *.run.xml *.toc

# Clean including PDF
distclean: clean
    rm -f *.pdf
"##;

const ML_PYPROJECT_TOML: &str = r##"[project]
name = "ml"
version = "0.1.0"
description = "ML module — research to production"
requires-python = ">=3.11"
dependencies = [
    "polars>=0.20.0",
    "scikit-learn>=1.4.0",
    "numpy>=1.26.0",
    "pydantic>=2.6.0",
    "huggingface-hub>=0.22.0",
    "jupyter>=1.0.0",
    "nbdev>=2.3.0",
]

[project.optional-dependencies]
dev = [
    "pytest>=8.0.0",
    "black>=24.0.0",
    "ruff>=0.3.0",
]

[tool.ruff]
line-length = 100
target-version = "py311"

[tool.pytest.ini_options]
testpaths = ["tests"]
"##;

const ML_INIT_PY: &str = r##""""ML starter package."""
"##;

const ML_DATA_PY: &str = r##""""Data loading and preprocessing — Polars-based, reproducible."""

from __future__ import annotations

import hashlib
from pathlib import Path

import polars as pl


def load_dataset(path: str | Path) -> pl.DataFrame:
    """Load a Parquet dataset and verify its checksum."""
    path = Path(path)
    if not path.exists():
        raise FileNotFoundError(f"Dataset not found: {path}")

    df = pl.read_parquet(path)
    return df


def compute_checksum(path: str | Path) -> str:
    """Compute SHA-256 checksum of a file for reproducibility tracking."""
    path = Path(path)
    sha256 = hashlib.sha256()
    with open(path, "rb") as f:
        for chunk in iter(lambda: f.read(8192), b""):
            sha256.update(chunk)
    return sha256.hexdigest()


def split_dataset(
    df: pl.DataFrame,
    target_col: str,
    test_size: float = 0.2,
    seed: int = 42,
) -> tuple[pl.DataFrame, pl.DataFrame]:
    """Reproducible train/test split."""
    df = df.sample(fraction=1.0, shuffle=True, seed=seed)
    split_idx = int(len(df) * (1 - test_size))
    return df[:split_idx], df[split_idx:]
"##;

const ML_MODEL_PY: &str = r##""""Model definition — same code used in notebooks AND production."""

from __future__ import annotations

import pickle
from pathlib import Path

from sklearn.linear_model import LogisticRegression
from sklearn.pipeline import Pipeline
from sklearn.preprocessing import StandardScaler


def build_pipeline(random_state: int = 42) -> Pipeline:
    """Build the production ML pipeline."""
    return Pipeline([
        ("scaler", StandardScaler()),
        ("classifier", LogisticRegression(random_state=random_state, max_iter=1000)),
    ])


def save_model(pipeline: Pipeline, path: str | Path) -> None:
    """Save model weights to disk."""
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    with open(path, "wb") as f:
        pickle.dump(pipeline, f)


def load_model(path: str | Path) -> Pipeline:
    """Load model from disk."""
    with open(Path(path), "rb") as f:
        return pickle.load(f)
"##;

const ML_TRAIN_PY: &str = r##""""Training script — identical logic to notebooks for inference parity."""

from __future__ import annotations

import json
from datetime import datetime, timezone
from pathlib import Path

from .data import compute_checksum, load_dataset, split_dataset
from .model import build_pipeline, save_model


def train(
    dataset_path: str | Path,
    output_dir: str | Path = "model_registry/latest",
    target_col: str = "label",
) -> dict:
    """Train and save a model. Returns metadata for the registry."""
    dataset_path = Path(dataset_path)
    output_dir = Path(output_dir)

    df = load_dataset(dataset_path)
    train_df, test_df = split_dataset(df, target_col)

    feature_cols = [c for c in df.columns if c != target_col]
    X_train = train_df.select(feature_cols).to_numpy()
    y_train = train_df[target_col].to_numpy()
    X_test = test_df.select(feature_cols).to_numpy()
    y_test = test_df[target_col].to_numpy()

    pipeline = build_pipeline()
    pipeline.fit(X_train, y_train)

    accuracy = float((pipeline.predict(X_test) == y_test).mean())

    model_path = output_dir / "model.pkl"
    save_model(pipeline, model_path)

    # Write heartbeat
    heartbeat = {"status": "trained", "timestamp": datetime.now(timezone.utc).isoformat()}
    Path("heartbeat.json").write_text(json.dumps(heartbeat))

    metadata = {
        "dataset": str(dataset_path),
        "dataset_checksum": compute_checksum(dataset_path),
        "accuracy": accuracy,
        "trained_at": heartbeat["timestamp"],
        "model_path": str(model_path),
    }
    (output_dir / "metadata.json").write_text(json.dumps(metadata, indent=2))

    return metadata


if __name__ == "__main__":
    import sys

    dataset = sys.argv[1] if len(sys.argv) > 1 else "data/dataset.parquet"
    metadata = train(dataset)
    print(json.dumps(metadata, indent=2))
"##;

const ML_PREDICT_PY: &str = r##""""Inference script — SAME pipeline as training for parity."""

from __future__ import annotations

from pathlib import Path

import numpy as np
import polars as pl

from .model import load_model


def predict(
    data: pl.DataFrame | list[dict],
    model_path: str | Path = "model_registry/latest/model.pkl",
) -> np.ndarray:
    """Run inference using the saved pipeline."""
    if isinstance(data, list):
        data = pl.DataFrame(data)

    pipeline = load_model(model_path)
    X = data.to_numpy()
    return pipeline.predict(X)


def predict_proba(
    data: pl.DataFrame | list[dict],
    model_path: str | Path = "model_registry/latest/model.pkl",
) -> np.ndarray:
    """Return class probabilities."""
    if isinstance(data, list):
        data = pl.DataFrame(data)

    pipeline = load_model(model_path)
    X = data.to_numpy()
    return pipeline.predict_proba(X)
"##;

const ML_MODEL_REGISTRY_METADATA_JSON: &str = r##"{
  "model": "placeholder",
  "weight_path": "model_registry/latest/model.pkl",
  "git_hash": null,
  "trained_at": null,
  "dataset_checksum": null,
  "accuracy": null
}
"##;

const ML_HEARTBEAT_JSON: &str = r##"{"status": "initialized"}
"##;

const ML_EDA_NOTEBOOK_IPYNB: &str = r##"{
 "cells": [
  {
   "cell_type": "markdown",
   "metadata": {},
   "source": ["# EDA Notebook\n", "Exploratory Data Analysis"]
  },
  {
   "cell_type": "code",
   "execution_count": null,
   "metadata": {},
   "outputs": [],
   "source": [
    "import polars as pl\n",
    "import numpy as np\n",
    "\n",
    "# Load your dataset\n",
    "# df = pl.read_parquet('../data/dataset.parquet')\n",
    "# df.head()"
   ]
  }
 ],
 "metadata": {
  "kernelspec": {
   "display_name": "Python 3",
   "language": "python",
   "name": "python3"
  },
  "language_info": {
   "name": "python",
   "version": "3.11.0"
  }
 },
 "nbformat": 4,
 "nbformat_minor": 5
}
"##;

const ML_TRAINING_NOTEBOOK_IPYNB: &str = r##"{
 "cells": [
  {
   "cell_type": "markdown",
   "metadata": {},
   "source": ["# Training Notebook\n", "Model training workflow"]
  },
  {
   "cell_type": "code",
   "execution_count": null,
   "metadata": {},
   "outputs": [],
   "source": [
    "from src.train import train\n",
    "\n",
    "# metadata = train('../data/dataset.parquet')\n",
    "# metadata"
   ]
  }
 ],
 "metadata": {
  "kernelspec": {
   "display_name": "Python 3",
   "language": "python",
   "name": "python3"
  },
  "language_info": {
   "name": "python",
   "version": "3.11.0"
  }
 },
 "nbformat": 4,
 "nbformat_minor": 5
}
"##;

const ML_TEST_SMOKE_PY: &str = r##"from src.model import build_pipeline


def test_build_pipeline():
    pipeline = build_pipeline()
    assert list(pipeline.named_steps.keys()) == ["scaler", "classifier"]
"##;

const ML_JUSTFILE: &str = r##"set shell := ["bash", "-uc"]

# Sync Python dependencies
deps:
    uv sync

# Run EDA notebook
notebooks:
    uv run jupyter lab notebooks/

# Train the model
train dataset="data/dataset.parquet":
    uv run python -m src.train {{dataset}}

# Run inference
predict:
    uv run python -m src.predict

# Run tests
test:
    uv run pytest

# Lint
lint:
    uv run ruff check src/
    uv run black --check src/

# Push model to HuggingFace
push-model repo="":
    uv run huggingface-cli upload {{repo}} model_registry/latest/
"##;

const HARDWARE_PLATFORMIO_INI: &str = r##"[env:esp32dev]
platform = espressif32
board = esp32dev
framework = arduino
monitor_speed = 115200
build_flags =
    -DCORE_DEBUG_LEVEL=3
    -DBOARD_HAS_PSRAM
lib_deps =
    ESP32 BLE Arduino
    AsyncTCP
    ESPAsyncWebServer
upload_protocol = esptool
"##;

const HARDWARE_VERSION: &str = r##"0.1.0
"##;

const HARDWARE_CONFIG_H: &str = r##"#pragma once

// --- Network ---
#define WIFI_SSID       ""
#define WIFI_PASSWORD   ""

// --- OTA ---
#define OTA_PASSWORD    "ota_password"
#define OTA_HOSTNAME    "device-hostname"

// --- Firmware ---
#define FIRMWARE_VERSION "0.1.0"

// --- Watchdog ---
#define WDT_TIMEOUT_SECONDS 30
"##;

const HARDWARE_WATCHDOG_H: &str = r##"#pragma once
#include <esp_task_wdt.h>

class Watchdog {
public:
    static void init(uint32_t timeout_seconds) {
        esp_task_wdt_config_t wdt_config = {
            .timeout_ms = timeout_seconds * 1000,
            .idle_core_mask = 0,
            .trigger_panic = true,
        };
        esp_task_wdt_reconfigure(&wdt_config);
        esp_task_wdt_add(NULL);
    }

    static void reset() {
        esp_task_wdt_reset();
    }

    static void deinit() {
        esp_task_wdt_delete(NULL);
    }
};
"##;

const HARDWARE_OTA_H: &str = r##"#pragma once
#include <ArduinoOTA.h>
#include <WiFi.h>
#include "config.h"

class OTAUpdater {
public:
    static void init() {
        ArduinoOTA.setHostname(OTA_HOSTNAME);
        ArduinoOTA.setPassword(OTA_PASSWORD);

        ArduinoOTA.onStart([]() {
            String type = (ArduinoOTA.getCommand() == U_FLASH) ? "sketch" : "filesystem";
            Serial.println("[OTA] Start updating " + type);
        });

        ArduinoOTA.onEnd([]() {
            Serial.println("\n[OTA] Update complete. Rebooting...");
        });

        ArduinoOTA.onProgress([](unsigned int progress, unsigned int total) {
            Serial.printf("[OTA] Progress: %u%%\r", (progress / (total / 100)));
        });

        ArduinoOTA.onError([](ota_error_t error) {
            Serial.printf("[OTA] Error[%u]: ", error);
            if (error == OTA_AUTH_ERROR)         Serial.println("Auth Failed");
            else if (error == OTA_BEGIN_ERROR)   Serial.println("Begin Failed");
            else if (error == OTA_CONNECT_ERROR) Serial.println("Connect Failed");
            else if (error == OTA_RECEIVE_ERROR) Serial.println("Receive Failed");
            else if (error == OTA_END_ERROR)     Serial.println("End Failed");
        });

        ArduinoOTA.begin();
        Serial.println("[OTA] Ready. IP: " + WiFi.localIP().toString());
    }

    static void handle() {
        ArduinoOTA.handle();
    }
};
"##;

const HARDWARE_MAIN_CPP: &str = r##"#include <Arduino.h>
#include <WiFi.h>
#include "config.h"
#include "ota.h"
#include "watchdog.h"

void setup() {
    Serial.begin(115200);
    Serial.println("[BOOT] Firmware v" FIRMWARE_VERSION);

    // Initialize watchdog (reboots device if loop() stalls)
    Watchdog::init(WDT_TIMEOUT_SECONDS);

    // Connect to WiFi
    WiFi.begin(WIFI_SSID, WIFI_PASSWORD);
    Serial.print("[WiFi] Connecting");
    while (WiFi.status() != WL_CONNECTED) {
        delay(500);
        Serial.print(".");
        Watchdog::reset();
    }
    Serial.println("\n[WiFi] Connected: " + WiFi.localIP().toString());

    // Initialize OTA updater
    OTAUpdater::init();

    Serial.println("[BOOT] Setup complete.");
}

void loop() {
    Watchdog::reset();      // Feed the watchdog
    OTAUpdater::handle();   // Listen for OTA updates

    // --- Application Logic ---
    // Add sensor reads, DMA transfers, and business logic here.

    delay(10);
}
"##;

const HARDWARE_JUSTFILE: &str = r##"set shell := ["bash", "-uc"]

# Build firmware
build:
    pio run

# Flash firmware to device
flash:
    pio run --target upload

# Monitor serial output
monitor:
    pio device monitor

# Run tests (QEMU/native)
test:
    pio test --environment native

# Clean build artifacts
clean:
    pio run --target clean

# Check for OTA environment variable
deploy target="ota":
    @echo "Deploying via OTA to {{target}}..."
    pio run --target upload --upload-port $OTA_DEVICE_IP
"##;

fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)?;
    Ok(())
}

pub fn scaffold_web(root: &Path) -> Result<()> {
    let client_root = root.join("web/client");
    let server_root = root.join("web/server");

    fs::create_dir_all(client_root.join("src/app/core/auth"))?;
    fs::create_dir_all(client_root.join("src/app/core/interceptors"))?;
    fs::create_dir_all(client_root.join("src/app/features/auth"))?;
    fs::create_dir_all(client_root.join("src/app/features/home"))?;
    fs::create_dir_all(client_root.join("src/app/features/shell"))?;
    fs::create_dir_all(client_root.join("src/environments"))?;

    fs::create_dir_all(server_root.join("src/lib"))?;
    fs::create_dir_all(server_root.join("src/middleware"))?;
    fs::create_dir_all(server_root.join("src/routes"))?;

    write_file(&root.join("web/justfile"), WEB_JUSTFILE)?;

    write_file(&client_root.join("package.json"), WEB_CLIENT_PACKAGE_JSON)?;
    write_file(&client_root.join("tsconfig.json"), WEB_CLIENT_TSCONFIG_JSON)?;
    write_file(&client_root.join("src/index.html"), WEB_CLIENT_INDEX_HTML)?;
    write_file(&client_root.join("src/main.ts"), WEB_CLIENT_MAIN_TS)?;
    write_file(&client_root.join("src/styles.scss"), WEB_CLIENT_STYLES_SCSS)?;
    write_file(
        &client_root.join("src/app/app.component.ts"),
        WEB_CLIENT_APP_COMPONENT_TS,
    )?;
    write_file(
        &client_root.join("src/app/app.config.ts"),
        WEB_CLIENT_APP_CONFIG_TS,
    )?;
    write_file(
        &client_root.join("src/app/app.routes.ts"),
        WEB_CLIENT_APP_ROUTES_TS,
    )?;
    write_file(
        &client_root.join("src/app/core/auth/auth.service.ts"),
        WEB_CLIENT_AUTH_SERVICE_TS,
    )?;
    write_file(
        &client_root.join("src/app/core/auth/auth.guard.ts"),
        WEB_CLIENT_AUTH_GUARD_TS,
    )?;
    write_file(
        &client_root.join("src/app/core/interceptors/auth.interceptor.ts"),
        WEB_CLIENT_AUTH_INTERCEPTOR_TS,
    )?;
    write_file(
        &client_root.join("src/app/features/shell/shell.component.ts"),
        WEB_CLIENT_SHELL_COMPONENT_TS,
    )?;
    write_file(
        &client_root.join("src/app/features/shell/shell.routes.ts"),
        WEB_CLIENT_SHELL_ROUTES_TS,
    )?;
    write_file(
        &client_root.join("src/app/features/auth/login.component.ts"),
        WEB_CLIENT_LOGIN_COMPONENT_TS,
    )?;
    write_file(
        &client_root.join("src/app/features/home/home.component.ts"),
        WEB_CLIENT_HOME_COMPONENT_TS,
    )?;
    write_file(
        &client_root.join("src/environments/environment.ts"),
        WEB_CLIENT_ENVIRONMENT_TS,
    )?;
    write_file(
        &client_root.join("src/environments/environment.prod.ts"),
        WEB_CLIENT_ENVIRONMENT_PROD_TS,
    )?;

    write_file(&server_root.join("package.json"), WEB_SERVER_PACKAGE_JSON)?;
    write_file(&server_root.join("tsconfig.json"), WEB_SERVER_TSCONFIG_JSON)?;
    write_file(&server_root.join(".env.example"), WEB_SERVER_ENV_EXAMPLE)?;
    write_file(&server_root.join("src/lib/logger.ts"), WEB_SERVER_LOGGER_TS)?;
    write_file(
        &server_root.join("src/middleware/auth.ts"),
        WEB_SERVER_AUTH_MIDDLEWARE_TS,
    )?;
    write_file(
        &server_root.join("src/middleware/rbac.ts"),
        WEB_SERVER_RBAC_MIDDLEWARE_TS,
    )?;
    write_file(
        &server_root.join("src/routes/health.ts"),
        WEB_SERVER_HEALTH_ROUTE_TS,
    )?;
    write_file(&server_root.join("src/index.ts"), WEB_SERVER_INDEX_TS)?;

    Ok(())
}

pub fn scaffold_mobile(root: &Path) -> Result<()> {
    let app_root = root.join("mobile/app");

    fs::create_dir_all(app_root.join("lib/core/database"))?;
    fs::create_dir_all(app_root.join("lib/core/network"))?;
    fs::create_dir_all(app_root.join("lib/core/auth"))?;
    fs::create_dir_all(app_root.join("lib/core/rbac"))?;
    fs::create_dir_all(app_root.join("lib/core/router"))?;
    fs::create_dir_all(app_root.join("lib/features/auth"))?;
    fs::create_dir_all(app_root.join("test"))?;

    write_file(&root.join("mobile/justfile"), MOBILE_JUSTFILE)?;
    write_file(&app_root.join("pubspec.yaml"), MOBILE_PUBSPEC_YAML)?;
    write_file(
        &app_root.join("analysis_options.yaml"),
        MOBILE_ANALYSIS_OPTIONS_YAML,
    )?;
    write_file(&app_root.join("lib/main.dart"), MOBILE_MAIN_DART)?;
    write_file(
        &app_root.join("lib/core/database/app_database.dart"),
        MOBILE_APP_DATABASE_DART,
    )?;
    write_file(
        &app_root.join("lib/core/network/dio_client.dart"),
        MOBILE_DIO_CLIENT_DART,
    )?;
    write_file(
        &app_root.join("lib/core/network/auth_interceptor.dart"),
        MOBILE_AUTH_INTERCEPTOR_DART,
    )?;
    write_file(
        &app_root.join("lib/core/auth/auth_service.dart"),
        MOBILE_AUTH_SERVICE_DART,
    )?;
    write_file(
        &app_root.join("lib/core/rbac/rbac_service.dart"),
        MOBILE_RBAC_SERVICE_DART,
    )?;
    write_file(
        &app_root.join("lib/core/router/app_router.dart"),
        MOBILE_APP_ROUTER_DART,
    )?;
    write_file(
        &app_root.join("lib/features/auth/login_screen.dart"),
        MOBILE_LOGIN_SCREEN_DART,
    )?;
    write_file(
        &app_root.join("test/widget_test.dart"),
        MOBILE_WIDGET_TEST_DART,
    )?;

    Ok(())
}

pub fn scaffold_research(root: &Path) -> Result<()> {
    let research_root = root.join("research");

    fs::create_dir_all(research_root.join("src"))?;

    write_file(&research_root.join("main.tex"), RESEARCH_MAIN_TEX)?;
    write_file(
        &research_root.join("src/01_introduction.tex"),
        RESEARCH_INTRODUCTION_TEX,
    )?;
    write_file(
        &research_root.join("src/02_methodology.tex"),
        RESEARCH_METHODOLOGY_TEX,
    )?;
    write_file(
        &research_root.join("src/03_results.tex"),
        RESEARCH_RESULTS_TEX,
    )?;
    write_file(
        &research_root.join("src/04_conclusion.tex"),
        RESEARCH_CONCLUSION_TEX,
    )?;
    write_file(
        &research_root.join("references.bib"),
        RESEARCH_REFERENCES_BIB,
    )?;
    write_file(&research_root.join("justfile"), RESEARCH_JUSTFILE)?;

    Ok(())
}

pub fn scaffold_ml(root: &Path) -> Result<()> {
    let ml_root = root.join("ml");

    fs::create_dir_all(ml_root.join("src"))?;
    fs::create_dir_all(ml_root.join("notebooks"))?;
    fs::create_dir_all(ml_root.join("model_registry"))?;
    fs::create_dir_all(ml_root.join("tests"))?;

    write_file(&ml_root.join("pyproject.toml"), ML_PYPROJECT_TOML)?;
    write_file(&ml_root.join("src/__init__.py"), ML_INIT_PY)?;
    write_file(&ml_root.join("src/data.py"), ML_DATA_PY)?;
    write_file(&ml_root.join("src/model.py"), ML_MODEL_PY)?;
    write_file(&ml_root.join("src/train.py"), ML_TRAIN_PY)?;
    write_file(&ml_root.join("src/predict.py"), ML_PREDICT_PY)?;
    write_file(
        &ml_root.join("model_registry/metadata.json"),
        ML_MODEL_REGISTRY_METADATA_JSON,
    )?;
    write_file(&ml_root.join("heartbeat.json"), ML_HEARTBEAT_JSON)?;
    write_file(
        &ml_root.join("notebooks/01_eda.ipynb"),
        ML_EDA_NOTEBOOK_IPYNB,
    )?;
    write_file(
        &ml_root.join("notebooks/02_training.ipynb"),
        ML_TRAINING_NOTEBOOK_IPYNB,
    )?;
    write_file(&ml_root.join("tests/test_smoke.py"), ML_TEST_SMOKE_PY)?;
    write_file(&ml_root.join("justfile"), ML_JUSTFILE)?;

    Ok(())
}

pub fn scaffold_hardware(root: &Path) -> Result<()> {
    let hardware_root = root.join("hardware");

    fs::create_dir_all(hardware_root.join("src"))?;

    write_file(
        &hardware_root.join("platformio.ini"),
        HARDWARE_PLATFORMIO_INI,
    )?;
    write_file(&hardware_root.join("VERSION"), HARDWARE_VERSION)?;
    write_file(&hardware_root.join("src/config.h"), HARDWARE_CONFIG_H)?;
    write_file(&hardware_root.join("src/watchdog.h"), HARDWARE_WATCHDOG_H)?;
    write_file(&hardware_root.join("src/ota.h"), HARDWARE_OTA_H)?;
    write_file(&hardware_root.join("src/main.cpp"), HARDWARE_MAIN_CPP)?;
    write_file(&hardware_root.join("justfile"), HARDWARE_JUSTFILE)?;

    Ok(())
}
