package design.tokens

import androidx.compose.material3.Typography
import androidx.compose.ui.text.TextStyle
import androidx.compose.ui.text.font.Font
import androidx.compose.ui.text.font.FontFamily
import androidx.compose.ui.unit.sp
import org.jetbrains.compose.resources.ExperimentalResourceApi
import org.jetbrains.compose.resources.painterResource

// Add Cabinet Grotesk font files in /resources/fonts/ (TTF/OTF)
val CabinetGrotesk = FontFamily(
    Font("fonts/cabinetgrotesk-regular.ttf"),
    Font("fonts/cabinetgrotesk-medium.ttf"),
    Font("fonts/cabinetgrotesk-bold.ttf")
)

val SelfieTypography = Typography(
    displayLarge = TextStyle(
        fontFamily = CabinetGrotesk,
        fontSize = 36.sp
    ),
    headlineMedium = TextStyle(
        fontFamily = CabinetGrotesk,
        fontSize = 24.sp
    ),
    bodyLarge = TextStyle(
        fontFamily = CabinetGrotesk,
        fontSize = 16.sp
    ),
    labelLarge = TextStyle(
        fontFamily = CabinetGrotesk,
        fontSize = 14.sp
    )
)
