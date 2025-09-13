package com.selfie.desktop

import androidx.compose.ui.window.Window
import androidx.compose.ui.window.application
import org.jetbrains.skiko.wasm.onWasmReady

fun main() = application {{
    Window(onCloseRequest = ::exitApplication, title = "Selfie Desktop") {{
        App()
    }}
}}