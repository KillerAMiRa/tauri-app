import { createApp } from "vue";
import * as Sentry from "@sentry/vue";
import App from "./App.vue";

const app = createApp(App);

Sentry.init({
  app,
  dsn: "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",
  integrations: [
    Sentry.browserTracingIntegration(),
  ],
  tracesSampleRate: 1.0,
});

app.config.errorHandler = (err, instance, info) => {
  console.error("Vue Error:", err, info);
  Sentry.captureException(err);
};

window.addEventListener("unhandledrejection", (event) => {
  Sentry.captureException(event.reason);
});

window.addEventListener("error", (event) => {
  Sentry.captureException(event.error);
});

app.mount("#app");
