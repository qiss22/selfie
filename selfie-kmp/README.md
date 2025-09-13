# Selfie KMP

![Selfie Logo](assets/logo.png) <!-- Placeholder for logo; add via scaffold -->

Selfie is a modern, cross-platform social media app rivaling Instagram and Facebook, built with Kotlin Multiplatform (KMP) for seamless code sharing across Android, iOS, Web (PWA), and Desktop. Leverage 85%+ shared code for business logic, UI (via Compose Multiplatform), and data layers, while maintaining native performance and feel on each platform. This monorepo follows 2025 best practices: modular features, hierarchical source sets, and stable K2 compiler integration for robust multiplatform development.

Powered by Kotlin 2.2.20 (stable K2 compiler with smart casting and multiplatform enhancements) and Compose Multiplatform 1.9.0 (stable iOS support, web accessibility, and Material3 decoupling), Selfie delivers real-time features like personalized feeds, stories, chats, and monetization (ads/shop) with offline-first caching via SQLDelight.

## 🚀 Quick Start

### Prerequisites
- **JDK 17+** (for JVM/Desktop targets)
- **Android Studio Koala+** (with Kotlin Multiplatform plugin)
- **Xcode 16+** (for iOS/watchOS)
- **Node.js 20+** (for Web target)
- **Gradle 8.10+** (wrapper included)

### Setup
1. Clone and enter the repo:
   ```
   git clone https://github.com/qiss22/selfie
   cd selfie-kmp
   ```

2. Initialize dependencies (downloads via Gradle):
   ```
   ./gradlew sync
   ```

3. Run platform-specific builds:
   - **Android**: Open in Android Studio → `./gradlew :platform:androidApp:installDebug`
   - **iOS**: Open `platform/iosApp/Selfie.xcodeproj` in Xcode → Build/Run on simulator
   - **Web**: `cd platform/webApp && npm install && npm run dev` (Vite dev server at http://localhost:3000)
   - **Desktop**: `./gradlew :platform:desktopApp:run`

4. Full build/test all targets:
   ```
   ./gradlew build test
   ```

For CI/CD, see `.github/workflows/gradle.yml`.

## 🏗️ Project Structure

```
selfie-kmp/
├── shared/                  # Core KMP module: 85% shared code
│   ├── src/commonMain/      # Domain (entities/use cases), Data (Ktor/SQLDelight), Presentation (Compose MP ViewModels)
│   ├── src/androidMain/     # Android actuals (OkHttp, Haptics)
│   ├── src/iosMain/         # iOS actuals (URLSession, Keychain)
│   ├── src/jsMain/          # Web actuals (Fetch API, LocalStorage)
│   └── src/desktopMain/     # Desktop actuals (JVM File I/O)
├── features/                # Modular features (shared + platform variants)
│   ├── auth/                # Login/2FA (biometrics actuals)
│   ├── home/                # Feeds/Stories/Reels (Exo/AVPlayer)
│   ├── chat/                # Real-time DMs (WebSockets, E2E crypto)
│   └── ...                  # (profile, search, shop, etc.)
├── platform/                # Native apps
│   ├── androidApp/          # Compose + Hilt DI
│   ├── iosApp/              # SwiftUI wrapper for KMP
│   ├── webApp/              # PWA with Vite/Webpack
│   └── desktopApp/          # JVM Compose window
├── libraries/               # Reusable KMP libs (e.g., selfie-ui for components)
├── buildSrc/                # Gradle conventions/version catalog
├── scripts/                 # Codegen (proto/GraphQL), lint
└── ci/                      # GitHub Actions workflows
```

- **Shared Code Ratio**: Targets 80-90% reuse via `commonMain` (domain/data/UI), with `expect/actual` for platform diffs.
- **UI**: Compose Multiplatform for declarative, shared screens (e.g., `HomeScreen.kt` with `LazyColumn` for feeds).
- **Data**: Ktor for unified networking (REST/gRPC to Rust backend), SQLDelight for offline DB.
- **Testing**: Multiplatform tests in `commonTest` (80%+ coverage with MockK/Turbine).

## 🛠️ Tech Stack

| Layer | Tech | Why? |
|-------|------|------|
| **Language** | Kotlin 2.2.20 | Stable K2 compiler, context receivers for DI, enhanced multiplatform stability. |
| **UI** | Compose Multiplatform 1.9.0 | Shared declarative UI, stable iOS/Web, Material3 theming. |
| **Networking** | Ktor 2.3.12 | Async multiplatform HTTP/WebSockets (auth, feeds, chat). |
| **Persistence** | SQLDelight 2.0.2 | Shared SQLite schema, offline caching. |
| **DI** | Koin/Hilt (Android) | Modular injection. |
| **Backend Integration** | gRPC/GraphQL via protos | Codegen DTOs from Rust backend. |
| **Build** | Gradle 8.10 + TOML catalogs | Hierarchical source sets, Amper-ready. |

## 🌟 Key Features

- **Personalized Feeds**: ML-powered (via backend recs), infinite scroll with pull-to-refresh.
- **Stories & Reels**: Ephemeral content, video editing (shared AV/ExoPlayer).
- **Real-Time Chat**: WebSockets, E2E encryption (libsodium-kmp), typing indicators.
- **Monetization**: In-app shop, ads (Stripe MP integration).
- **Offline-First**: Cache feeds/chats in SQLDelight, sync on reconnect.
- **Accessibility**: Semantics in Compose, TalkBack/VoiceOver actuals.
- **PWA/Offline Web**: Service workers for caching, WASM fallback.

## 🚀 Development Workflow

1. **Code in Shared**: Write use cases in `commonMain` (e.g., `FetchFeedUseCase.kt`).
2. **Platform Tweaks**: Implement actuals (e.g., `HapticFeedbackAndroid.kt`).
3. **Test**: `./gradlew test` (common + platform-specific).
4. **Run Feature**: `./gradlew :features:home:run` (JVM preview).
5. **Deploy**:
   - Android: `./gradlew :platform:androidApp:publish`
   - iOS: Xcode archive → TestFlight
   - Web: `npm run build` → Static host
   - Desktop: `./gradlew :platform:desktopApp:jpackage`

Use `./scripts/generate-models.kts` for backend proto sync. Lint with `./scripts/lint-all.sh` (ktlint/Detekt).

## 📱 Platforms

- **Android**: Min SDK 24, Compose 1.6+, Hilt DI.
- **iOS**: iOS 17+, SwiftUI interop, watchOS complications for notifications.
- **Web**: Modern browsers (Chrome 120+), PWA installable, WASM/JS fallback.
- **Desktop**: JVM, macOS/Windows/Linux, tray icon support.

## 🔍 Contributing

1. Fork & PR to `main`.
2. Follow [Kotlin Style Guide](https://kotlinlang.org/docs/coding-conventions.html) + 2025 best practices (e.g., sealed classes for models, structured concurrency).
3. Run CI checks: `./gradlew check`.
4. Add tests; aim for 80% coverage.

See `CONTRIBUTING.md` for details. Issues? Open a ticket!

## 📄 License

MIT License. See [LICENSE](LICENSE).

## 🔗 Resources

- [Kotlin Multiplatform Docs](https://kotlinlang.org/docs/multiplatform.html)
- [Compose Multiplatform Roadmap 2025](https://blog.jetbrains.com/kotlin/2025/08/kmp-roadmap-aug-2025/)
- Community: [Reddit r/Kotlin](https://www.reddit.com/r/Kotlin/) (adoption discussions)

---

*Last updated: September 13, 2025.*
