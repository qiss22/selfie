plugins {
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
}