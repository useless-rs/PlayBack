import {
  Film,
  FolderOpen,
  ListVideo,
  PanelLeftClose,
  PanelLeftOpen,
  Settings2,
} from "lucide-react";

import type { LibraryView } from "../types";
import { IconButton } from "./IconButton";

interface SidebarProps {
  activeView: LibraryView;
  collapsed: boolean;
  onViewChange: (view: LibraryView) => void;
  onOpenFiles: () => void;
  onOpenSettings: () => void;
  onToggleCollapsed: () => void;
}

export function Sidebar({
  activeView,
  collapsed,
  onViewChange,
  onOpenFiles,
  onOpenSettings,
  onToggleCollapsed,
}: SidebarProps) {
  return (
    <aside className={`sidebar${collapsed ? " sidebar--collapsed" : ""}`} aria-label="Primary navigation">
      <div className="sidebar__topline">
        <div className="brand-mark" aria-label="PlayBack home">
          <span className="brand-mark__glyph">
            <img src="/playback-mark.svg" alt="" />
          </span>
          {!collapsed ? <span className="brand-mark__name">PlayBack</span> : null}
        </div>
        <IconButton
          label={collapsed ? "Expand sidebar" : "Collapse sidebar"}
          icon={collapsed ? PanelLeftOpen : PanelLeftClose}
          size="sm"
          onClick={onToggleCollapsed}
        />
      </div>

      <div className="sidebar__actions">
        <button
          type="button"
          className={`sidebar__action${activeView === "library" ? " is-active" : ""}`}
          onClick={() => onViewChange("library")}
          aria-current={activeView === "library" ? "page" : undefined}
          title="Library"
        >
          <Film aria-hidden="true" size={18} strokeWidth={1.8} />
          {!collapsed ? <span>Library</span> : null}
        </button>
        <button
          type="button"
          className={`sidebar__action${activeView === "playlist" ? " is-active" : ""}`}
          onClick={() => onViewChange("playlist")}
          aria-current={activeView === "playlist" ? "page" : undefined}
          title="Playlist"
        >
          <ListVideo aria-hidden="true" size={18} strokeWidth={1.8} />
          {!collapsed ? <span>Playlist</span> : null}
          {!collapsed ? <span className="sidebar__count">3</span> : null}
        </button>
        <button type="button" className="sidebar__action" onClick={onOpenFiles} title="Open media">
          <FolderOpen aria-hidden="true" size={18} strokeWidth={1.8} />
          {!collapsed ? <span>Open media</span> : null}
        </button>
      </div>

      <div className="sidebar__footer">
        <button type="button" className="sidebar__action" onClick={onOpenSettings} title="Settings">
          <Settings2 aria-hidden="true" size={18} strokeWidth={1.8} />
          {!collapsed ? <span>Settings</span> : null}
        </button>
      </div>
    </aside>
  );
}
