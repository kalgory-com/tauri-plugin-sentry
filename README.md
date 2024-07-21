# Sentry plugin for Tauri v2

Integrate Sentry to your Tauri v2 application, making JavaScript and Rust work seamlessly

> [!WARNING]
> This project is under active development, so the API isn't stable before production release.
>
> Any contributions are welcomed! We love open-source communities 😍

## Features

- [ ] Sentry integration for Tauri v2
- [ ] Enrich events with automatic breadcrumbs, context, and tags
- [x] Distributed tracing (Trace propagate from JavaScript to Rust)

## Getting started

Tauri v2 provides 2 ways to install plugin.

### Automatic setup

Use your project manager and Tauri CLI to add the dependency:

```bash
npm run tauri add dialog  # or
yarn run tauri add dialog # or
pnpm tauri add dialog     # or
bun tauri add dialog      # or
cargo tauri add dialog
```

### Manual setup

1. Run cargo add `tauri-plugin-sentry` to add the plugin to the project’s dependencies in `Cargo.toml`.
2. Modify `lib.rs` to initialize the plugin:
    ```rust
    pub fn run() {
        tauri::Builder::default()
            // Initialize the plugin
            .plugin(tauri_plugin_sntry::init())
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
    ```
3. If you’d like create dialogs in JavaScript, install the npm package as well:
    ```bash
    npm install tauri-plugin-sentry-api # or
    yarn add tauri-plugin-sentry-api    # or
    pnpm add tauri-plugin-sentry-api    # or
    bun add tauri-plugin-sentry-api
    ```

### Distributed Tracing

On the frontend side, you should use the `invoke` function exported by `tauri-plugin-sentry-api` instead of the default one provided by Tauri.

The arguments and return types are the same as the original `invoke` function.

```typescript
import { invoke } from "tauri-plugin-sentry-api";

async function greet() {
    return await invoke("greet", { name: "kalgory" });
}
```

On the Rust side, you should enable the `tracing` feature to support distributed tracing. Then include `TransactionContextArg` as command argument:

```Rust
use sentry::TransactionContext;

#[tauri::command]
fn greet(name: &str, arg: TransactionContextArg) {
    let context_option: Option<TransactionContext> = arg.into();
    match context_option {
       None => println!("No distributed tracing information"),
       Some(context) =>
          println!(
             "Distributed tracing with name: \"{}\" and trace id: \"{}\"",
             context.name(),
             context.trace_id()
          ),
    }
}
```

To see how to setup and run a Tauri v2 application with Sentry integration, check the [examples](./examples) folder.

> [!NOTE]
> You should start a span manually or enable automatic instrumentation before the `invoke` function.
> Otherwise, the Rust command won't have the tracing information.