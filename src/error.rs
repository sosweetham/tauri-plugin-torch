use serde::{ser::Serializer, Serialize};

/// Alias for the result type returned by this crate's functions.
pub type Result<T> = std::result::Result<T, Error>;

/// The error type returned by this crate's APIs. Serialized as its [`Display`](std::fmt::Display)
/// string when it crosses the IPC boundary.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// An I/O error. Also returned by [`Torch::enable`](crate::Torch::enable) and
    /// [`Torch::disable`](crate::Torch::disable) on desktop, with
    /// [`std::io::ErrorKind::Unsupported`].
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Failed to run a command on the mobile plugin implementation (Kotlin on Android, Swift on
    /// iOS). This includes the native side rejecting the call, e.g. because the device has no
    /// torch or the camera is held by another app.
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
