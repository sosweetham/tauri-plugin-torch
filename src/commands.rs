use tauri::{command, AppHandle, Runtime};

use crate::{models::*, Result, TorchExt};

#[derive(serde::Serialize)]
pub(crate) struct AvailableResponse {
    available: bool,
}

#[derive(serde::Serialize)]
pub(crate) struct EnabledResponse {
    enabled: bool,
}

#[command]
pub(crate) async fn enable<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.torch().enable()
}

#[command]
pub(crate) async fn disable<R: Runtime>(app: AppHandle<R>) -> Result<()> {
    app.torch().disable()
}

#[command]
pub(crate) async fn toggle<R: Runtime>(
    app: AppHandle<R>,
    payload: Option<TorchRequest>,
) -> Result<TorchResponse> {
    app.torch()
        .toggle(payload.unwrap_or(TorchRequest { value: None }))
}

#[command]
pub(crate) async fn check<R: Runtime>(app: AppHandle<R>) -> Result<TorchResponse> {
    #[allow(deprecated)]
    app.torch().check()
}

#[command]
pub(crate) async fn is_available<R: Runtime>(app: AppHandle<R>) -> Result<AvailableResponse> {
    app.torch()
        .is_available()
        .map(|available| AvailableResponse { available })
}

#[command]
pub(crate) async fn is_enabled<R: Runtime>(app: AppHandle<R>) -> Result<EnabledResponse> {
    app.torch()
        .is_enabled()
        .map(|enabled| EnabledResponse { enabled })
}
