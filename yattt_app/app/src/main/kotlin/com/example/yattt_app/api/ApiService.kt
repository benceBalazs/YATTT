package com.example.yattt_app.api

import retrofit2.Call
import retrofit2.http.Body
import retrofit2.http.Header
import retrofit2.http.POST

data class TagInfo(val tag_id: String, val device_id: String)
data class ResponseModel(val status: String)

interface ApiService {
    @POST("api/v1/auth-tokens/scanin")
    fun submitTagInfo(@Header("Authorization") authToken: String, @Body tagInfo: TagInfo): Call<ResponseModel>
}