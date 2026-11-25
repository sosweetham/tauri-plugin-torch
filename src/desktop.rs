use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::{models::*, Result};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> Result<Torch<R>> {
    Ok(Torch(app.clone()))
}

fn unsupported() -> crate::Error {
    std::io::Error::new(
        std::io::ErrorKind::Unsupported,
        "The torch is not supported on this platform",
    )
    .into()
}

/// Access to the torch APIs.
///
/// Desktop has no torch: [`Self::is_available`], [`Self::is_enabled`], [`Self::check`] and the
/// legacy [`Self::toggle`] with `payload.value` set report it as off, and [`Self::enable`],
/// [`Self::disable`] and [`Self::toggle`] without a value return an
/// [`std::io::ErrorKind::Unsupported`] error.
pub struct Torch<R: Runtime>(#[allow(dead_code)] AppHandle<R>);

impl<R: Runtime> Torch<R> {
    /// Turns the torch on.
    pub fn enable(&self) -> Result<()> {
        Err(unsupported())
    }

    /// Turns the torch off.
    pub fn disable(&self) -> Result<()> {
        Err(unsupported())
    }

    /// Turns the torch on or off when `payload.value` is set, and flips it otherwise. The
    /// response holds the new state.
    pub fn toggle(&self, payload: TorchRequest) -> Result<TorchResponse> {
        match payload.value {
            // the deprecated `toggle(value)` reports a missing torch as off instead of failing
            Some(_) => Ok(TorchResponse { value: Some(false) }),
            None => Err(unsupported()),
        }
    }

    /// Whether the torch is currently on.
    #[deprecated = "use `is_enabled` instead"]
    pub fn check(&self) -> Result<TorchResponse> {
        Ok(TorchResponse { value: Some(false) })
    }

    /// Whether the device has a torch.
    pub fn is_available(&self) -> Result<bool> {
        Ok(false)
    }

    /// Whether the torch is currently on.
    pub fn is_enabled(&self) -> Result<bool> {
        Ok(false)
    }
}
