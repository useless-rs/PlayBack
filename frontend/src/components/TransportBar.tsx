import {
  Captions,
  Gauge,
  ListVideo,
  Maximize2,
  Pause,
  Play,
  RotateCcw,
  RotateCw,
  SkipBack,
  SkipForward,
  Volume2,
  VolumeX,
} from "lucide-react";

import { IconButton } from "./IconButton";

interface TransportBarProps {
  paused: boolean;
  position: number;
  duration: number | null;
  volume: number;
  speed: number;
  subtitleVisible: boolean;
  playlistVisible: boolean;
  onTogglePlayback: () => void;
  onSeekRelative: (seconds: number) => void;
  onSeekAbsolute: (seconds: number) => void;
  onPreviousTrack: () => void;
  onNextTrack: () => void;
  onSetVolume: (volume: number) => void;
  onToggleSubtitles: () => void;
  onTogglePlaylist: () => void;
  onToggleFullscreen: () => void;
}

function formatTime(seconds: number): string {
  if (!Number.isFinite(seconds) || seconds < 0) {
    return "00:00";
  }
  const total = Math.floor(seconds);
  const hours = Math.floor(total / 3600);
  const minutes = Math.floor((total % 3600) / 60);
  const remainder = total % 60;
  return hours > 0
    ? `${hours.toString().padStart(2, "0")}:${minutes.toString().padStart(2, "0")}:${remainder.toString().padStart(2, "0")}`
    : `${minutes.toString().padStart(2, "0")}:${remainder.toString().padStart(2, "0")}`;
}

export function TransportBar({
  paused,
  position,
  duration,
  volume,
  speed,
  subtitleVisible,
  playlistVisible,
  onTogglePlayback,
  onSeekRelative,
  onSeekAbsolute,
  onPreviousTrack,
  onNextTrack,
  onSetVolume,
  onToggleSubtitles,
  onTogglePlaylist,
  onToggleFullscreen,
}: TransportBarProps) {
  const progress = duration && duration > 0 ? Math.min(100, (position / duration) * 100) : 0;
  const muted = volume <= 0;

  return (
    <section className="transport" aria-label="Playback controls">
      <div className="transport__timeline">
        <div
          className="timeline"
          role="slider"
          tabIndex={0}
          aria-label="Playback position"
          aria-valuemin={0}
          aria-valuemax={duration ?? 0}
          aria-valuenow={position}
          onClick={(event) => {
            if (!duration) {
              return;
            }
            const bounds = event.currentTarget.getBoundingClientRect();
            const ratio = Math.max(0, Math.min(1, (event.clientX - bounds.left) / bounds.width));
            onSeekAbsolute(duration * ratio);
          }}
          onKeyDown={(event) => {
            if (event.key === "ArrowLeft") {
              event.preventDefault();
              onSeekRelative(-10);
            } else if (event.key === "ArrowRight") {
              event.preventDefault();
              onSeekRelative(10);
            }
          }}
        >
          <span className="timeline__fill" style={{ width: `${progress}%` }} />
          <span className="timeline__thumb" style={{ left: `${progress}%` }} />
        </div>
        <div className="transport__time"><span>{formatTime(position)}</span><span>{duration ? formatTime(duration) : "--:--"}</span></div>
      </div>
      <div className="transport__main-row">
        <div className="transport__left-actions">
          <IconButton label="Previous track" icon={SkipBack} size="sm" onClick={onPreviousTrack} />
          <IconButton label="Seek backward 10 seconds" icon={RotateCcw} size="sm" onClick={() => onSeekRelative(-10)} />
          <button type="button" className="transport__play" onClick={onTogglePlayback} aria-label={paused ? "Play" : "Pause"}>
            {paused ? <Play aria-hidden="true" size={18} fill="currentColor" /> : <Pause aria-hidden="true" size={18} fill="currentColor" />}
          </button>
          <IconButton label="Seek forward 10 seconds" icon={RotateCw} size="sm" onClick={() => onSeekRelative(10)} />
          <IconButton label="Next track" icon={SkipForward} size="sm" onClick={onNextTrack} />
        </div>
        <div className="transport__right-actions">
          <button type="button" className="speed-control" aria-label={`Playback speed ${speed.toFixed(2)}x`} title="Playback speed">
            <Gauge aria-hidden="true" size={15} />
            <span>{speed.toFixed(2)}x</span>
          </button>
          <IconButton label={subtitleVisible ? "Hide subtitles" : "Show subtitles"} icon={Captions} active={subtitleVisible} size="sm" onClick={onToggleSubtitles} />
          <IconButton label={playlistVisible ? "Hide playlist" : "Show playlist"} icon={ListVideo} active={playlistVisible} size="sm" onClick={onTogglePlaylist} />
          <div className="volume-control">
            <IconButton label={muted ? "Unmute" : "Mute"} icon={muted ? VolumeX : Volume2} size="sm" onClick={() => onSetVolume(muted ? 100 : 0)} />
            <input
              aria-label="Volume"
              type="range"
              min="0"
              max="100"
              value={volume}
              onChange={(event) => onSetVolume(Number(event.target.value))}
            />
          </div>
          <IconButton label="Toggle fullscreen" icon={Maximize2} size="sm" onClick={onToggleFullscreen} />
        </div>
      </div>
    </section>
  );
}
