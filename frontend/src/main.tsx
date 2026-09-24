import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import App from "./App";
import { AppErrorBoundary } from "./components/AppErrorBoundary";
import "./styles.css";

if (import.meta.env.DEV) {
  void import("react-grab");
  void import("react-scan");
}

const root = document.getElementById("root");

if (!root) {
  throw new Error("PlayBack root element is missing");
}

createRoot(root).render(
  <StrictMode>
    <AppErrorBoundary>
      <App />
    </AppErrorBoundary>
  </StrictMode>,
);
