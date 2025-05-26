use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::Result;
use crate::TorchExt;

#[command]
pub(crate) async fn toggle<R: Runtime>(
    app: AppHandle<R>,
    payload: TorchRequest,
) -> Result<TorchResponse> {
    app.torch().toggle(payload)
}

#[command]
pub(crate) async fn check<R: Runtime>(app: AppHandle<R>) -> Result<TorchResponse> {
    app.torch().check()
}
