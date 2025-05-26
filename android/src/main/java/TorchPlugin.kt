package com.plugin.torch

import android.app.Activity
import android.content.Context
import android.hardware.camera2.CameraAccessException
import android.hardware.camera2.CameraManager
import android.os.Build
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke

@InvokeArg
class ToggleArgs {
    var value: Boolean = false
}

@TauriPlugin
class TorchPlugin(private val activity: Activity): Plugin(activity) {
    private var isOn: Boolean = false
    private var cameraId: String? = null

    private fun getCameraIdWithTorch(): String? {
        val cameraManager = activity.getSystemService(Context.CAMERA_SERVICE) as CameraManager
        return cameraManager.cameraIdList.firstOrNull { id ->
            val characteristics = cameraManager.getCameraCharacteristics(id)
            val hasFlash = characteristics.get(android.hardware.camera2.CameraCharacteristics.FLASH_INFO_AVAILABLE) == true
            val isBackFacing = characteristics.get(android.hardware.camera2.CameraCharacteristics.LENS_FACING) ==
                android.hardware.camera2.CameraCharacteristics.LENS_FACING_BACK
            hasFlash && isBackFacing
        }
    }

    @Command
    fun toggle(invoke: Invoke) {
        val args = invoke.parseArgs(ToggleArgs::class.java)
        val turnOn = args.value

        val cameraManager = activity.getSystemService(Context.CAMERA_SERVICE) as CameraManager
        val id = cameraId ?: getCameraIdWithTorch().also { cameraId = it }

        if (id == null || Build.VERSION.SDK_INT < Build.VERSION_CODES.M) {
            invoke.resolve(JSObject().put("value", false))
            return
        }

        try {
            cameraManager.setTorchMode(id, turnOn)
            isOn = turnOn
            val ret = JSObject()
            ret.put("value", turnOn)
            invoke.resolve(ret)
        } catch (e: CameraAccessException) {
            isOn = false
            invoke.reject("Failed to access torch: ${e.message}")
        }
    }

    @Command
    fun check(invoke: Invoke) {
        val ret = JSObject()
        ret.put("value", isOn)
        invoke.resolve(ret)
    }
}
