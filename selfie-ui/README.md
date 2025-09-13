# Selfie UI (Compose Multiplatform)

CMP implementation for shared UI screens in Selfie app.

## Structure
- `shared-ui/` - CMP screens and components
- `shared-core/` - Business logic and ViewModels  
- `design-system/` - Design tokens and theming
- `platform/` - Platform bridges

## Build
```bash
./gradlew build
```

## Integration
1. Move modules into main selfie-kmp project
2. Update settings.gradle.kts
3. Connect to existing business logic
4. Implement platform bridges
