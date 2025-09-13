package com.selfie.designsystem

import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable

@Composable
fun SelfieTheme(
    darkTheme: Boolean = isSystemInDarkTheme(),
    content: @Composable () -> Unit
) {
    val colorScheme = if (darkTheme) {
        darkColorScheme(
            primary = SelfieColors.PrimaryDark,
            secondary = SelfieColors.SecondaryDark,
            background = SelfieColors.BackgroundDark
        )
    } else {
        lightColorScheme(
            primary = SelfieColors.Primary,
            secondary = SelfieColors.Secondary,
            background = SelfieColors.Background
        )
    }
    
    MaterialTheme(
        colorScheme = colorScheme,
        typography = SelfieTypography,
        content = content
    )
}