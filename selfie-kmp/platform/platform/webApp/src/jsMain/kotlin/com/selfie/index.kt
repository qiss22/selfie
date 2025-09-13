package com.selfie.web

import androidx.compose.ui.window.CanvasBasedWindow
import org.jetbrains.skiko.wasm.onWasmReady

fun main() {{
    onWasmReady {{
        CanvasBasedWindow("Selfie Web") {{
            App()
        }}
    }}
}}