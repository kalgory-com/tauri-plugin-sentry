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
use desktop::Sentry;
#[cfg(mobile)]
use mobile::Sentry;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the sentry APIs.
pub trait SentryExt<R: Runtime> {
  fn sentry(&self) -> &Sentry<R>;
}

impl<R: Runtime, T: Manager<R>> crate::SentryExt<R> for T {
  fn sentry(&self) -> &Sentry<R> {
    self.state::<Sentry<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("sentry")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let sentry = mobile::init(app, api)?;
      #[cfg(desktop)]
      let sentry = desktop::init(app, api)?;
      app.manage(sentry);
      Ok(())
    })
    .build()
}
