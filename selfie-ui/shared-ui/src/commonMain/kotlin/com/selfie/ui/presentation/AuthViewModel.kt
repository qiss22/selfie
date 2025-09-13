package com.selfie.ui.presentation

import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.MutableStateFlow

interface AuthViewModelContract {
    val uiState: StateFlow<AuthUiState>
    fun login(email: String, password: String)
    fun signUp(email: String, password: String, username: String)
}

data class AuthUiState(
    val isLoading: Boolean = false,
    val isSuccess: Boolean = false,
    val errorMessage: String? = null
)

class AuthViewModel : AuthViewModelContract {
    private val _uiState = MutableStateFlow(AuthUiState())
    override val uiState: StateFlow<AuthUiState> = _uiState
    
    override fun login(email: String, password: String) {
        // TODO: Connect to domain use cases from selfie-kmp
    }
    
    override fun signUp(email: String, password: String, username: String) {
        // TODO: Connect to domain use cases from selfie-kmp  
    }
}