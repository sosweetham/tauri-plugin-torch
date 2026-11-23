//! Control the device flashlight (torch).
//!
//! - Supported platforms: Android and iOS. On desktop the plugin reports that no torch is
//!   available.
//!
//! On Android the torch keeps working while the
//! [barcode scanner](https://docs.rs/tauri-plugin-barcode-scanner) plugin holds the camera.

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};
pub use models::*;

#[cfg(desktop)]
pub use desktop::Torch;
#[cfg(mobile)]
pub use mobile::Torch;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`], [`tauri::WebviewWindow`], [`tauri::Webview`] and [`tauri::Window`] to access the torch APIs.
pub trait TorchExt<R: Runtime> {
    /// Returns the [`Torch`] instance managed by the app.
    fn torch(&self) -> &Torch<R>;
}

impl<R: Runtime, T: Manager<R>> crate::TorchExt<R> for T {
    fn torch(&self) -> &Torch<R> {
        self.state::<Torch<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("torch")
        .invoke_handler(tauri::generate_handler![
            commands::check,
            commands::enable,
            commands::disable,
            commands::toggle,
            commands::is_available,
            commands::is_enabled
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let torch = mobile::init(app, api)?;
            #[cfg(desktop)]
            let torch = desktop::init(app, api)?;
            app.manage(torch);
            Ok(())
        })
        .build()
}
