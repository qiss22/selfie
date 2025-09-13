package com.selfie.ui.presentation

import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.MutableStateFlow

interface SettingsViewModelContract {
    val uiState: StateFlow<SettingsUiState>
    fun updateNotifications(enabled: Boolean)
    fun updatePrivacy(isPrivate: Boolean)
}

data class SettingsUiState(
    val notificationsEnabled: Boolean = true,
    val isPrivateAccount: Boolean = false,
    val isLoading: Boolean = false
)

class SettingsViewModel : SettingsViewModelContract {
    private val _uiState = MutableStateFlow(SettingsUiState())
    override val uiState: StateFlow<SettingsUiState> = _uiState
    
    override fun updateNotifications(enabled: Boolean) {
        // TODO: Connect to domain use cases
    }
    
    override fun updatePrivacy(isPrivate: Boolean) {
        // TODO: Connect to domain use cases
    }
}