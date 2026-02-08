use nosleep::NoSleep;
use std::sync::Mutex;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod commands;
mod error;

pub use error::{Error, Result};

pub(crate) struct NoSleepState {
    pub(crate) no_sleep: Mutex<NoSleep>,
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("nosleep")
        .invoke_handler(tauri::generate_handler![commands::block, commands::unblock])
        .setup(|app, _api| {
            app.manage(NoSleepState {
                no_sleep: Mutex::new(NoSleep::new()?),
            });
            Ok(())
        })
        .build()
}
