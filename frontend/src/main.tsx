import { StrictMode } from "react";
import { createRoot } from "react-dom/client";

import "@fontsource/lora/latin-400.css";
import "@fontsource/lora/latin-500.css";
import "@fontsource/poppins/latin-500.css";
import "@fontsource/poppins/latin-600.css";
import App from "./App";
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
    <App />
  </StrictMode>,
);
