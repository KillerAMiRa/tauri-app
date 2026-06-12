package com.chens.s

import android.os.Bundle
import androidx.activity.enableEdgeToEdge
import com.tencent.bugly.crashreport.CrashReport

class MainActivity : TauriActivity() {
  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    
    // 初始化 Bugly
    CrashReport.initCrashReport(applicationContext, "32b06e6be7", false)
  }
}
