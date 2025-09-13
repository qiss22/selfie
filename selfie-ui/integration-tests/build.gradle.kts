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
}