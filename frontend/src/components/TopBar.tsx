import { Command, MoreHorizontal, Moon, PanelRight, Search, Sun } from "lucide-react";

import type { ThemeMode } from "../types";
import { IconButton } from "./IconButton";

interface TopBarProps {
  title: string;
  theme: ThemeMode;
  playlistVisible: boolean;
  onToggleTheme: () => void;
  onTogglePlaylist: () => void;
  onOpenSettings: () => void;
}

export function TopBar({
  title,
  theme,
  playlistVisible,
  onToggleTheme,
  onTogglePlaylist,
  onOpenSettings,
}: TopBarProps) {
  return (
    <header className="topbar">
      <div className="topbar__drag-region" aria-hidden="true" />
      <div className="topbar__title-block">
        <span className="topbar__eyebrow">Now playing</span>
        <strong>{title}</strong>
      </div>
      <div className="topbar__tools">
        <label className="search-field">
          <Search aria-hidden="true" size={15} strokeWidth={1.8} />
          <input aria-label="Search library" placeholder="Search library" type="search" />
          <kbd><Command aria-hidden="true" size={11} /> K</kbd>
        </label>
        <IconButton
          label={theme === "dark" ? "Use light appearance" : "Use dark appearance"}
          icon={theme === "dark" ? Sun : Moon}
          onClick={onToggleTheme}
        />
        <IconButton
          label={playlistVisible ? "Hide playlist" : "Show playlist"}
          icon={PanelRight}
          active={playlistVisible}
          onClick={onTogglePlaylist}
        />
        <IconButton label="More actions" icon={MoreHorizontal} onClick={onOpenSettings} />
      </div>
    </header>
  );
}
