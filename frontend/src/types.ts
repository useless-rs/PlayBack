export type ThemeMode = "dark" | "light";

export type LibraryView = "library" | "playlist";

export interface PlaybackSnapshot {
  paused: boolean;
  position: number;
  duration: number | null;
  volume: number;
  speed: number;
  loopFile: boolean;
  subtitleVisible: boolean;
  fullscreen: boolean;
  mediaTitle: string;
  mediaPath: string | null;
  backend: "mpv" | "browser-preview";
}

export interface MediaItem {
  id: string;
  path: string;
  title: string;
  subtitle: string;
  kind: "video" | "audio" | "folder";
  duration: string;
  addedAt: string;
}

export interface AppConfig {
  theme: ThemeMode;
  animations: boolean;
  blurBackground: boolean;
  showPlaylist: boolean;
  showControls: boolean;
  hardwareDecoding: string;
  scale: string;
  subtitleSize: number;
  subtitleFont: string;
}

export interface PlaybackStateEvent {
  snapshot: PlaybackSnapshot;
  playlist: MediaItem[];
}

export const DEFAULT_CONFIG: AppConfig = {
  theme: "dark",
  animations: true,
  blurBackground: true,
  showPlaylist: true,
  showControls: true,
  hardwareDecoding: "auto-safe",
  scale: "bilinear",
  subtitleSize: 52,
  subtitleFont: "SF Pro Display",
};

export const DEMO_PLAYLIST: MediaItem[] = [
  {
    id: "demo-1",
    path: "",
    title: "The Long Way Home",
    subtitle: "Demo library item",
    kind: "video",
    duration: "01:42:18",
    addedAt: "Today",
  },
  {
    id: "demo-2",
    path: "",
    title: "A Small Film About Space",
    subtitle: "Demo library item",
    kind: "video",
    duration: "00:18:42",
    addedAt: "Today",
  },
  {
    id: "demo-3",
    path: "",
    title: "Field Notes — Episode 04",
    subtitle: "Demo library item",
    kind: "video",
    duration: "00:32:09",
    addedAt: "Yesterday",
  },
];
