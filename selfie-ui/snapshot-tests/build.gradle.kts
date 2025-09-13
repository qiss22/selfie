plugins {
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
}