use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<Torch<R>> {
  Ok(Torch(app.clone()))
}

/// Access to the torch APIs.
pub struct Torch<R: Runtime>(AppHandle<R>);

impl<R: Runtime> Torch<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
  pub fn toggle(&self, _: TorchRequest) -> crate::Result<TorchResponse> {
    Ok(TorchResponse {
      value: Some(false),
    })
  }
  pub fn check(&self) -> crate::Result<TorchResponse> {
    Ok(TorchResponse { value: Some(false) })
  }
}
