# tauri-plugin-nosleep

Tauri v2 plugin to prevent the OS from entering sleep/power save mode.

Supports macOS, Windows, and Linux.

## Install

Add the Rust crate to your Tauri app:

```console
cargo add tauri-plugin-nosleep
```

Add the JS bindings:

```console
npm install tauri-plugin-nosleep-api
```

## Setup

### Rust

Register the plugin in your Tauri app:

```rust
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_nosleep::init())
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
```

### Permissions

Add the plugin permission to your `capabilities` configuration:

```json
{
  "permissions": ["nosleep:default"]
}
```

## Usage

```typescript
import { block, unblock, NoSleepType } from 'tauri-plugin-nosleep-api'

// Prevent system sleep (e.g. during a download)
await block(NoSleepType.PreventUserIdleSystemSleep)

// Prevent display sleep (e.g. during video playback)
await block(NoSleepType.PreventUserIdleDisplaySleep)

// Release when done
await unblock()
```

## Compatibility

| Plugin Version | Tauri Version |
|---------------|---------------|
| 2.x           | 2.x           |
| 0.1.x         | 1.x           |

## License

MIT
