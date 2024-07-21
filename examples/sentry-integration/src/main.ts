import { invoke } from "tauri-plugin-sentry-api";
import * as Sentry from "@sentry/browser";

Sentry.init({
  dsn: "https://dd5c9da07c932390436504bfdf28b72e@o4507582139793408.ingest.de.sentry.io/4507599497003088",
  debug: true,
  environment: "dev",
  release: "0.1.0",
  integrations: [Sentry.browserTracingIntegration()],
})

let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  await Sentry.startSpan({ name: "ipc.invoke.greet" }, async () => {
    if (greetMsgEl && greetInputEl) {
      // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
      greetMsgEl.textContent = await invoke("greet", {
        name: greetInputEl.value,
      });
    }
  })
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
});
