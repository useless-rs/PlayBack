import { open } from "@tauri-apps/plugin-dialog";
import { useCallback, useEffect, useMemo, useState } from "react";

import { PlaylistPanel } from "./components/PlaylistPanel";
import { SettingsSheet } from "./components/SettingsSheet";
import { Sidebar } from "./components/Sidebar";
import { TopBar } from "./components/TopBar";
import { TransportBar } from "./components/TransportBar";
import { VideoStage } from "./components/VideoStage";
import {
  fetchConfig,
  fetchSnapshot,
  isTauriRuntime,
  openMedia,
  saveConfig,
  seekAbsolute,
  seekRelative,
  setVolume,
  subscribeToPlayback,
  toggleFullscreen,
  togglePlayback,
} from "./lib/ipc";
import {
  DEFAULT_CONFIG,
  DEMO_PLAYLIST,
  type AppConfig,
  type LibraryView,
  type MediaItem,
  type PlaybackSnapshot,
  type PlaybackStateEvent,
} from "./types";

const INITIAL_SNAPSHOT: PlaybackSnapshot = {
  paused: true,
  position: 0,
  duration: null,
  volume: 100,
  speed: 1,
  loopFile: false,
  subtitleVisible: true,
  fullscreen: false,
  mediaTitle: "Nothing playing",
  mediaPath: null,
  backend: "browser-preview",
};

function titleFromPath(path: string): string {
  const parts = path.split(/[\\/]/);
  return parts.at(-1) || path;
}

function mediaFromPath(path: string, index: number): MediaItem {
  const title = titleFromPath(path);
  return {
    id: `${path}-${index}`,
    path,
    title,
    subtitle: "Added from your library",
    kind: "video",
    duration: "--:--",
    addedAt: "Just now",
  };
}

export default function App() {
  const [config, setConfig] = useState<AppConfig>(DEFAULT_CONFIG);
  const [snapshot, setSnapshot] = useState<PlaybackSnapshot>(INITIAL_SNAPSHOT);
  const [playlist, setPlaylist] = useState<MediaItem[]>(DEMO_PLAYLIST);
  const [activeId, setActiveId] = useState<string | null>(null);
  const [activeView, setActiveView] = useState<LibraryView>("library");
  const [searchQuery, setSearchQuery] = useState("");
  const [notice, setNotice] = useState<string | null>(null);
  const [settingsOpen, setSettingsOpen] = useState(false);
  const [sidebarCollapsed, setSidebarCollapsed] = useState(false);

  const applyStateEvent = useCallback((event: PlaybackStateEvent) => {
    setSnapshot(event.snapshot);
    if (event.playlist.length > 0) {
      setPlaylist(event.playlist);
    }
  }, []);

  useEffect(() => {
    let disposed = false;
    void fetchSnapshot().then((value) => {
      if (!disposed && value) {
        setSnapshot(value);
      }
    });
    void fetchConfig().then((value) => {
      if (!disposed && value) {
        setConfig(value);
      }
    });
    void subscribeToPlayback((event) => applyStateEvent(event)).then((unlisten) => {
      if (disposed) {
        unlisten();
      }
    });
    return () => {
      disposed = true;
    };
  }, [applyStateEvent]);

  useEffect(() => {
    if (!notice) {
      return;
    }
    const timeout = window.setTimeout(() => setNotice(null), 6000);
    return () => window.clearTimeout(timeout);
  }, [notice]);

  const updateSnapshot = useCallback((update: Partial<PlaybackSnapshot>) => {
    setSnapshot((current) => ({ ...current, ...update }));
  }, []);

  const openFiles = useCallback(async () => {
    if (!isTauriRuntime()) {
      setNotice("File picking is available in the desktop app. Use the Tauri build to open local media.");
      return;
    }
    setNotice(null);
    try {
      const selection = await open({
        multiple: true,
        directory: false,
        filters: [{ name: "Media", extensions: ["mp4", "mkv", "mov", "webm", "mp3", "flac", "wav"] }],
      });
      if (!selection) {
        return;
      }
      const paths = Array.isArray(selection) ? selection : [selection];
      if (paths.length === 0) {
        return;
      }
      const nextItems = paths.map(mediaFromPath);
      setPlaylist((current) => [...current, ...nextItems]);
      setActiveId(nextItems[0]?.id ?? null);
      updateSnapshot({ mediaPath: paths[0] ?? null, mediaTitle: nextItems[0]?.title ?? "Nothing playing", paused: false, backend: "mpv" });
      const result = await openMedia(paths);
      if (result) {
        applyStateEvent(result);
      }
    } catch (error) {
      console.warn("PlayBack could not open the media picker", error);
      setNotice("PlayBack could not open the file picker. Try again from Open media.");
    }
  }, [applyStateEvent, updateSnapshot]);

  const runCommand = useCallback(async (request: Promise<PlaybackStateEvent | null>) => {
    const result = await request;
    if (result) {
      applyStateEvent(result);
    }
  }, [applyStateEvent]);

  const handleTogglePlayback = useCallback(() => {
    updateSnapshot({ paused: !snapshot.paused });
    void runCommand(togglePlayback());
  }, [runCommand, snapshot.paused, updateSnapshot]);

  const handleSeekRelative = useCallback((seconds: number) => {
    const max = snapshot.duration ?? Number.POSITIVE_INFINITY;
    updateSnapshot({ position: Math.max(0, Math.min(max, snapshot.position + seconds)) });
    void runCommand(seekRelative(seconds));
  }, [runCommand, snapshot.duration, snapshot.position, updateSnapshot]);

  const handleSeekAbsolute = useCallback((seconds: number) => {
    const max = snapshot.duration ?? Number.POSITIVE_INFINITY;
    updateSnapshot({ position: Math.max(0, Math.min(max, seconds)) });
    void runCommand(seekAbsolute(seconds));
  }, [runCommand, snapshot.duration, updateSnapshot]);

  const handleVolume = useCallback((volume: number) => {
    updateSnapshot({ volume: Math.max(0, Math.min(100, volume)) });
    void runCommand(setVolume(volume));
  }, [runCommand, updateSnapshot]);

  const handleFullscreen = useCallback(() => {
    updateSnapshot({ fullscreen: !snapshot.fullscreen });
    void runCommand(toggleFullscreen());
  }, [runCommand, snapshot.fullscreen, updateSnapshot]);

  const handleSelect = useCallback((item: MediaItem) => {
    setActiveId(item.id);
    updateSnapshot({ mediaPath: item.path || null, mediaTitle: item.title, paused: false, position: 0, backend: item.path ? "mpv" : "browser-preview" });
    if (item.path) {
      void runCommand(openMedia([item.path]));
    }
  }, [runCommand, updateSnapshot]);

  const handleNext = useCallback(() => {
    const index = playlist.findIndex((item) => item.id === activeId);
    const next = playlist[(index + 1) % playlist.length];
    if (next) {
      handleSelect(next);
    }
  }, [activeId, handleSelect, playlist]);

  const handlePrevious = useCallback(() => {
    const index = playlist.findIndex((item) => item.id === activeId);
    const previous = playlist[(index - 1 + playlist.length) % playlist.length];
    if (previous) {
      handleSelect(previous);
    }
  }, [activeId, handleSelect, playlist]);

  useEffect(() => {
    const onKeyDown = (event: KeyboardEvent) => {
      const target = event.target as HTMLElement | null;
      if (target?.tagName === "INPUT" || target?.tagName === "TEXTAREA") {
        return;
      }
      if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "k") {
        event.preventDefault();
        document.getElementById("library-search")?.focus();
      } else if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === "o") {
        event.preventDefault();
        void openFiles();
      } else if (event.key === " ") {
        event.preventDefault();
        handleTogglePlayback();
      } else if (event.key === "ArrowLeft") {
        event.preventDefault();
        handleSeekRelative(-10);
      } else if (event.key === "ArrowRight") {
        event.preventDefault();
        handleSeekRelative(10);
      } else if (event.key === "ArrowUp") {
        event.preventDefault();
        handleVolume(snapshot.volume + 5);
      } else if (event.key === "ArrowDown") {
        event.preventDefault();
        handleVolume(snapshot.volume - 5);
      } else if (event.key.toLowerCase() === "f") {
        event.preventDefault();
        handleFullscreen();
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [handleFullscreen, handleSeekRelative, handleTogglePlayback, handleVolume, openFiles, snapshot.volume]);

  const activeMedia = useMemo(() => playlist.find((item) => item.id === activeId) ?? null, [activeId, playlist]);
  const filteredPlaylist = useMemo(() => {
    const query = searchQuery.trim().toLocaleLowerCase();
    if (!query) {
      return playlist;
    }
    return playlist.filter((item) => `${item.title} ${item.subtitle}`.toLocaleLowerCase().includes(query));
  }, [playlist, searchQuery]);
  const title = snapshot.mediaTitle || activeMedia?.title || "Nothing playing";

  const updateConfig = useCallback((next: AppConfig) => {
    setConfig(next);
    void saveConfig(next);
  }, []);

  const handleViewChange = useCallback((view: LibraryView) => {
    setActiveView(view);
    if (view === "playlist" && !config.showPlaylist) {
      updateConfig({ ...config, showPlaylist: true });
    }
  }, [config, updateConfig]);

  const removeItem = useCallback((id: string) => {
    setPlaylist((current) => current.filter((item) => item.id !== id));
    setActiveId((current) => (current === id ? null : current));
  }, []);

  return (
    <div className={`app app--${config.theme} app--view-${activeView}${config.animations ? " app--animated" : ""}${config.blurBackground ? "" : " app--reduced-transparency"}`}>
      <Sidebar
        activeView={activeView}
        collapsed={sidebarCollapsed}
        onViewChange={handleViewChange}
        onOpenFiles={() => void openFiles()}
        onOpenSettings={() => setSettingsOpen(true)}
        onToggleCollapsed={() => setSidebarCollapsed((value) => !value)}
      />
      <main className="workspace">
        <TopBar
          title={title}
           theme={config.theme}
           playlistVisible={config.showPlaylist}
           searchQuery={searchQuery}
           onSearchChange={setSearchQuery}
           onToggleTheme={() => updateConfig({ ...config, theme: config.theme === "dark" ? "light" : "dark" })}
          onTogglePlaylist={() => updateConfig({ ...config, showPlaylist: !config.showPlaylist })}
          onOpenSettings={() => setSettingsOpen(true)}
        />
        <div className="workspace__content">
          <div className="workspace__player">
            <VideoStage media={activeMedia} onOpenFiles={() => void openFiles()} onToggleFullscreen={handleFullscreen} />
            <TransportBar
              paused={snapshot.paused}
              position={snapshot.position}
               duration={snapshot.duration}
               volume={snapshot.volume}
               onTogglePlayback={handleTogglePlayback}
               onSeekRelative={handleSeekRelative}
               onSeekAbsolute={handleSeekAbsolute}
               onPreviousTrack={handlePrevious}
               onNextTrack={handleNext}
               onSetVolume={handleVolume}
               onToggleFullscreen={handleFullscreen}
            />
          </div>
          {config.showPlaylist ? (
            <PlaylistPanel
               items={filteredPlaylist}
              activeId={activeId}
              onSelect={handleSelect}
              onRemove={removeItem}
              onOpenFiles={() => void openFiles()}
            />
          ) : null}
        </div>
      </main>
      <SettingsSheet open={settingsOpen} config={config} onClose={() => setSettingsOpen(false)} onChange={updateConfig} />
      {notice ? (
        <div className="app-notice" role="status" aria-live="polite">
          <span>{notice}</span>
          <button type="button" className="app-notice__dismiss" onClick={() => setNotice(null)}>
            Dismiss
          </button>
        </div>
      ) : null}
    </div>
  );
}
