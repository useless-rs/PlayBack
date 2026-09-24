# PlayBack Design System

## 1. Atmosphere & Identity

PlayBack is a calm desktop media room: warm paper and ink around a focused, near-black viewing surface. The interface borrows Anthropic's editorial restraint without copying its identity, product copy, logos, or illustrations. Poppins gives controls and headings quiet precision; Lora gives descriptions and long-form interface text a readable, human cadence. The signature is the warm orange playback ring against an ink-dark media stage, carried through the play button, timeline, active navigation, and focus states.

## 2. Color

### Palette

| Role | Token | Light | Dark | Usage |
| --- | --- | --- | --- | --- |
| App canvas | `--surface-canvas` | `#faf9f5` | `#141413` | Main application background |
| Base panel | `--surface-base` | warm mix of `#faf9f5` and `#e8e6dc` | warm mix of `#141413` and `#faf9f5` | Sidebar, top bar, playlist |
| Raised panel | `--surface-raised` | `#faf9f5` | warm charcoal derived from `#141413` | Dialogs and selected controls |
| Media stage | `--surface-stage` | `#141413` | `#0f0f0e` derived | Video-focused region in both themes |
| Primary text | `--text-primary` | `#141413` | `#faf9f5` | Headings and primary labels |
| Secondary text | `--text-secondary` | derived warm gray | `#b0aea5` | Supporting copy and metadata |
| Disabled text | `--text-tertiary` | derived warm gray | derived from `#b0aea5` | Disabled and low-emphasis content |
| Border | `--border-default` | derived from `#e8e6dc` | derived from `#faf9f5` | Dividers and contained surfaces |
| Primary accent | `--accent` | `#a95335` derived for text contrast | `#d97757` | Playback, active navigation, primary action |
| Accent surface | `--accent-soft` | `#d97757` at low opacity | `#d97757` at low opacity | Selected and playback states |
| Focus support | `--focus` | `#3f6f9f` derived | `#6a9bcc` | Focus ring and informational state |
| Success | `--status-success` | `#5f7446` derived | `#788c5d` | Ready and success state |
| Error | `--status-error` | `#9f342f` derived | `#c76258` derived | Destructive feedback |

### Rules

- Orange signals playback and primary action only.
- Blue supports focus and information; it is not decorative.
- Green signals readiness and success; it is not a second primary action color.
- The media stage remains dark in both themes so content does not compete with the video surface.
- Depth comes from tonal shifts and warm one-pixel rings, not glossy cards or cool glass.
- Every derived semantic color must maintain WCAG AA contrast against its intended surface.

## 3. Typography

### Font Stack

- UI and headings: `Poppins`, `Arial`, sans-serif
- Body and editorial copy: `Lora`, `Georgia`, serif
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

- Poppins is used for headings, navigation, buttons, and compact interface labels.
- Lora is used for body copy, media descriptions, and empty-state guidance.
- Monospace is reserved for time-varying or keyboard-specific data.
- Body text never falls below 12px in the desktop shell.

## 4. Spacing & Layout

### Base Unit

All spacing derives from 4px. The primary rhythm uses 4, 8, 12, 16, 24, 32, and 48px.

### Shell

- Sidebar: 248px expanded, 72px collapsed.
- Top bar: 76px desktop, 68px compact.
- Playlist: 304px desktop, 270px medium, hidden below 680px.
- Transport: 112px desktop with a stable timeline and control row.
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

- **Structure:** brand mark, primary navigation, recent items, local storage, settings.
- **Variants:** expanded and collapsed.
- **States:** default, hover, active, focus-visible, collapsed.
- **Layout:** fixed-width flex column; recent list may scroll independently when content grows.

### Top Bar

- **Structure:** current media title, search, appearance, playlist, and settings actions.
- **States:** default, focused input, icon hover, icon active.
- **Layout:** drag region behind controls, single-line desktop toolbar.

### Video Stage

- **Structure:** status, fullscreen action, active or empty media content, shortcut footer.
- **Variants:** active media and empty library.
- **States:** ready, empty, fullscreen control hover/focus.
- **Depth:** near-black stage with one warm ring around the focal content.

### Transport Bar

- **Structure:** timeline, time labels, primary playback controls, secondary tools.
- **States:** playing, paused, muted, subtitle active, playlist active.
- **Accessibility:** timeline is keyboard operable with visible focus and ARIA values.
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

## 6. Motion & Interaction

- Micro feedback: 120-160ms ease-out.
- Panel and sidebar transitions: 180-220ms ease-out.
- Only `transform`, `opacity`, `color`, `background-color`, and `border-color` may transition.
- Pressed controls may scale down slightly; no layout properties animate.
- `prefers-reduced-motion: reduce` disables non-essential transitions and the playing indicator animation.
- No entrance animation or ambient motion is added to the media stage.

## 7. Depth & Surface

The depth strategy is **mixed tonal shift and warm ring elevation**.

- Resting surfaces: tonal shift with a subtle warm border.
- Interactive surfaces: one-pixel warm ring on hover and focus.
- Primary action: orange fill with near-black text and a restrained warm shadow.
- Dialog: raised warm surface plus a soft ambient shadow.
- Media frame: near-black surface with an orange progress ring; no blurred glow or glass.

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
| Central media frame is a placeholder for mpv's native output window. | `VideoStage` | The current Tauri architecture intentionally keeps mpv process control separate from the webview shell. | Replace when the embedded media surface is introduced. |
| Settings dialog does not yet trap focus or close on Escape. | `SettingsSheet` | Pre-existing interaction debt outside this visual rebrand. | Implement with the next settings workflow improvement. |
