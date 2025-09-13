#!/usr/bin/env python3
"""
Selfie UI Scaffolding Script - Creates CMP structure with minimal stubs
"""

import os
from pathlib import Path


def create_file(path: Path, content: str = "") -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)


class SelfieUIScaffolder:
    def __init__(self, root_dir: str = "selfie-ui"):
        self.root = Path(root_dir)
        
    def scaffold(self):
        print(f"Creating Selfie UI structure in {self.root}")
        
        self.create_root_config()
        self.create_shared_ui()
        self.create_design_system()
        self.create_platform_bridges()
        self.create_tests()
        
        print("✅ Selfie UI structure created!")
        
    def create_root_config(self):
        create_file(self.root / "settings.gradle.kts", '''rootProject.name = "selfie-ui"

include(
    ":shared-ui",
    ":design-system",
    ":platform:android-bridge",
    ":platform:ios-bridge",
    ":integration-tests",
    ":snapshot-tests"
)''')

        # Root build.gradle.kts
        create_file(self.root / "build.gradle.kts", '''plugins {
    kotlin("multiplatform") apply false
    kotlin("android") apply false
    id("com.android.library") apply false
    id("org.jetbrains.compose") apply false
}

allprojects {
    repositories {
        google()
        mavenCentral()
        maven("https://maven.pkg.jetbrains.space/public/p/compose/dev")
    }
}''')

        # gradle.properties
        create_file(self.root / "gradle.properties", '''kotlin.code.style=official
kotlin.mpp.enableCInteropCommonization=true
kotlin.mpp.androidSourceSetLayoutVersion=2
android.useAndroidX=true
android.compileSdk=34
android.minSdk=24
org.gradle.jvmargs=-Xmx4g''')

        # Version catalog
        create_file(self.root / "gradle/libs.versions.toml", '''[versions]
kotlin = "1.9.20"
compose = "1.5.10"
coroutines = "1.7.3"
koin = "3.5.0"

[libraries]
compose-ui = { module = "androidx.compose.ui:ui", version.ref = "compose" }
compose-material3 = { module = "androidx.compose.material3:material3", version = "1.1.2" }
kotlinx-coroutines = { module = "org.jetbrains.kotlinx:kotlinx-coroutines-core", version.ref = "coroutines" }
koin-core = { module = "io.insert-koin:koin-core", version.ref = "koin" }

[plugins]
kotlinMultiplatform = { id = "org.jetbrains.kotlin.multiplatform", version.ref = "kotlin" }
androidLibrary = { id = "com.android.library", version = "8.1.4" }
jetbrainsCompose = { id = "org.jetbrains.compose", version.ref = "compose" }''')

    def create_shared_ui(self):
        # Build config
        create_file(self.root / "shared-ui/build.gradle.kts", '''plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.jetbrainsCompose)
}

kotlin {
    androidTarget()
    iosX64()
    iosArm64()
    iosSimulatorArm64()
    js(IR) { browser() }
    jvm()
    
    sourceSets {
        commonMain.dependencies {
            implementation(project(":design-system"))
            implementation(compose.runtime)
            implementation(compose.foundation)
            implementation(compose.material3)
            implementation(compose.ui)
        }
    }
}

android {
    namespace = "com.selfie.ui"
    compileSdk = 34
    defaultConfig { minSdk = 24 }
}''')

        # Main app composable
        create_file(self.root / "shared-ui/src/commonMain/kotlin/com/selfie/ui/SelfieApp.kt", '''package com.selfie.ui

import androidx.compose.runtime.Composable

@Composable
fun SelfieApp() {
    // TODO: Implement main app composable
}''')

        # Screen directories and stubs
        screens = [
            "auth/LoginScreen.kt",
            "auth/SignUpScreen.kt", 
            "settings/SettingsScreen.kt",
            "settings/AccountSettingsScreen.kt",
            "settings/PrivacySettingsScreen.kt",
            "profile/EditProfileScreen.kt",
            "shop/ShopScreen.kt",
            "shop/ProductDetailScreen.kt",
            "admin/AdminDashboardScreen.kt",
            "support/HelpScreen.kt",
            "legal/TermsScreen.kt"
        ]

        for screen in screens:
            screen_name = screen.split('/')[-1].replace('.kt', '')
            create_file(self.root / f"shared-ui/src/commonMain/kotlin/com/selfie/ui/screens/{screen}", 
                f'''package com.selfie.ui.screens.{screen.split('/')[0]}

import androidx.compose.runtime.Composable

@Composable
fun {screen_name}() {{
    // TODO: Implement {screen_name}
}}''')

        # Component stubs
        components = [
            "buttons/PrimaryButton.kt",
            "forms/InputField.kt",
            "forms/PasswordField.kt",
            "layout/TopBar.kt",
            "cards/BaseCard.kt"
        ]

        for component in components:
            comp_name = component.split('/')[-1].replace('.kt', '')
            create_file(self.root / f"shared-ui/src/commonMain/kotlin/com/selfie/ui/components/{component}",
                f'''package com.selfie.ui.components.{component.split('/')[0]}

import androidx.compose.runtime.Composable

@Composable
fun {comp_name}() {{
    // TODO: Implement {comp_name}
}}''')

        # Navigation
        create_file(self.root / "shared-ui/src/commonMain/kotlin/com/selfie/ui/navigation/Destinations.kt", '''package com.selfie.ui.navigation

sealed class Destination(val route: String) {
    object Login : Destination("auth/login")
    object Settings : Destination("settings")
    object Shop : Destination("shop")
    object Admin : Destination("admin")
}''')

        # ViewModels with StateFlow contracts
        create_file(self.root / "shared-ui/src/commonMain/kotlin/com/selfie/ui/presentation/AuthViewModel.kt", '''package com.selfie.ui.presentation

import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.MutableStateFlow

interface AuthViewModelContract {
    val uiState: StateFlow<AuthUiState>
    fun login(email: String, password: String)
    fun signUp(email: String, password: String, username: String)
}

data class AuthUiState(
    val isLoading: Boolean = false,
    val isSuccess: Boolean = false,
    val errorMessage: String? = null
)

class AuthViewModel : AuthViewModelContract {
    private val _uiState = MutableStateFlow(AuthUiState())
    override val uiState: StateFlow<AuthUiState> = _uiState
    
    override fun login(email: String, password: String) {
        // TODO: Connect to domain use cases from selfie-kmp
    }
    
    override fun signUp(email: String, password: String, username: String) {
        // TODO: Connect to domain use cases from selfie-kmp  
    }
}''')

        create_file(self.root / "shared-ui/src/commonMain/kotlin/com/selfie/ui/presentation/SettingsViewModel.kt", '''package com.selfie.ui.presentation

import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.MutableStateFlow

interface SettingsViewModelContract {
    val uiState: StateFlow<SettingsUiState>
    fun updateNotifications(enabled: Boolean)
    fun updatePrivacy(isPrivate: Boolean)
}

data class SettingsUiState(
    val notificationsEnabled: Boolean = true,
    val isPrivateAccount: Boolean = false,
    val isLoading: Boolean = false
)

class SettingsViewModel : SettingsViewModelContract {
    private val _uiState = MutableStateFlow(SettingsUiState())
    override val uiState: StateFlow<SettingsUiState> = _uiState
    
    override fun updateNotifications(enabled: Boolean) {
        // TODO: Connect to domain use cases
    }
    
    override fun updatePrivacy(isPrivate: Boolean) {
        // TODO: Connect to domain use cases
    }
}''')

    def create_shared_viewmodels(self):
        # UI-specific ViewModels in shared-ui
        create_file(self.root / "shared-ui/src/commonMain/kotlin/com/selfie/ui/presentation/AuthViewModel.kt", '''package com.selfie.ui.presentation

import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.MutableStateFlow

interface AuthViewModelContract {
    val uiState: StateFlow<AuthUiState>
    fun login(email: String, password: String)
    fun signUp(email: String, password: String, username: String)
}

data class AuthUiState(
    val isLoading: Boolean = false,
    val isSuccess: Boolean = false,
    val errorMessage: String? = null
)

class AuthViewModel : AuthViewModelContract {
    private val _uiState = MutableStateFlow(AuthUiState())
    override val uiState: StateFlow<AuthUiState> = _uiState
    
    override fun login(email: String, password: String) {
        // TODO: Connect to domain use cases from selfie-kmp
    }
    
    override fun signUp(email: String, password: String, username: String) {
        // TODO: Connect to domain use cases from selfie-kmp  
    }
}''')

    def create_design_system(self):
        create_file(self.root / "design-system/build.gradle.kts", '''plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.jetbrainsCompose)
}

kotlin {
    androidTarget()
    iosX64()
    iosArm64()
    iosSimulatorArm64()
    js(IR) { browser() }
    jvm()
    
    sourceSets {
        commonMain.dependencies {
            implementation(compose.runtime)
            implementation(compose.material3)
        }
    }
}

android {
    namespace = "com.selfie.designsystem"
    compileSdk = 34
    defaultConfig { minSdk = 24 }
}''')

        # Separate design token files
        create_file(self.root / "design-system/src/commonMain/kotlin/com/selfie/designsystem/Colors.kt", '''package com.selfie.designsystem

import androidx.compose.ui.graphics.Color

object SelfieColors {
    // Light theme
    val Primary = Color(0xFF007AFF)
    val Secondary = Color(0xFF5856D6)
    val Background = Color(0xFFFFFFFF)
    val Surface = Color(0xFFF2F2F7)
    val Error = Color(0xFFFF3B30)
    
    // Dark theme
    val PrimaryDark = Color(0xFF0A84FF)
    val SecondaryDark = Color(0xFF5E5CE6)
    val BackgroundDark = Color(0xFF000000)
    val SurfaceDark = Color(0xFF1C1C1E)
    val ErrorDark = Color(0xFFFF453A)
}''')

        create_file(self.root / "design-system/src/commonMain/kotlin/com/selfie/designsystem/Typography.kt", '''package com.selfie.designsystem

import androidx.compose.material3.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.sp

val SelfieTypography = Typography(
    headlineLarge = TextStyle(fontWeight = FontWeight.Bold, fontSize = 32.sp),
    headlineMedium = TextStyle(fontWeight = FontWeight.Bold, fontSize = 28.sp),
    titleLarge = TextStyle(fontWeight = FontWeight.SemiBold, fontSize = 22.sp),
    bodyLarge = TextStyle(fontWeight = FontWeight.Normal, fontSize = 16.sp),
    bodyMedium = TextStyle(fontWeight = FontWeight.Normal, fontSize = 14.sp),
    labelMedium = TextStyle(fontWeight = FontWeight.Medium, fontSize = 12.sp)
)''')

        create_file(self.root / "design-system/src/commonMain/kotlin/com/selfie/designsystem/Spacing.kt", '''package com.selfie.designsystem

import androidx.compose.ui.unit.dp

object SelfieSpacing {
    val xs = 4.dp
    val sm = 8.dp
    val md = 16.dp
    val lg = 24.dp
    val xl = 32.dp
    val xxl = 48.dp
}''')

        create_file(self.root / "design-system/src/commonMain/kotlin/com/selfie/designsystem/Theme.kt", '''package com.selfie.designsystem

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable

@Composable
fun SelfieTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit
) {
    val colorScheme = if (darkTheme) {
        darkColorScheme(
            primary = SelfieColors.PrimaryDark,
            secondary = SelfieColors.SecondaryDark,
            background = SelfieColors.BackgroundDark
        )
    } else {
        lightColorScheme(
            primary = SelfieColors.Primary,
            secondary = SelfieColors.Secondary,
            background = SelfieColors.Background
        )
    }
    
    MaterialTheme(
        colorScheme = colorScheme,
        typography = SelfieTypography,
        content = content
    )
}''')

    def create_platform_bridges(self):
        # Android bridge
        create_file(self.root / "platform/android-bridge/build.gradle.kts", '''plugins {
    alias(libs.plugins.androidLibrary)
    kotlin("android")
}

android {
    namespace = "com.selfie.ui.android"
    compileSdk = 34
    defaultConfig { minSdk = 24 }
    buildFeatures { compose = true }
}

dependencies {
    implementation(project(":shared-ui"))
    implementation(libs.compose.ui)
}''')

        create_file(self.root / "platform/android-bridge/src/main/kotlin/com/selfie/ui/android/CMPActivity.kt", '''package com.selfie.ui.android

import android.os.Bundle
import androidx.activity.ComponentActivity

class CMPActivity : ComponentActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // TODO: Setup CMP integration
    }
}''')

        # iOS bridge
        create_file(self.root / "platform/ios-bridge/Package.swift", '''// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "SelfieUIBridge",
    platforms: [.iOS(.v14)],
    products: [
        .library(name: "SelfieUIBridge", targets: ["SelfieUIBridge"]),
    ],
    targets: [
        .target(name: "SelfieUIBridge"),
    ]
)''')

        create_file(self.root / "platform/ios-bridge/Sources/SelfieUIBridge/CMPViewController.swift", '''import UIKit

public class CMPViewController: UIViewController {
    public override func viewDidLoad() {
        super.viewDidLoad()
        // TODO: Setup CMP integration
    }
}''')

    def create_tests(self):
        # Main integration tests (multiplatform)
        create_file(self.root / "integration-tests/build.gradle.kts", '''plugins {
    alias(libs.plugins.kotlinMultiplatform)
    alias(libs.plugins.androidLibrary)
    alias(libs.plugins.jetbrainsCompose)
}

kotlin {
    androidTarget()
    iosX64()
    iosArm64()
    iosSimulatorArm64()
    js(IR) { browser() }
    jvm()
    
    sourceSets {
        commonTest.dependencies {
            implementation(project(":shared-ui"))
            implementation(project(":design-system"))
            implementation(kotlin("test"))
            implementation(compose.ui.test)
        }
    }
}

android {
    namespace = "com.selfie.ui.tests"
    compileSdk = 34
    defaultConfig { minSdk = 24 }
}''')

        # Pure Android module for snapshot tests
        create_file(self.root / "snapshot-tests/build.gradle.kts", '''plugins {
    alias(libs.plugins.androidLibrary)
    kotlin("android")
    alias(libs.plugins.jetbrainsCompose)
}

android {
    namespace = "com.selfie.ui.snapshots"
    compileSdk = 34
    defaultConfig { minSdk = 24 }
    buildFeatures { compose = true }
}

dependencies {
    implementation(project(":shared-ui"))
    implementation(project(":design-system"))
    
    testImplementation("app.cash.paparazzi:paparazzi:1.3.1")
    testImplementation("junit:junit:4.13.2")
}''')

        create_file(self.root / "snapshot-tests/src/test/kotlin/SnapshotTests.kt", '''import app.cash.paparazzi.DeviceConfig.Companion.PIXEL_5
import app.cash.paparazzi.Paparazzi
import org.junit.Rule
import org.junit.Test

class SnapshotTests {
    @get:Rule
    val paparazzi = Paparazzi(deviceConfig = PIXEL_5)

    @Test
    fun loginScreenSnapshot() {
        // TODO: Implement snapshot test for LoginScreen
        // paparazzi.snapshot { SelfieTheme { LoginScreen() } }
    }

    @Test
    fun settingsScreenSnapshot() {
        // TODO: Implement snapshot test for SettingsScreen
        // paparazzi.snapshot { SelfieTheme { SettingsScreen() } }
    }
}''')

        # Regular multiplatform tests
        create_file(self.root / "integration-tests/src/commonTest/kotlin/ComposeUITest.kt", '''import kotlin.test.Test

class ComposeUITest {
    @Test
    fun testViewModelContract() {
        // TODO: Test ViewModel StateFlow contracts
        // Verify both CMP and native can bind to same interface
    }
    
    @Test
    fun testScreenNavigation() {
        // TODO: Test navigation between CMP screens
    }
}''')

        create_file(self.root / "integration-tests/src/commonTest/kotlin/UITest.kt", '''import kotlin.test.Test
import kotlin.test.assertTrue

class UITest {
    @Test
    fun testBasic() {
        assertTrue(true)
    }
}''')

        # README
        create_file(self.root / "README.md", '''# Selfie UI (Compose Multiplatform)

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
''')


def main():
    import argparse
    
    parser = argparse.ArgumentParser(description="Scaffold Selfie UI CMP Structure")
    parser.add_argument("--dir", default="selfie-ui", help="Root directory name")
    
    args = parser.parse_args()
    
    scaffolder = SelfieUIScaffolder(args.dir)
    scaffolder.scaffold()
    
    print(f"\nProject created in: {Path(args.dir).absolute()}")
    print("\nNext steps:")
    print("1. Move shared-ui/ into your selfie-kmp/ project")
    print("2. Update selfie-kmp/settings.gradle.kts to include new modules")
    print("3. Implement the TODO stubs with actual UI code")
    print("4. Connect to existing KMP business logic")


if __name__ == "__main__":
    main()
