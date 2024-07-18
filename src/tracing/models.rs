use sentry::TransactionContext;
use tauri::ipc::{CommandArg, CommandItem, InvokeError};
use tauri::Runtime;

pub struct TransactionContextArg(Option<TransactionContext>);

impl<'de, R: Runtime> CommandArg<'de, R> for TransactionContextArg {
    fn from_command(command: CommandItem<'de, R>) -> Result<Self, InvokeError> {
        let headers = command.message.headers();

        let context = match headers.get("sentry-trace") {
            None => None,
            Some(header) => {
                let context = TransactionContext::continue_from_headers(
                    command.name,
                    "ipc.invoke",
                    [(
                        "sentry-trace",
                        header.to_str().map_err(InvokeError::from_error)?,
                    )],
                );
                Some(context)
            }
        };
        Ok(TransactionContextArg(context))
    }
}

impl From<TransactionContextArg> for Option<TransactionContext> {
    fn from(value: TransactionContextArg) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use sentry::protocol::{SpanId, TraceId};
    use tauri::generate_handler;
    use tauri::http::{HeaderMap, HeaderValue};
    use tauri::ipc::CallbackFn;
    use tauri::test::{mock_builder, INVOKE_KEY};
    use tauri::webview::InvokeRequest;

    use crate::test::{create_app, invoke_ipc_command};

    use super::*;

    #[tauri::command]
    fn test_command(arg: TransactionContextArg) -> String {
        match arg.0 {
            None => String::from("None"),
            Some(context) => format!("{:?}", context),
        }
    }

    #[test]
    fn test_command_arg_implementation() {
        let app = create_app(mock_builder(), generate_handler![test_command]);

        // Create a mock TransactionContext with sentry-trace header
        let trace_id = TraceId::default();
        let span_id = SpanId::default();
        let sentry_trace = format!("{}-{}", trace_id, span_id);
        let context = TransactionContext::continue_from_headers(
            "test_command",
            "ipc.invoke",
            [("sentry-trace", sentry_trace.as_str())],
        );

        // Include trace id in the InvokeRequest header
        let mut headers = HeaderMap::new();
        headers.append(
            "sentry-trace",
            HeaderValue::from_str(sentry_trace.as_str()).unwrap(),
        );

        let res = invoke_ipc_command(
            &app,
            InvokeRequest {
                cmd: context.name().to_string(),
                callback: CallbackFn(0),
                error: CallbackFn(1),
                url: "http://tauri.localhost".parse().unwrap(),
                body: Default::default(),
                headers,
                invoke_key: INVOKE_KEY.to_string(),
            },
        )
        .unwrap();

        let response_context = res.deserialize::<String>().unwrap();
        assert_eq!(response_context, format!("{:?}", context))
    }
}
