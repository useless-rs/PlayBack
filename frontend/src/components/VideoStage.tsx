import { Film, Maximize2, Play, Upload } from "lucide-react";

import type { MediaItem } from "../types";
import { IconButton } from "./IconButton";

interface VideoStageProps {
  media: MediaItem | null;
  onOpenFiles: () => void;
  onToggleFullscreen: () => void;
}

export function VideoStage({ media, onOpenFiles, onToggleFullscreen }: VideoStageProps) {
  return (
    <section className="video-stage" aria-label="Video output">
      <div className="video-stage__topline">
        <span className="status-pill"><span className="status-pill__dot" /> mpv ready</span>
        <IconButton label="Toggle fullscreen" icon={Maximize2} onClick={onToggleFullscreen} />
      </div>
      {media ? (
        <div className="video-stage__content">
          <div className="video-stage__frame">
            <Film aria-hidden="true" className="video-stage__film-icon" size={54} strokeWidth={1.2} />
            <p className="video-stage__kicker">Now playing</p>
            <h2>{media.title}</h2>
            <p className="video-stage__hint">The mpv output surface will appear here when playback starts.</p>
            <button type="button" className="stage-play-button" aria-label="Resume playback">
              <Play aria-hidden="true" size={17} fill="currentColor" />
              <span>Resume</span>
            </button>
          </div>
        </div>
      ) : (
        <div className="video-stage__content video-stage__content--empty">
          <div className="empty-state">
            <div className="empty-state__icon"><Film aria-hidden="true" size={28} strokeWidth={1.4} /></div>
            <p className="empty-state__eyebrow">Your cinema, focused</p>
            <h1>Open something worth watching.</h1>
            <p className="empty-state__copy">Drop a file here or browse your library. PlayBack keeps the controls quiet until you need them.</p>
            <button type="button" className="primary-button" onClick={onOpenFiles}>
              <Upload aria-hidden="true" size={16} />
              <span>Open media</span>
            </button>
            <span className="empty-state__shortcut">Press <kbd>⌘ O</kbd> to browse</span>
          </div>
        </div>
      )}
      <div className="video-stage__footer">
        <span>Space play/pause</span>
        <span>← → seek</span>
        <span>F fullscreen</span>
      </div>
    </section>
  );
}
