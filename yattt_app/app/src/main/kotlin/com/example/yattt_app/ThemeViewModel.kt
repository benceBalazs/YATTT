
import android.app.Application
import android.content.Context
import android.widget.Toast
import androidx.lifecycle.AndroidViewModel
import androidx.lifecycle.LiveData
import androidx.lifecycle.MutableLiveData

class ThemeViewModel(application: Application) : AndroidViewModel(application) {
    private val prefs = application.getSharedPreferences("AppPrefs", Context.MODE_PRIVATE)
    private val _isDarkTheme = MutableLiveData<Boolean>()

    val isDarkTheme: LiveData<Boolean> = _isDarkTheme

    init {
        _isDarkTheme.value = prefs.getBoolean("dark_theme", false)
    }

    fun toggleTheme() {
        val currentTheme = _isDarkTheme.value ?: false
        _isDarkTheme.postValue(!currentTheme)
        prefs.edit().putBoolean("dark_theme", !currentTheme).apply()
    }
}