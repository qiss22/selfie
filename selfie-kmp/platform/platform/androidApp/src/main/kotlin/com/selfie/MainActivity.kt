package com.selfie.android

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import org.jetbrains.compose.ui.tooling.preview.Preview

class MainActivity : ComponentActivity() {{
    override fun onCreate(savedInstanceState: Bundle?) {{
        super.onCreate(savedInstanceState)
        setContent {{
            Greeting("Android")
        }}
    }}
}}

@Composable
fun Greeting(name: String) {{
    Text(text = "Hello $name!")
}}

@Preview
@Composable
fun DefaultPreview() {{
    SelfieTheme {{
        Surface {{
            Greeting("Android")
        }}
    }}
}}