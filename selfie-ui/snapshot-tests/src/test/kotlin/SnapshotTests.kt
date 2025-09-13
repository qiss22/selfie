import app.cash.paparazzi.DeviceConfig.Companion.PIXEL_5
import app.cash.paparazzi.Paparazzi
import org.junit.Rule
import org.junit.Test

class SnapshotTests {
    @get:Rule
    val paparazzi = Paparazzi(deviceConfig = PIXEL_5)

    @Test
    fun loginScreenSnapshot() {
        // TODO: Implement snapshot test for LoginScreen
        // paparazzi.snapshot { SelfieTheme { LoginScreen() } }
    }

    @Test
    fun settingsScreenSnapshot() {
        // TODO: Implement snapshot test for SettingsScreen
        // paparazzi.snapshot { SelfieTheme { SettingsScreen() } }
    }
}