package com.example.yattt_app

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import com.example.yattt_app.databinding.FragmentSettingsBinding

class SettingsFragment : Fragment() {
    private lateinit var binding: FragmentSettingsBinding

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        binding = FragmentSettingsBinding.inflate(inflater, container, false)
        initComponents()
        return binding.root
    }

    companion object {
        @JvmStatic
        fun newInstance() =
            SettingsFragment().apply {
            }
    }

    private fun initComponents() {
        binding.btnSwitchTheme.setOnClickListener {
            (activity as? MainActivity)?.updateTheme(true)
        }
    }
}
