package com.example.yattt_app.helper

import android.app.Activity
import android.util.Log
import android.view.View
import com.example.yattt_app.api.ResponseModel
import com.example.yattt_app.api.RetrofitClient
import com.example.yattt_app.api.TagInfo
import com.google.android.material.snackbar.Snackbar
import retrofit2.Call
import retrofit2.Callback
import retrofit2.Response

fun sendTokenInfo(activity: Activity, view: View, tagId: String, deviceId: String) {
    val tagInfo = TagInfo(tagId, deviceId)
    val call = RetrofitClient.instance.submitTagInfo("Token",tagInfo)
    Log.d("Retrofit", "Request URL: ${call.request().url()}")

    call.enqueue(object : Callback<ResponseModel> {
        override fun onResponse(call: Call<ResponseModel>, response: Response<ResponseModel>) {
            if (response.isSuccessful) {
                val status = response.body()?.status
                when (status) {
                    "SCANNED_IN" -> showSnackbar(
                        activity,
                        view,
                        "Data sent successfully",
                        view.context.getColor(android.R.color.holo_green_light)
                    )

                    "ALREADY" -> showSnackbar(
                        activity,
                        view,
                        "Tag already submitted",
                        view.context.getColor(android.R.color.holo_orange_light)
                    )

                    "NOT_ALLOWED" -> showSnackbar(
                        activity,
                        view,
                        "Submission not allowed",
                        view.context.getColor(android.R.color.holo_red_light)
                    )

                    else -> showSnackbar(
                        activity,
                        view,
                        "Unknown status: $status",
                        view.context.getColor(android.R.color.holo_blue_light)
                    )
                }
            }
        }


        override fun onFailure(call: Call<ResponseModel>, t: Throwable) {
            showSnackbar(
                activity,
                view,
                "Error: ${t.message}",
                view.context.getColor(android.R.color.holo_green_light)
            )
        }
    })
}

private fun showSnackbar(activity: Activity, view: View, message: String, color: Int) {
    activity.runOnUiThread {
        Snackbar.make(view, message, Snackbar.LENGTH_LONG)
            .setBackgroundTint(color)
            .show()
    }
}