// Extract Sentry distributed tracing headers
// Reference: https://docs.sentry.io/platforms/javascript/tracing/trace-propagation/custom-instrumentation/#inject-tracing-information-to-outgoing-requests
function extractHeaders() {
    const activeSpan = globalThis.Sentry.getActiveSpan();
    const rootSpan = activeSpan ? globalThis.Sentry.getRootSpan(activeSpan) : undefined;

    // Create sentry-trace and baggage header
    const sentryTraceHeader = rootSpan ? globalThis.Sentry.spanToTraceHeader(rootSpan) : undefined;
    const sentryBaggageHeader = rootSpan ? globalThis.Sentry.spanToBaggageHeader(rootSpan) : undefined;

    return {
        "sentry-trace": sentryTraceHeader,
        baggage: sentryBaggageHeader,
    };
}

globalThis.__TAURI_INTERNALS__.invoke = (cmd, args, options) => {
    const tracingHeaders = extractHeaders();

    // Merge headers
    if (!options) options = { headers: {} };
    options.headers = { ...options.headers, ...tracingHeaders }

    globalThis.__TAURI_INTERNALS__.invoke(cmd, args, options);
}
