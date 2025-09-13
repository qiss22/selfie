plugins {
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
}