import {invoke as tauriInvoke, InvokeArgs, InvokeOptions} from '@tauri-apps/api/core';
import * as Sentry from '@sentry/browser';

export async function invoke<T>(cmd: string, args?: InvokeArgs, options?: InvokeOptions): Promise<T> {
  const tracingHeaders = extractHeaders();

  if (options === undefined) options = { headers: {} };
  options.headers = { ...options.headers, ...Object.fromEntries(tracingHeaders) }

  return await tauriInvoke<T>(cmd, args, options);
}

// Extract Sentry distributed tracing headers
// Reference: https://docs.sentry.io/platforms/javascript/tracing/trace-propagation/custom-instrumentation/#inject-tracing-information-to-outgoing-requests
function extractHeaders() {
  const activeSpan = Sentry.getActiveSpan();
  const rootSpan = activeSpan ? Sentry.getRootSpan(activeSpan) : undefined;

  // Create sentry-trace and baggage header
  const sentryTraceHeader = rootSpan ? Sentry.spanToTraceHeader(rootSpan) : undefined;
  const sentryBaggageHeader = rootSpan ? Sentry.spanToBaggageHeader(rootSpan) : undefined;

  const headers = new Headers();
  if (typeof sentryTraceHeader === "string") headers.append("sentry-trace", sentryTraceHeader);
  if (typeof sentryBaggageHeader === "string") headers.append("baggage", sentryBaggageHeader);

  return headers;
}

