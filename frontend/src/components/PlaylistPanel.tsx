import { useMemo, useState } from "react";
import { Check, MoreHorizontal, Play, Plus, Search, Trash2 } from "lucide-react";

import type { MediaItem } from "../types";
import { IconButton } from "./IconButton";

interface PlaylistPanelProps {
  items: MediaItem[];
  activeId: string | null;
  onSelect: (item: MediaItem) => void;
  onRemove: (id: string) => void;
  onOpenFiles: () => void;
}

export function PlaylistPanel({ items, activeId, onSelect, onRemove, onOpenFiles }: PlaylistPanelProps) {
  const [filter, setFilter] = useState("");
  const visibleItems = useMemo(() => {
    const query = filter.trim().toLocaleLowerCase();
    if (!query) {
      return items;
    }
    return items.filter((item) => `${item.title} ${item.subtitle}`.toLocaleLowerCase().includes(query));
  }, [filter, items]);

  return (
    <aside className="playlist-panel" aria-label="Playlist">
      <div className="playlist-panel__header">
        <div>
          <span className="panel-eyebrow">Up next</span>
          <h2>Playlist <span>{items.length}</span></h2>
        </div>
        <div className="playlist-panel__header-actions">
          <IconButton label="Add media" icon={Plus} size="sm" onClick={onOpenFiles} />
          <IconButton label="Playlist actions" icon={MoreHorizontal} size="sm" />
        </div>
      </div>
      <label className="playlist-search">
        <Search aria-hidden="true" size={14} />
        <input
          aria-label="Filter playlist"
          placeholder="Filter playlist"
          type="search"
          value={filter}
          onChange={(event) => setFilter(event.target.value)}
        />
      </label>
      <div className="playlist-panel__list">
        {visibleItems.length > 0 ? visibleItems.map((item, index) => {
          const active = item.id === activeId;
          return (
            <div className={`playlist-row${active ? " is-active" : ""}`} key={item.id}>
              <button type="button" className="playlist-row__main" onClick={() => onSelect(item)}>
                <span className="playlist-row__index">{active ? <Check aria-hidden="true" size={13} /> : String(index + 1).padStart(2, "0")}</span>
                <span className="playlist-row__copy">
                  <strong>{item.title}</strong>
                  <small>{item.subtitle}</small>
                </span>
                <span className="playlist-row__duration">{item.duration}</span>
              </button>
              <div className="playlist-row__actions">
                {active ? <span className="playing-bars" aria-label="Playing"><i /><i /><i /></span> : null}
                <IconButton label={`Play ${item.title}`} icon={Play} size="sm" onClick={() => onSelect(item)} />
                <IconButton label={`Remove ${item.title}`} icon={Trash2} size="sm" onClick={() => onRemove(item.id)} />
              </div>
            </div>
          );
        }) : (
          <div className="playlist-panel__empty">
            <Search aria-hidden="true" size={18} />
            <strong>No matches</strong>
            <span>Try a different title or clear the filter.</span>
          </div>
        )}
      </div>
      <button type="button" className="playlist-panel__add" onClick={onOpenFiles}>
        <Plus aria-hidden="true" size={15} />
        <span>Add media</span>
      </button>
    </aside>
  );
}
