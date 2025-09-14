package design.theme

import androidx.compose.material3.MaterialTheme
import androidx.compose.runtime.Composable
import design.tokens.SelfieColors
import design.tokens.SelfieSpacing
import design.tokens.SelfieTypography

@Composable
fun SelfieTheme(content: @Composable () -> Unit) {
    MaterialTheme(
        colorScheme = androidx.compose.material3.lightColorScheme(
            primary = SelfieColors.PrimaryBrown,
            secondary = SelfieColors.AccentGray,
            background = SelfieColors.LightGray,
            surface = SelfieColors.White,
            onPrimary = SelfieColors.White,
            onSecondary = SelfieColors.Black,
            onBackground = SelfieColors.Black,
            onSurface = SelfieColors.PrimaryBrown
        ),
        typography = SelfieTypography,
        content = content
    )
}
