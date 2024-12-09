package com.example.yattt_app.ui.theme

import android.os.Build
import androidx.compose.foundation.isSystemInDarkTheme
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.darkColorScheme
import androidx.compose.material3.dynamicDarkColorScheme
import androidx.compose.material3.dynamicLightColorScheme
import androidx.compose.material3.lightColorScheme
import androidx.compose.runtime.Composable
import androidx.compose.ui.platform.LocalContext

private val DarkColorScheme = darkColorScheme(
    primary = DarkColorPrimary,
    secondary = DarkColorSecondary,
    tertiary = DarkColorTertiary
)

private val LightColorScheme = lightColorScheme(
    primary = LightColorPrimary,
    secondary = LightColorSecondary,
    tertiary = LightColorTertiary
)

@Composable
fun YATTTTheme(
    theme: String, // Add this parameter to handle theme switching
    content: @Composable () -> Unit
) {
    val colors = when (theme) {
        "Light" -> LightColorScheme
        "Dark" -> DarkColorScheme
        else -> DarkColorScheme
    }

    MaterialTheme(
        colorScheme = colors,
        typography = Typography, // Ensure you have defined Typography
        content = content
    )
}