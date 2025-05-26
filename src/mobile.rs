use serde::de::DeserializeOwned;
use tauri::{
    plugin::{PluginApi, PluginHandle},
    AppHandle, Runtime,
};

use crate::models::*;

#[cfg(target_os = "ios")]
tauri::ios_plugin_binding!(init_plugin_torch);

// initializes the Kotlin or Swift plugin classes
pub fn init<R: Runtime, C: DeserializeOwned>(
    _app: &AppHandle<R>,
    api: PluginApi<R, C>,
) -> crate::Result<Torch<R>> {
    #[cfg(target_os = "android")]
    let handle = api.register_android_plugin("com.plugin.torch", "TorchPlugin")?;
    #[cfg(target_os = "ios")]
    let handle = api.register_ios_plugin(init_plugin_torch)?;
    Ok(Torch(handle))
}

/// Access to the torch APIs.
pub struct Torch<R: Runtime>(PluginHandle<R>);

impl<R: Runtime> Torch<R> {
    pub fn toggle(&self, payload: TorchRequest) -> crate::Result<TorchResponse> {
        self.0
            .run_mobile_plugin("toggle", payload)
            .map_err(Into::into)
    }
    pub fn check(&self) -> crate::Result<TorchResponse> {
        self.0.run_mobile_plugin("check", ()).map_err(Into::into)
    }
}
