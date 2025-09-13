// Feature module build.gradle.kts
plugins {
    kotlin("multiplatform")
    id("com.android.library")
    kotlin("plugin.compose") version "2.0.20"
}

kotlin {
    // Similar to shared
}

dependencies {
    // Inherit from shared
    implementation(project(":shared"))
    commonMain.dependencies {
        implementation(compose.runtime)
    }
}
