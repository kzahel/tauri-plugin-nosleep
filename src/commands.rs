use nosleep::NoSleepType;
use tauri::{command, State};

use crate::{NoSleepState, Result};

#[command]
pub(crate) async fn block(
    state: State<'_, NoSleepState>,
    no_sleep_type: NoSleepType,
) -> Result<()> {
    state.no_sleep.lock().unwrap().start(no_sleep_type)?;
    Ok(())
}

#[command]
pub(crate) async fn unblock(state: State<'_, NoSleepState>) -> Result<()> {
    state.no_sleep.lock().unwrap().stop()?;
    Ok(())
}
