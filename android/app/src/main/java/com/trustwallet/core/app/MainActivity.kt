package com.trustwallet.core.app

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import uniffi.wallet_modules.twExampleModuleRequest

class MainActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        twExampleModuleRequest(1u)
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)
    }
}
