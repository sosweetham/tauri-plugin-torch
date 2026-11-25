use serde::{de::DeserializeOwned, Deserialize};
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::{models::*, Result};

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.plugin.torch";

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_torch);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> Result<Torch<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "TorchPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_torch)?;
    Ok(Torch(handle))
}

#[derive(Deserialize)]
struct AvailableResponse {
    available: bool,
}

#[derive(Deserialize)]
struct EnabledResponse {
    enabled: bool,
}

/// Access to the torch APIs.
pub struct Torch<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Torch<R> {
    /// Turns the torch on.
    ///
    /// # Errors
    ///
    /// Returns [`Error::PluginInvoke`](crate::Error::PluginInvoke) if the device has no torch,
    /// the torch is temporarily unavailable (e.g. the device is overheating) or the camera is
    /// held by another app.
    pub fn enable(&self) -> Result<()> {
        self.0.run_mobile_plugin("enable", ()).map_err(Into::into)
    }

    /// Turns the torch off.
    ///
    /// # Errors
    ///
    /// Same as [`Self::enable`].
    pub fn disable(&self) -> Result<()> {
        self.0.run_mobile_plugin("disable", ()).map_err(Into::into)
    }

    /// Turns the torch on or off when `payload.value` is set, and flips it otherwise. The
    /// response holds the new state.
    ///
    /// When `payload.value` is set and the device has no torch, this returns `false` instead of
    /// an error.
    ///
    /// # Errors
    ///
    /// Same as [`Self::enable`].
    pub fn toggle(&self, payload: TorchRequest) -> Result<TorchResponse> {
        let enabled = match payload.value {
            Some(_) if !self.is_available()? => false,
            Some(true) => self.enable().map(|()| true)?,
            Some(false) => self.disable().map(|()| false)?,
            None => {
                self.0
                    .run_mobile_plugin::<EnabledResponse>("toggle", ())?
                    .enabled
            }
        };
        Ok(TorchResponse {
            value: Some(enabled),
        })
    }

    /// Whether the torch is currently on.
    #[deprecated = "use `is_enabled` instead"]
    pub fn check(&self) -> Result<TorchResponse> {
        self.is_enabled().map(|enabled| TorchResponse {
            value: Some(enabled),
        })
    }

    /// Whether the device has a torch.
    pub fn is_available(&self) -> Result<bool> {
        self.0
            .run_mobile_plugin::<AvailableResponse>("isAvailable", ())
            .map(|r| r.available)
            .map_err(Into::into)
    }

    /// Whether the torch is currently on. Reads the hardware state, so it also reflects the torch
    /// being turned off by the system or another app.
    pub fn is_enabled(&self) -> Result<bool> {
        self.0
            .run_mobile_plugin::<EnabledResponse>("isEnabled", ())
            .map(|r| r.enabled)
            .map_err(Into::into)
    }
}
