plugins {
    id("com.android.application")
    kotlin("android")
    id("org.jetbrains.compose")
    id("kotlin-parcelize")
}

android {
    namespace = "com.selfie.android"
    compileSdk = 35
    
    defaultConfig {
        applicationId = "com.selfie"
        minSdk = 24
        targetSdk = 35
        versionCode = 1
        versionName = "0.1.0"
    }
    
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    
    buildFeatures {
        compose = true
    }
    
    composeOptions {
        kotlinCompilerExtensionVersion = "1.6.10"
    }
}

dependencies {
    implementation(project(":shared"))
    implementation(compose.ui)
    implementation(compose.ui.tooling.preview)
    implementation(compose.material3)
    // Hilt, etc.
}
