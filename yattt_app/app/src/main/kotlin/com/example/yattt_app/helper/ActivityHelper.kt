package com.example.yattt_app.helper

import android.annotation.SuppressLint
import android.content.ContentResolver
import android.provider.Settings

@SuppressLint("HardwareIds")
fun getUniqueDeviceId(resolver: ContentResolver): String {
    return Settings.Secure.getString(resolver, Settings.Secure.ANDROID_ID) ?: ""
}