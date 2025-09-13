kotlin {
            jvm {
                compilations.all {
                    kotlinOptions.jvmTarget = "17"
                }
                withJava()
                binaries.all {
                    "jar"
                }
            }
        }
        compose.desktop {
            application {
                mainClass = "com.selfie.desktop.MainKt"
                projectName = "Selfie Desktop"
                projectVersion = "1.0.0"
                packageName = "com.selfie"
            }
        }