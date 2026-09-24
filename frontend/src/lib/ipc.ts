import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

import type { AppConfig, PlaybackSnapshot, PlaybackStateEvent } from "../types";

export const isTauriRuntime = (): boolean => "__TAURI_INTERNALS__" in window;

async function safeInvoke<T>(command: string, args?: Record<string, unknown>): Promise<T | null> {
  if (!isTauriRuntime()) {
    return null;
  }

  try {
    return await invoke<T>(command, args);
  } catch (error) {
    console.warn(`PlayBack command failed: ${command}`, error);
    return null;
  }
}

export function fetchSnapshot(): Promise<PlaybackSnapshot | null> {
  return safeInvoke<PlaybackSnapshot>("get_playback_snapshot");
}

export function fetchConfig(): Promise<AppConfig | null> {
  return safeInvoke<AppConfig>("get_app_config");
}

export function openMedia(paths: string[]): Promise<PlaybackStateEvent | null> {
  return safeInvoke<PlaybackStateEvent>("open_media", { paths });
}

export function togglePlayback(): Promise<PlaybackStateEvent | null> {
  return safeInvoke<PlaybackStateEvent>("toggle_playback");
}

export function seekRelative(seconds: number): Promise<PlaybackStateEvent | null> {
  return safeInvoke<PlaybackStateEvent>("seek_relative", { seconds });
}

export function seekAbsolute(seconds: number): Promise<PlaybackStateEvent | null> {
  return safeInvoke<PlaybackStateEvent>("seek_absolute", { seconds });
}

export function setVolume(volume: number): Promise<PlaybackStateEvent | null> {
  return safeInvoke<PlaybackStateEvent>("set_volume", { volume });
}

export function toggleFullscreen(): Promise<PlaybackStateEvent | null> {
  return safeInvoke<PlaybackStateEvent>("toggle_fullscreen");
}

export function nextTrack(): Promise<PlaybackStateEvent | null> {
  return safeInvoke<PlaybackStateEvent>("next_track");
}

export function previousTrack(): Promise<PlaybackStateEvent | null> {
  return safeInvoke<PlaybackStateEvent>("previous_track");
}

export function saveConfig(config: AppConfig): Promise<AppConfig | null> {
  return safeInvoke<AppConfig>("save_app_config", { config });
}

export function subscribeToPlayback(
  onState: (event: PlaybackStateEvent) => void,
): Promise<() => void> {
  if (!isTauriRuntime()) {
    return Promise.resolve(() => undefined);
  }

  return listen<PlaybackStateEvent>("playback://state", (event) => onState(event.payload)).then(
    (unlisten) => unlisten,
  );
}
