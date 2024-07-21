use serde_json::value::Value;
use tauri::ipc::InvokeBody;
use tauri::test::{get_ipc_response, mock_context, noop_assets, MockRuntime};
use tauri::webview::InvokeRequest;
use tauri::{App, Builder, Runtime, Webview, WebviewWindow, WebviewWindowBuilder};

pub fn create_app<R: Runtime>(builder: Builder<R>) -> App<R> {
    builder.build(mock_context(noop_assets())).unwrap()
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
