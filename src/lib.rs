use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::Torch;
#[cfg(mobile)]
use mobile::Torch;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the torch APIs.
pub trait TorchExt<R: Runtime> {
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
        .invoke_handler(tauri::generate_handler![commands::toggle, commands::check])
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
