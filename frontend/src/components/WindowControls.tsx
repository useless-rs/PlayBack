import { Maximize2, Minus, X } from "lucide-react";
import { getCurrentWindow } from "@tauri-apps/api/window";

import { isTauriRuntime } from "../lib/ipc";
import { IconButton } from "./IconButton";

export function WindowControls() {
  if (!isTauriRuntime()) {
    return null;
  }

  const appWindow = getCurrentWindow();

  return (
    <div className="window-controls" aria-label="Window controls">
      <IconButton
        label="Minimize window"
        icon={Minus}
        size="sm"
        onClick={() => void appWindow.minimize()}
      />
      <IconButton
        label="Maximize window"
        icon={Maximize2}
        size="sm"
        onClick={() => void appWindow.toggleMaximize()}
      />
      <IconButton
        label="Close window"
        icon={X}
        size="sm"
        onClick={() => void appWindow.close()}
      />
    </div>
  );
}
