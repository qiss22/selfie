#!/usr/bin/env python3
"""
Robust Python script to scaffold the Kotlin Multiplatform (KMP) beast for Selfie app.

This script generates the full monorepo structure as outlined, including:
- Root Gradle files (settings.gradle.kts, build.gradle.kts, gradle.properties).
- Shared module with source sets for common, android, ios, js, desktop.
- Feature modules (auth, home, etc.) with shared structure.
- Platform modules (androidApp, iosApp, webApp, desktopApp).
- Libraries, tools, buildSrc, scripts, ci, docs.

Requires Python 3.8+. Run with: python scaffold_selfie_kmp.py --root selfie-kmp

Options:
--root: Root directory name (default: selfie-kmp)
--package: Base package name (default: com.selfie)
--version: Project version (default: 0.1.0)

Uses templates for Gradle KTS files based on best practices from Kotlin docs and JetBrains.
"""

import os
import argparse
import shutil
from pathlib import Path

# Version info (as of Sep 2025)
KOTLIN_VERSION = "2.0.20"
COMPOSE_MP_VERSION = "1.6.10"
KTOR_VERSION = "2.3.12"
SQLDELIGHT_VERSION = "2.0.2"

# Templates as multiline strings
ROOT_BUILD_GRADLE_KTS = """// Top-level build file where you can add configuration options common to all sub-projects/modules.
plugins {{
    kotlin("multiplatform") version "{kotlin_version}" apply false
    id("com.android.application") version "8.8.0" apply false
    id("com.android.library") version "8.8.0" apply false
    id("org.jetbrains.compose") version "{compose_mp_version}" apply false
    id("app.cash.sqldelight") version "{sqldelight_version}" apply false
    id("maven-publish") apply false
}}

allprojects {{
    repositories {{
        google()
        mavenCentral()
        gradlePluginPortal()
        maven("https://maven.pkg.jetbrains.space/public/p/compose/dev")
    }}
}}

tasks.register("clean", Delete::class) {{
    delete(rootProject.buildDir)
}}
""".format(kotlin_version=KOTLIN_VERSION, compose_mp_version=COMPOSE_MP_VERSION, sqldelight_version=SQLDELIGHT_VERSION)

GRADLE_PROPERTIES = """# Gradle properties for KMP
kotlin.code.style=official
org.gradle.jvmargs=-Xmx4g -XX:MaxMetaspaceSize=512m
org.gradle.parallel=true
org.gradle.caching=true
kotlin.mpp.enableCInteropCommonization=true
android.useAndroidX=true
android.enableJetifier=true
"""

SETTINGS_GRADLE_KTS = """// Top-level build file where you can add configuration options common to all sub-projects/modules.
rootProject.name = "selfie-kmp"
include(":shared")
include(":features:auth")
include(":features:home")
include(":features:profile")
include(":features:chat")
include(":features:search")
include(":features:notifications")
include(":features:shop")
include(":features:settings")
include(":features:admin")
include(":platform:androidApp")
include(":platform:iosApp")
include(":platform:webApp")
include(":platform:desktopApp")
include(":libraries:selfie-ui")
include(":libraries:selfie-network")
include(":libraries:selfie-db")
include(":libraries:selfie-analytics")
include(":libraries:selfie-crypto")
include(":tools:codegen")
include(":buildSrc")
"""

VERSIONS_TOML = """[versions]
kotlin = "{kotlin_version}"
compose = "{compose_mp_version}"
ktor = "{ktor_version}"
sqldelight = "{sqldelight_version}"
android-gradle-plugin = "8.8.0"
hilt = "2.52"
room = "2.6.1"
stripe = "20.44.0"  # For shop

[libraries]
kotlinx-coroutines-core = {{ module = "org.jetbrains.kotlinx:kotlinx-coroutines-core", version.ref = "kotlin" }}
ktor-client-core = {{ module = "io.ktor:ktor-client-core", version.ref = "ktor" }}
sqldelight-runtime = {{ module = "app.cash.sqldelight:runtime", version.ref = "sqldelight" }}

[plugins]
kotlin-multiplatform = {{ id = "org.jetbrains.kotlin.multiplatform", version.ref = "kotlin" }}
""".format(kotlin_version=KOTLIN_VERSION, compose_mp_version=COMPOSE_MP_VERSION, ktor_version=KTOR_VERSION, sqldelight_version=SQLDELIGHT_VERSION)

SHARED_BUILD_GRADLE_KTS = """kotlin {{
    android {{
        compilations.all {{
            kotlinOptions {{
                jvmTarget = "17"
            }}
        }}
    }}
    
    listOf(
        iosX64(),
        iosArm64(),
        iosSimulatorArm64()
    ).forEach {{ iosTarget ->
        val platformName = "ios${{iosTarget.name}}"
        val binaryName = "shared${{platformName.replaceFirstChar {{ it.uppercase() }}}}"
        
        iosTarget.binaries.framework {{
            baseName = "shared"
            isStatic = true
            export(project(":composeApp"))
            linkerOpts.add("-rpath", "@executable_path/Frameworks")
        }}
    }}
    
    sourceSets {{
        val commonMain by getting {{
            dependencies {{
                implementation(compose.runtime)
                implementation(compose.foundation)
                implementation(compose.material3)
                implementation(libs.kotlinx.coroutines.core)
                implementation(libs.ktor.client.core)
                implementation(libs.sqldelight.runtime)
            }}
        }}
        val androidMain by getting {{
            dependencies {{
                implementation(libs.ktor.client.okhttp)
            }}
        }}
        val iosMain by getting {{
            dependencies {{
                implementation(libs.ktor.client.darwin)
            }}
        }}
        val jsMain by getting {{
            dependencies {{
                implementation(libs.ktor.client.js)
            }}
        }}
        val desktopMain by getting {{
            dependencies {{
                implementation(libs.ktor.client.apache5)
            }}
        }}
    }}
}}

sqldelight {{
    databases {{
        create("SelfieDatabase") {{
            packageName = "com.selfie.shared.data.local"
            dialect("org.intellij.lang.annotations.Language") = "SQLDELIGHT_W3C_SQL"
            srcDirs("sqldelight/.sq")
        }}
    }}
}}
"""

FEATURE_BUILD_GRADLE_KTS_TEMPLATE = """// Feature module build.gradle.kts
plugins {{
    kotlin("multiplatform")
    id("com.android.library")
    kotlin("plugin.compose") version "{kotlin_version}"
}}

kotlin {{
    // Similar to shared
}}

dependencies {{
    // Inherit from shared
    implementation(project(":shared"))
    commonMain.dependencies {{
        implementation(compose.runtime)
    }}
}}
"""

PLATFORM_ANDROID_BUILD_GRADLE_KTS = """plugins {{
    id("com.android.application")
    kotlin("android")
    id("org.jetbrains.compose")
    id("kotlin-parcelize")
}}

android {{
    namespace = "{package}.android"
    compileSdk = 35
    
    defaultConfig {{
        applicationId = "{package}"
        minSdk = 24
        targetSdk = 35
        versionCode = 1
        versionName = "{version}"
    }}
    
    compileOptions {{
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }}
    
    buildFeatures {{
        compose = true
    }}
    
    composeOptions {{
        kotlinCompilerExtensionVersion = "{compose_mp_version}"
    }}
}}

dependencies {{
    implementation(project(":shared"))
    implementation(compose.ui)
    implementation(compose.ui.tooling.preview)
    implementation(compose.material3)
    // Hilt, etc.
}}
"""

# List of features
FEATURES = ["auth", "home", "profile", "chat", "search", "notifications", "shop", "settings", "admin"]

# List of libraries
LIBRARIES = ["selfie-ui", "selfie-network", "selfie-db", "selfie-analytics", "selfie-crypto"]

# List of tools
TOOLS = ["codegen"]

def mkdir_p(path: Path):
    path.mkdir(parents=True, exist_ok=True)

def write_file(path: Path, content: str):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content)
    print(f"Created {path}")

def copy_template_if_exists(src: Path, dst: Path):
    if src.exists():
        shutil.copy2(src, dst)
        print(f"Copied {src} to {dst}")

def generate_shared_module(base_dir: Path, package_name: str):
    shared_dir = base_dir / "shared"
    mkdir_p(shared_dir / "src" / "commonMain" / "kotlin" / package_name.replace(".", "/") / "domain")
    mkdir_p(shared_dir / "src" / "commonMain" / "kotlin" / package_name.replace(".", "/") / "data")
    mkdir_p(shared_dir / "src" / "commonMain" / "kotlin" / package_name.replace(".", "/") / "presentation")
    mkdir_p(shared_dir / "src" / "androidMain" / "kotlin" / package_name.replace(".", "/"))
    mkdir_p(shared_dir / "src" / "iosMain" / "kotlin" / package_name.replace(".", "/"))
    mkdir_p(shared_dir / "src" / "jsMain" / "kotlin" / package_name.replace(".", "/"))
    mkdir_p(shared_dir / "src" / "desktopMain" / "kotlin" / package_name.replace(".", "/"))
    mkdir_p(shared_dir / "src" / "commonTest" / "kotlin" / package_name.replace(".", "/"))
    
    # Sample files
    (shared_dir / "src" / "commonMain" / "kotlin" / package_name.replace(".", "/") / "domain" / "User.kt").write_text(
        """data class User(val id: String, val name: String)"""
    )
    
    write_file(shared_dir / "build.gradle.kts", SHARED_BUILD_GRADLE_KTS)
    mkdir_p(shared_dir / "sqldelight" / ".sq")
    (shared_dir / "sqldelight" / ".sq" / "SelfieDatabase.sq").write_text(
        """CREATE TABLE User (\n    id TEXT NOT NULL PRIMARY KEY,\n    name TEXT NOT NULL\n);"""
    )

def generate_feature_module(base_dir: Path, feature: str, package_name: str):
    feat_dir = base_dir / "features" / feature
    mkdir_p(feat_dir / "src" / "commonMain" / "kotlin" / f"{package_name}.features.{feature}".replace(".", "/") / "domain")
    mkdir_p(feat_dir / "src" / "commonMain" / "kotlin" / f"{package_name}.features.{feature}".replace(".", "/") / "presentation")
    # Add more subdirs as needed
    
    write_file(feat_dir / "build.gradle.kts", FEATURE_BUILD_GRADLE_KTS_TEMPLATE.format(kotlin_version=KOTLIN_VERSION))

def generate_platform_android(base_dir: Path, package_name: str, version: str):
    android_dir = base_dir / "platform" / "androidApp"
    mkdir_p(android_dir / "src" / "main" / "kotlin" / package_name.replace(".", "/"))
    mkdir_p(android_dir / "src" / "main" / "res" / "values")
    
    (android_dir / "src" / "main" / "kotlin" / package_name.replace(".", "/") / "MainActivity.kt").write_text(
        """package {package}.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import org.jetbrains.compose.ui.tooling.preview.Preview

class MainActivity : ComponentActivity() {{
    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)
        setContent {{
            Greeting("Android")
        }}
    }}
}}

@Composable
fun Greeting(name: String) {{
    Text(text = "Hello $name!")
}}

@Preview
@Composable
fun DefaultPreview() {{
    SelfieTheme {{
        Surface {{
            Greeting("Android")
        }}
    }}
}}""".replace("{package}", package_name)
    )
    
    (android_dir / "src" / "main" / "AndroidManifest.xml").write_text(
        f"""<?xml version="1.0" encoding="utf-8"?>
<manifest xmlns:android="http://schemas.android.com/apk/res/android"
    package="{package_name}">

    <application
        android:allowBackup="true"
        android:label="@string/app_name"
        android:theme="@style/Theme.Selfie">
        <activity
            android:name=".MainActivity"
            android:exported="true">
            <intent-filter>
                <action android:name="android.intent.action.MAIN" />
                <category android:name="android.intent.category.LAUNCHER" />
            </intent-filter>
        </activity>
    </application>
</manifest>"""
    )
    
    write_file(android_dir / "build.gradle.kts", PLATFORM_ANDROID_BUILD_GRADLE_KTS.format(package=package_name, version=version, compose_mp_version=COMPOSE_MP_VERSION, package_name=package_name))

def generate_platform_ios(base_dir: Path, package_name: str):
    ios_dir = base_dir / "platform" / "iosApp"
    # For iOS, we create a basic Xcode project stub; in practice, use Tuist or manual Xcode
    mkdir_p(ios_dir / "Selfie")
    (ios_dir / "Selfie" / "SelfieApp.swift").write_text(
        """import SwiftUI
import SharedCode  // KMP framework

@main
struct SelfieApp: App {{
    var body: some Scene {{
        WindowGroup {{
            ContentView()
        }}
    }}
}}

struct ContentView: View {{
    var body: some View {{
        Text("Hello, iOS from KMP!")
    }}
}}"""
    )
    # Note: Real iOS setup requires Xcode; this is a placeholder. Run `xcodeproj` or Tuist post-script.
    write_file(ios_dir / "build.gradle.kts", 
        """kotlin {{
            ios {{
                binaries {{
                    framework {{
                        baseName = "shared"
                        isStatic = true
                    }}
                }}
            }}
        }}"""
    )

def generate_platform_web(base_dir: Path, package_name: str):
    web_dir = base_dir / "platform" / "webApp"
    mkdir_p(web_dir / "src" / "jsMain" / "kotlin" / package_name.replace(".", "/"))
    (web_dir / "src" / "jsMain" / "kotlin" / package_name.replace(".", "/") / "index.kt").write_text(
        """package {package}.web

import androidx.compose.ui.window.CanvasBasedWindow
import org.jetbrains.skiko.wasm.onWasmReady

fun main() {{
    onWasmReady {{
        CanvasBasedWindow("Selfie Web") {{
            App()
        }}
    }}
}}""".replace("{package}", package_name)
    )
    (web_dir / "index.html").write_text(
        """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <title>Selfie Web</title>
    <script src="/static/kotlin/selfie-kmp.js"></script>
</head>
<body>
    <div id="root"></div>
    <script>
        window.main();
    </script>
</body>
</html>"""
    )
    write_file(web_dir / "build.gradle.kts", 
        """kotlin {{
            js(IR) {{
                browser {{
                    webpackTask {{
                        outputFileName = "selfie-kmp.js"
                    }}
                }}
                binaries.executable()
            }}
        }}"""
    )
    (web_dir / "package.json").write_text(
        """{{
  "name": "selfie-web",
  "version": "0.1.0",
  "dependencies": {{
    "vite": "^5.0.0"
  }},
  "scripts": {{
    "dev": "vite",
    "build": "vite build"
  }}
}}"""
    )

def generate_platform_desktop(base_dir: Path, package_name: str):
    desktop_dir = base_dir / "platform" / "desktopApp"
    mkdir_p(desktop_dir / "src" / "jvmMain" / "kotlin" / package_name.replace(".", "/"))
    (desktop_dir / "src" / "jvmMain" / "kotlin" / package_name.replace(".", "/") / "Main.kt").write_text(
        """package {package}.desktop

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import org.jetbrains.skiko.wasm.onWasmReady

fun main() = application {{
    Window(onCloseRequest = ::exitApplication, title = "Selfie Desktop") {{
        App()
    }}
}}""".replace("{package}", package_name)
    )
    write_file(desktop_dir / "build.gradle.kts", 
        """kotlin {{
            jvm {{
                compilations.all {{
                    kotlinOptions.jvmTarget = "17"
                }}
                withJava()
                binaries.all {{
                    "jar"
                }}
            }}
        }}
        compose.desktop {{
            application {{
                mainClass = "{package}.desktop.MainKt"
                projectName = "Selfie Desktop"
                projectVersion = "1.0.0"
                packageName = "{package}"
            }}
        }}""".format(package=package_name)
    )

def generate_library_module(base_dir: Path, lib: str):
    lib_dir = base_dir / "libraries" / lib
    mkdir_p(lib_dir / "src" / "commonMain" / "kotlin")
    write_file(lib_dir / "build.gradle.kts", 
        """plugins {{
    kotlin("multiplatform")
    id("maven-publish")
}}

kotlin {{
    // Targets as in shared
}}

publishing {{
    publications {{
        maven(MavenPublication) {{
            from(components["kotlin"])
        }}
    }}
}}"""
    )

def generate_tool_module(base_dir: Path, tool: str):
    tool_dir = base_dir / "tools" / tool
    mkdir_p(tool_dir / "src" / "main" / "kotlin")
    write_file(tool_dir / "build.gradle.kts", 
        """plugins {{
    kotlin("multiplatform")
}}

kotlin {{
    js(IR) {{
        browser()
    }}
}}"""
    )

def generate_build_src(base_dir: Path):
    buildsrc_dir = base_dir / "buildSrc"
    mkdir_p(buildsrc_dir / "src" / "main" / "kotlin")
    (buildsrc_dir / "src" / "main" / "kotlin" / "Dependencies.kt").write_text(
        """object Dependencies {{
    object Kotlin {{
        const val version = "{kotlin_version}"
    }}
}}""".format(kotlin_version=KOTLIN_VERSION)
    )
    write_file(buildsrc_dir / "build.gradle.kts", 
        """plugins {{
    `kotlin-dsl`
}}"""
    )

def generate_scripts(base_dir: Path):
    scripts_dir = base_dir / "scripts"
    mkdir_p(scripts_dir)
    (scripts_dir / "generate-models.kts").write_text(
        """// Sample Kotlin script for codegen
println("Generating models from proto...")"""
    )
    (scripts_dir / "lint-all.sh").write_text(
        """#!/bin/bash
./gradlew ktlintCheck detekt"""
    )
    scripts_dir.joinpath("lint-all.sh").chmod(0o755)

def generate_ci(base_dir: Path):
    ci_dir = base_dir / "ci"
    mkdir_p(ci_dir)
    (ci_dir / "gradle.yml").write_text(
        """# GitHub Actions workflow for Gradle
name: CI
on: [push, pull_request]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    - uses: actions/setup-java@v4
      with:
        java-version: '17'
    - run: ./gradlew build"""
    )

def generate_docs(base_dir: Path):
    docs_dir = base_dir / "docs"
    mkdir_p(docs_dir)
    (docs_dir / "README.md").write_text(
        """# Selfie KMP Docs
Architecture diagrams here."""
    )

def main():
    parser = argparse.ArgumentParser(description="Scaffold Selfie KMP beast")
    parser.add_argument("--root", default="selfie-kmp", help="Root dir name")
    parser.add_argument("--package", default="com.selfie", help="Base package")
    parser.add_argument("--version", default="0.1.0", help="Project version")
    args = parser.parse_args()
    
    root_dir = Path(args.root)
    if root_dir.exists():
        print(f"Error: {root_dir} already exists. Remove or choose another name.")
        return
    
    print(f"Scaffolding Selfie KMP in {root_dir}...")
    
    # Root files
    mkdir_p(root_dir)
    write_file(root_dir / "build.gradle.kts", ROOT_BUILD_GRADLE_KTS)
    write_file(root_dir / "settings.gradle.kts", SETTINGS_GRADLE_KTS)
    write_file(root_dir / "gradle.properties", GRADLE_PROPERTIES)
    write_file(root_dir / "gradle" / "libs.versions.toml", VERSIONS_TOML)
    write_file(root_dir / "README.md", "# Selfie KMP\nCross-platform beast.")
    
    # Gradle wrapper
    os.system(f"cd {root_dir} && gradle wrapper --gradle-version 8.10")
    
    # Shared
    generate_shared_module(root_dir, args.package)
    
    # Features
    features_dir = root_dir / "features"
    mkdir_p(features_dir)
    for feat in FEATURES:
        generate_feature_module(features_dir, feat, args.package)
    
    # Platforms
    platform_dir = root_dir / "platform"
    mkdir_p(platform_dir)
    generate_platform_android(platform_dir, args.package, args.version)
    generate_platform_ios(platform_dir, args.package)
    generate_platform_web(platform_dir, args.package)
    generate_platform_desktop(platform_dir, args.package)
    
    # Libraries
    libs_dir = root_dir / "libraries"
    mkdir_p(libs_dir)
    for lib in LIBRARIES:
        generate_library_module(libs_dir, lib)
    
    # Tools
    tools_dir = root_dir / "tools"
    mkdir_p(tools_dir)
    for tool in TOOLS:
        generate_tool_module(tools_dir, tool)
    
    # buildSrc
    generate_build_src(root_dir)
    
    # scripts, ci, docs
    generate_scripts(root_dir)
    generate_ci(root_dir)
    generate_docs(root_dir)
    
    # .gitignore
    gitignore_content = """# Gradle
.gradle
build/
.idea/
.DS_Store
iosApp/build/
iosApp/**/*.xcworkspace
iosApp/iosApp.xcworkspace/
node_modules/
DerivedData/
"""
    write_file(root_dir / ".gitignore", gitignore_content)
    
    print("Scaffolding complete! Run `cd {root_dir} && ./gradlew build` to verify.".format(root_dir=root_dir))
    print("For iOS: Open iosApp/Selfie.xcodeproj in Xcode (may need manual setup).")
    print("For Web: npm install in webApp && npm run dev")

if __name__ == "__main__":
    main()
