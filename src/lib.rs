use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

mod api;

mod commands;
mod error;
mod models;
#[cfg(test)]
#[cfg(feature = "test")]
pub mod test;
#[cfg(feature = "tracing")]
pub mod tracing;

pub use error::{Error, Result};

use api::Sentry;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the sentry APIs.
pub trait SentryExt<R: Runtime> {
    fn sentry(&self) -> &Sentry<R>;
}

impl<R: Runtime, T: Manager<R>> SentryExt<R> for T {
    fn sentry(&self) -> &Sentry<R> {
        self.state::<Sentry<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("sentry")
        .setup(|app, api| {
            let sentry = api::init(app, api)?;
            app.manage(sentry);
            Ok(())
        })
        .build()
}
