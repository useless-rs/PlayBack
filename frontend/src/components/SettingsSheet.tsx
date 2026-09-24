import { useState } from "react";
import { Cpu, Keyboard, MonitorPlay, Palette, SlidersHorizontal, Subtitles, Volume2, X } from "lucide-react";

import type { AppConfig } from "../types";
import { IconButton } from "./IconButton";

interface SettingsSheetProps {
  open: boolean;
  config: AppConfig;
  onClose: () => void;
  onChange: (config: AppConfig) => void;
}

type SettingsSection = "playback" | "video" | "subtitles" | "appearance" | "shortcuts";

const sections: Array<{ id: SettingsSection; label: string; icon: typeof SlidersHorizontal }> = [
  { id: "playback", label: "Playback", icon: SlidersHorizontal },
  { id: "video", label: "Video", icon: MonitorPlay },
  { id: "subtitles", label: "Subtitles", icon: Subtitles },
  { id: "appearance", label: "Appearance", icon: Palette },
  { id: "shortcuts", label: "Shortcuts", icon: Keyboard },
];

const sectionCopy: Record<SettingsSection, { title: string; body: string }> = {
  playback: { title: "Make playback feel yours", body: "These defaults stay local to your machine and can be changed at any time." },
  video: { title: "Video that stays out of the way", body: "Choose a safe hardware decoder and let the surface handle the rest." },
  subtitles: { title: "Readable, not distracting", body: "Tune subtitle size and appearance for the way you watch." },
  appearance: { title: "A quieter interface", body: "PlayBack follows your system appearance and keeps motion intentional." },
  shortcuts: { title: "Keyboard first", body: "Keep your hands on the controls and your eyes on the story." },
};

export function SettingsSheet({ open, config, onClose, onChange }: SettingsSheetProps) {
  const [activeSection, setActiveSection] = useState<SettingsSection>("playback");

  if (!open) {
    return null;
  }

  const update = <K extends keyof AppConfig>(key: K, value: AppConfig[K]) => {
    onChange({ ...config, [key]: value });
  };

  return (
    <div className="sheet-backdrop" role="presentation" onMouseDown={onClose}>
      <section className="settings-sheet" role="dialog" aria-modal="true" aria-labelledby="settings-title" onMouseDown={(event) => event.stopPropagation()}>
        <header className="settings-sheet__header">
          <div>
            <span className="panel-eyebrow">PlayBack preferences</span>
            <h2 id="settings-title">Settings</h2>
          </div>
          <IconButton label="Close settings" icon={X} onClick={onClose} />
        </header>
        <div className="settings-sheet__body">
          <nav className="settings-nav" aria-label="Settings sections">
            {sections.map(({ id, label, icon: Icon }) => (
              <button
                type="button"
                className={`settings-nav__item${activeSection === id ? " is-active" : ""}`}
                key={id}
                onClick={() => setActiveSection(id)}
                aria-current={activeSection === id ? "page" : undefined}
              >
                <Icon aria-hidden="true" size={16} />
                <span>{label}</span>
              </button>
            ))}
          </nav>
          <div className="settings-content">
            <div className="settings-content__intro">
              <span className="settings-content__icon"><Cpu aria-hidden="true" size={18} /></span>
              <div>
                <h3>{sectionCopy[activeSection].title}</h3>
                <p>{sectionCopy[activeSection].body}</p>
              </div>
            </div>
            <label className="setting-row">
              <span><strong>Blur surfaces</strong><small>Use a subtle material effect for the sidebar and sheets.</small></span>
              <input type="checkbox" checked={config.blurBackground} onChange={(event) => update("blurBackground", event.target.checked)} />
            </label>
            <label className="setting-row">
              <span><strong>Show playlist by default</strong><small>Keep the queue visible when you open a file.</small></span>
              <input type="checkbox" checked={config.showPlaylist} onChange={(event) => update("showPlaylist", event.target.checked)} />
            </label>
            <label className="setting-row">
              <span><strong>Interface animations</strong><small>Respect reduced motion automatically when disabled by the system.</small></span>
              <input type="checkbox" checked={config.animations} onChange={(event) => update("animations", event.target.checked)} />
            </label>
            <label className="setting-range">
              <span><strong>Subtitle size</strong><small>{config.subtitleSize} pt</small></span>
              <input type="range" min="28" max="72" value={config.subtitleSize} onChange={(event) => update("subtitleSize", Number(event.target.value))} />
            </label>
            <label className="setting-select">
              <span><strong>Hardware decoding</strong><small>Leave on auto-safe unless you need a specific backend.</small></span>
              <select value={config.hardwareDecoding} onChange={(event) => update("hardwareDecoding", event.target.value)}>
                <option value="auto-safe">Auto safe</option>
                <option value="auto">Auto</option>
                <option value="no">Disabled</option>
              </select>
            </label>
            <div className="shortcut-card">
              <div><Keyboard aria-hidden="true" size={17} /><strong>Keyboard first</strong></div>
              <p>Space toggles playback, arrow keys seek and adjust volume, F toggles fullscreen, and ⌘K opens search.</p>
            </div>
            <div className="settings-content__footer"><Volume2 aria-hidden="true" size={15} /> Changes save automatically.</div>
          </div>
        </div>
      </section>
    </div>
  );
}
