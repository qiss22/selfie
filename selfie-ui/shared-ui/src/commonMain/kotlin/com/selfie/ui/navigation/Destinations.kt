package com.selfie.ui.navigation

sealed class Destination(val route: String) {
    object Login : Destination("auth/login")
    object Settings : Destination("settings")
    object Shop : Destination("shop")
    object Admin : Destination("admin")
}