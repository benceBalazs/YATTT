package com.example.yattt_app

import android.app.PendingIntent
import android.content.Intent
import android.content.IntentFilter
import android.nfc.NfcAdapter
import android.nfc.Tag
import android.nfc.tech.NfcF
import android.os.Build
import android.os.Bundle
import android.provider.Settings
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import androidx.fragment.app.Fragment
import androidx.fragment.app.FragmentTransaction
import com.example.yattt_app.databinding.ActivityMainBinding
import com.example.yattt_app.helper.getUniqueDeviceId

class MainActivity : AppCompatActivity() {
    private lateinit var binding: ActivityMainBinding
    private var nfcAdapter: NfcAdapter? = null
    private var isDarkTheme: Boolean = true
    private lateinit var pendingIntent: PendingIntent
    private lateinit var intentFiltersArray: Array<IntentFilter>
    private lateinit var techListsArray: Array<Array<String>>
    private lateinit var deviceId: String

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        val prefs = getSharedPreferences("ThemePrefs", MODE_PRIVATE)
        isDarkTheme = prefs.getBoolean("isDarkTheme", false)
        updateTheme(false)
        binding = ActivityMainBinding.inflate(layoutInflater)
        setContentView(binding.root)

        if (checkNfcCapabilities()) return

        deviceId = getUniqueDeviceId(contentResolver)

        pendingIntent = PendingIntent.getActivity(
            this, 0, Intent(this, javaClass).addFlags(Intent.FLAG_ACTIVITY_SINGLE_TOP), PendingIntent.FLAG_MUTABLE
        )

        val tag = IntentFilter(NfcAdapter.ACTION_TAG_DISCOVERED)
        intentFiltersArray = arrayOf(tag)
        techListsArray = arrayOf(arrayOf(NfcF::class.java.name))

        initComponents()
    }

    private fun checkNfcCapabilities(): Boolean {
        nfcAdapter = NfcAdapter.getDefaultAdapter(this)
        if (nfcAdapter == null) {
            Toast.makeText(this, "NFC is not available", Toast.LENGTH_LONG).show()
            finish()
            return true
        }

        if (!nfcAdapter!!.isEnabled) {
            Toast.makeText(this, "NFC is disabled", Toast.LENGTH_LONG).show()
            startActivity(Intent(Settings.ACTION_NFC_SETTINGS))
            finish()
            return true
        }
        return false
    }

    override fun onPause() {
        super.onPause()
        nfcAdapter?.disableForegroundDispatch(this)
    }

    override fun onResume() {
        super.onResume()
        nfcAdapter?.enableForegroundDispatch(this, pendingIntent, intentFiltersArray, techListsArray)
    }

    override fun onNewIntent(intent: Intent?) {
        super.onNewIntent(intent)

        if (intent?.action == NfcAdapter.ACTION_TAG_DISCOVERED) {
            @Suppress("DEPRECATION")
            val tag = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.TIRAMISU) {
                intent.getParcelableExtra(NfcAdapter.EXTRA_TAG, Tag::class.java)
            } else {
                intent.getParcelableExtra(NfcAdapter.EXTRA_TAG)
            }

            tag?.id?.let {
                val tagId = it.toHexString()
                //Toast.makeText(this, "NFC tag detected: $tagId", Toast.LENGTH_SHORT).show()

                // TODO: send Token to API
            }
        }
    }

    private fun initComponents() {
        var fragment: Fragment = TokenFragment.newInstance()
        replaceFragment(fragment)

        binding.bottomNav.setOnItemSelectedListener { item ->
            when (item.itemId) {
                R.id.tokens -> {
                    fragment = TokenFragment.newInstance()
                    replaceFragment(fragment)
                    true
                }
                R.id.emulator -> {
                    fragment = EmulatorFragment.newInstance()
                    replaceFragment(fragment)
                    true
                }
                R.id.settings -> {
                    fragment = SettingsFragment.newInstance()
                    replaceFragment(fragment)
                    true
                }

                else -> false
            }
        }
    }

    private fun replaceFragment(fragment: Fragment) {
        val fragmentTransaction: FragmentTransaction = supportFragmentManager.beginTransaction()
        fragmentTransaction.setCustomAnimations(android.R.anim.fade_in, android.R.anim.fade_out)
        fragmentTransaction.replace(R.id.content, fragment, "")
        fragmentTransaction.commit()
    }

    fun updateTheme(shouldRecreate: Boolean = true) {
        if (isDarkTheme) {
            setTheme(R.style.Theme_Yattt_app_Dark)
        } else {
            setTheme(R.style.Theme_Yattt_app_Light)
        }
        val prefs = getSharedPreferences("ThemePrefs", MODE_PRIVATE)
        prefs.edit().putBoolean("isDarkTheme", isDarkTheme).apply()
        isDarkTheme = !isDarkTheme

        if (shouldRecreate) {
            recreate()
        }
    }

}

private fun ByteArray.toHexString(): String {
    return joinToString(separator = "") { eachByte -> "%02x".format(eachByte) }
}
