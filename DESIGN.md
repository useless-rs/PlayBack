# PlayBack Design System

## 1. Atmosphere & Identity

PlayBack is a quiet macOS-style media room: a near-black viewing surface framed by translucent system chrome, with content doing the talking. The interface borrows macOS 26's liquid-glass material language without copying Apple product chrome or logos. System sans typography keeps controls compact and familiar; the signature is a single warm playback signal against cool graphite glass, carried through the play button, timeline, and active media state.

## 2. Color

### Palette

| Role | Token | Light | Dark | Usage |
| --- | --- | --- | --- | --- |
| App canvas | `--surface-canvas` | `#f5f5f7` | `#0b0b0d` | Main application background |
| Glass chrome | `--surface-glass` | luminous white with a restrained sheen | deep graphite with a restrained sheen | Sidebar, top bar, playlist, transport |
| Glass raised | `--surface-glass-strong` | bright white with inner highlight | lifted graphite with inner highlight | Active controls and sheets |
| Media stage | `--surface-stage` | `#1d1d1f` | `#09090a` | Video-focused region in both themes |
| Primary text | `--text-primary` | `#1d1d1f` | `#f5f5f7` | Headings and primary labels |
| Secondary text | `--text-secondary` | `#6e6e73` | `#a1a1a6` | Supporting copy and metadata |
| Disabled text | `--text-tertiary` | `#86868b` | `#86868b` | Disabled and low-emphasis content |
| Glass border | `--border-default` | translucent black | translucent white | Dividers and contained surfaces |
| Action blue | `--accent` | `#0071e3` | `#0a84ff` | Primary action, selection, focus |
| Playback signal | `--playback` | `#ff9f0a` | `#ffb340` | Play button, timeline, active media |
| Focus support | `--focus` | `#0071e3` | `#0a84ff` | Focus ring and informational state |
| Success | `--status-success` | `#248a3d` | `#30d158` | Ready and success state |
| Error | `--status-error` | `#d70015` | `#ff453a` | Destructive feedback |

### Rules

- Action blue is reserved for primary actions, selection, and focus.
- Warm amber is reserved for playback state and the playhead; it is not a second navigation accent.
- Green signals readiness and success; it is not a second primary action color.
- The media stage remains dark in both themes so content does not compete with the video surface.
- Glass belongs to navigation, toolbars, sheets, and controls; the content stage stays visually solid.
- Every derived semantic color must maintain WCAG AA contrast against its intended surface.

## 3. Typography

### Font Stack

- UI and headings: `-apple-system`, `BlinkMacSystemFont`, `SF Pro Display`, `Helvetica Neue`, sans-serif
- Body and supporting copy: `-apple-system`, `BlinkMacSystemFont`, `SF Pro Text`, `Helvetica Neue`, sans-serif
- Timecodes and keyboard hints: `ui-monospace`, `SFMono-Regular`, `Consolas`, monospace

### Scale

| Role | Size | Weight | Line height | Usage |
| --- | --- | --- | --- | --- |
| Display | `clamp(2rem, 4vw, 3rem)` | 500 | 1.12 | Empty-state and active-media titles |
| Heading | `1.5rem` | 500 | 1.2 | Dialog and panel titles |
| Subheading | `1.125rem` | 500 | 1.3 | Settings section titles |
| Body | `0.875rem` | 400 | 1.6 | Explanatory copy and metadata |
| UI | `0.75rem` | 500 | 1.35 | Buttons, navigation, compact labels |
| Caption | `0.6875rem` | 500 | 1.4 | Timecodes and secondary indicators |

### Rules

- The system sans stack is used for every interface surface; the player should feel native before it feels branded.
- Weight and color, not extra font families, create hierarchy.
- Monospace is reserved for time-varying or keyboard-specific data.
- Body text never falls below 12px in the desktop shell.

## 4. Spacing & Layout

### Base Unit

All spacing derives from 4px. The primary rhythm uses 4, 8, 12, 16, 24, 32, and 48px.

### Shell

- Sidebar: 220px expanded, 68px collapsed.
- Top bar: 68px desktop, 60px compact.
- Playlist: 320px desktop, 280px medium, hidden below 680px.
- Transport: 96px desktop with a stable timeline and compact control row.
- Content remains left-aligned. The app never centers the full shell.
- The video stage owns the flexible central space; sidebars own their own scrolling.

### Breakpoints

- Compact sidebar: below 860px.
- Single-column settings and hidden playlist: below 680px.
- All primary content must remain usable at 375px width and 200% zoom.

## 5. Components

### Icon Button

- **Structure:** semantic button with one Lucide icon and optional visible label.
- **Variants:** small, medium, large.
- **States:** default, hover, active, focus-visible, disabled.
- **Accessibility:** requires an accessible label; active state exposes `aria-pressed`.
- **Motion:** color and background transition; pressed state scales to 0.96.

### Sidebar Navigation

- **Structure:** brand mark, Library, Playlist, Open media, and Settings.
- **Variants:** expanded and collapsed glass rail.
- **States:** default, hover, active, focus-visible, collapsed.
- **Layout:** fixed-width flex column with no dashboard widgets or storage meters; media remains the focus.

### Top Bar

- **Structure:** current media title, search, appearance, playlist, settings, and desktop window controls.
- **States:** default, focused input, icon hover, icon active.
- **Layout:** drag region behind controls, single-line desktop toolbar with a compact glass control cluster.

### Video Stage

- **Structure:** status, fullscreen action, active or empty media content, shortcut footer.
- **Variants:** active media and empty library.
- **States:** ready, empty, fullscreen control hover/focus.
- **Depth:** near-black stage with one warm ring around the focal content.

### Transport Bar

- **Structure:** timeline, time labels, previous/play/next, volume, and fullscreen.
- **States:** playing, paused, muted, and timeline focus.
- **Accessibility:** timeline is keyboard operable with visible focus and ARIA values.
- **Simplification:** speed, subtitle, and playlist controls belong in Settings or the top bar rather than crowding the transport.
- **Motion:** progress and control feedback only; no decorative motion.

### Playlist Panel

- **Structure:** heading, filter, scrollable rows, add action.
- **States:** empty, populated, row hover, active row, destructive action.
- **Layout:** panel owns vertical scrolling; rows never force the player wider.

### Settings Sheet

- **Structure:** modal header, section navigation, content area, automatic-save footer.
- **Variants:** five sections using the same layout.
- **States:** open, closed, control hover, focus, checked, unchecked.
- **Accessibility:** dialog role, modal semantics, labelled title, full keyboard reachability.

### Brand Mark

- The canonical mark is [`logo.svg`](../logo.svg).
- The compact app mark is [`assets/logos/icon-only/logo-icon.svg`](../assets/logos/icon-only/logo-icon.svg).
- Keep the mark flat, preserve its aspect ratio, and use clear space equal to the icon height.

### Liquid Glass Material

- **Scope:** sidebar, top bar, playlist, transport, settings sheet, menus, and compact control clusters only.
- **Structure:** a translucent semantic tint, 24–32px backdrop blur, 140–160% saturation, one-pixel inner highlight, and a soft ambient shadow.
- **Fallback:** reduced-transparency environments use an opaque semantic surface with the same border and shadow hierarchy.
- **Performance:** blur is restricted to fixed or non-scrolling chrome; playlist and settings content scroll inside an opaque inner surface.
- **Consistency:** one glass recipe is used everywhere. Panels do not each invent a different radius, opacity, or blur.
- **Depth order:** window canvas → glass chrome → solid content surfaces → focused controls → overlays.

## 6. Motion & Interaction

- Micro feedback: 140-180ms with the shared Apple-like ease-out curve.
- Panel and sidebar transitions: 220-280ms with the same curve.
- Only `transform`, `opacity`, `color`, `background-color`, `border-color`, and `filter` may transition.
- Pressed controls may scale down slightly; no layout properties animate.
- `prefers-reduced-motion: reduce` disables non-essential transitions and the playing indicator animation.
- No entrance animation or ambient motion is added to the media stage.

## 7. Depth & Surface

The depth strategy is **macOS liquid glass over solid content**.

- Resting chrome: translucent material with a one-pixel inner highlight and restrained ambient shadow.
- Interactive surfaces: brighter glass tint plus blue focus/selection ring.
- Primary action: system blue fill with white text and a restrained blue ambient shadow.
- Playback signal: amber playhead and play control, never a decorative glow.
- Dialog: raised glass frame surrounding an opaque scrolling content surface.
- Media frame: a deep, blurred stage that keeps desktop texture atmospheric without competing with active video; video content remains opaque when present.

## 8. Accessibility Constraints & Accepted Debt

### Constraints

- Target WCAG 2.2 AA: 4.5:1 body contrast and 3:1 large text and UI boundaries.
- Every interactive element has a visible focus state.
- Light and dark themes preserve equivalent hierarchy and task completion.
- Keyboard playback, seek, volume, fullscreen, and dialog controls remain reachable.
- Reduced-motion and 200% zoom are mandatory acceptance checks.
- Icons never carry meaning alone; labels or accessible names accompany them.

### Accepted Debt

| Item | Location | Why accepted | Owner / Exit |
| --- | --- | --- | --- |
| Native settings dialog behavior is platform-owned. | `SettingsSheet` | The browser-native `<dialog>` owns modal focus and Escape semantics; app code only synchronizes open state. | Re-test focus restoration if the dialog implementation changes. |

## 9. Delivery & Recovery Contract

- The crates.io package `playback-player` is the CLI distribution; it must never be presented as the desktop UI installer.
- The Tauri desktop distribution is the user-facing app. Its release must include the production `frontend/dist` bundle and an installable artifact for the target platform.
- The desktop window is opaque and uses the app canvas token as its background. Transparency and under-page effects are not allowed because they expose terminal or desktop content through the player shell.
- A frameless desktop shell must expose labeled minimize, maximize/restore, and close controls. These controls are hidden in browser preview and never replace the native web content.
- The first-run state is a composed invitation to open media, not an empty canvas. The primary action must be reachable by pointer, keyboard, and the documented Cmd/Ctrl+O shortcut.
- If React rendering or an unexpected UI exception occurs, the app must show a branded recovery surface with a reload action rather than a blank window.
- Playlist navigation must reveal the playlist when it is hidden; navigation state must correspond to a visible layout change.
- Browser preview and Tauri desktop use the same React surface. Tauri-only controls and IPC failures must degrade safely without removing the core shell.

### Recovery Primitive

- **Structure:** icon, concise failure message, explanation, reload button, and short diagnostic hint.
- **State:** rendered only after a caught UI exception; the normal shell remains the default state.
- **Accessibility:** the recovery heading receives focus after mount; the reload action is a semantic button with a visible focus ring.
- **Motion:** opacity and transform only, with reduced-motion support.

### Window Controls Primitive

- **Structure:** three compact icon buttons in the top bar, aligned to the trailing edge.
- **States:** default, hover, focus-visible, pressed, and close hover.
- **Accessibility:** each button has a descriptive label and a native disabled state only when the action is unavailable.
- **Visibility:** Tauri desktop only; browser preview renders no empty control cluster.
