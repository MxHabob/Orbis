import React from "react";
import ReactDOM from "react-dom/client";
import { AppErrorBoundary } from "@/components/app-error-boundary";
import { App } from "./App";
import "@/styles/tailwind.css";
import "@/styles/globals.css";

function normalizeRouterPath() {
  const { pathname, search, hash } = window.location;

  if (pathname.endsWith("/index.html")) {
    const next = pathname.replace(/\/index\.html$/, "") || "/";
    window.history.replaceState(null, "", `${next}${search}${hash}`);
  }
}

normalizeRouterPath();

window.addEventListener("error", (event) => {
  console.error("Orbis window error:", event.error ?? event.message);
});

window.addEventListener("unhandledrejection", (event) => {
  console.error("Orbis unhandled rejection:", event.reason);
});

const root = document.getElementById("root");

if (!root) {
  throw new Error("Root element #root was not found");
}

ReactDOM.createRoot(root).render(
  <React.StrictMode>
    <AppErrorBoundary>
      <App />
    </AppErrorBoundary>
  </React.StrictMode>,
);

document.getElementById("boot-fallback")?.remove();
