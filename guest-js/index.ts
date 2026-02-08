import { invoke } from '@tauri-apps/api/core'

export enum NoSleepType {
  /** Prevents the display from dimming automatically (e.g. playing a video). */
  PreventUserIdleDisplaySleep = 'PreventUserIdleDisplaySleep',

  /** Prevents the system from sleeping due to inactivity (e.g. downloading a file). */
  PreventUserIdleSystemSleep = 'PreventUserIdleSystemSleep',
}

export async function block(noSleepType: NoSleepType): Promise<void> {
  await invoke('plugin:nosleep|block', { noSleepType })
}

export async function unblock(): Promise<void> {
  await invoke('plugin:nosleep|unblock')
}
