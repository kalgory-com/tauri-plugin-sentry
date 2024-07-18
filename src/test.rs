use serde_json::value::Value;
use tauri::ipc::{Invoke, InvokeBody};
use tauri::test::{get_ipc_response, MockRuntime};
use tauri::webview::InvokeRequest;
use tauri::{
    generate_context, App, Builder, Runtime, Webview, WebviewWindow, WebviewWindowBuilder,
};

pub fn create_app<R: Runtime, F>(builder: Builder<R>, invoke_handler: F) -> App<R>
where
    F: Fn(Invoke<R>) -> bool + Send + Sync + 'static,
{
    builder
        .invoke_handler(invoke_handler)
        .build(generate_context!(
            "examples/tauri-app/src-tauri/tauri.conf.json"
        ))
        .unwrap()
}

pub fn invoke_ipc_command<R: Runtime>(
    app: &App<R>,
    request: InvokeRequest,
) -> Result<InvokeBody, Value>
where
    WebviewWindow<R>: AsRef<Webview<MockRuntime>>,
{
    let webview = WebviewWindowBuilder::new(app, "main", Default::default())
        .build()
        .unwrap();
    get_ipc_response(&webview, request)
}
