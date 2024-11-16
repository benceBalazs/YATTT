package com.example.yattt_app

import android.os.Bundle
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import androidx.fragment.app.Fragment
import com.example.yattt_app.databinding.FragmentEmulatorBinding
import com.example.yattt_app.helper.getUniqueDeviceId

class EmulatorFragment : Fragment() {
    private lateinit var binding: FragmentEmulatorBinding

    override fun onCreateView(
        inflater: LayoutInflater, container: ViewGroup?,
        savedInstanceState: Bundle?
    ): View {
        binding = FragmentEmulatorBinding.inflate(inflater, container, false)

        initComponents()

        return binding.root
    }

    companion object {
        @JvmStatic
        fun newInstance() =
            EmulatorFragment().apply {
            }
    }

    private fun initComponents() {
        binding.NFC.setOnClickListener {
            val tagId = "04d026827b4880"
            val deviceId = getUniqueDeviceId(binding.root.context.contentResolver)
            // TODO: send Token to API
        }

        binding.RFID.setOnClickListener {
            val tagId = "04db28827b4880"
            val deviceId = getUniqueDeviceId(binding.root.context.contentResolver)
            // TODO: send Token to API
        }
    }
}