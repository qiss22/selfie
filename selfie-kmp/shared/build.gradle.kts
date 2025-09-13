kotlin {{
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
