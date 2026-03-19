# Changelog

All notable changes to Claude Usage Widget are documented here.
Newest releases at the top. Format inspired by [Keep a Changelog](https://keepachangelog.com).

---

## [Unreleased] — v2.0.0

### Breaking
- **Migrated from Electron to Tauri 2** — removed all Electron dependencies and backend code.
  The app now uses a Rust backend via Tauri for smaller binaries (~20 MB vs ~200 MB),
  lower memory usage, and native OS integration.
- Removed `main.js`, `preload.js`, and `src/fetch-via-window.js` (Electron-specific)
- Build system changed from `electron-builder` to `cargo tauri build`
- GitHub Actions workflows replaced with a single unified Tauri build workflow

### Notes
- The frontend (`frontend/`) is unchanged — `app.js` uses `window.electronAPI`
  which is provided by `tauri-bridge.js` mapping to Tauri `invoke()` calls.
- Frontend moved from `src/renderer/` to `frontend/` at project root for a cleaner structure.
- Auto-login via BrowserWindow is not available in Tauri; use Manual session key entry.

---

## [1.7.0] — 2026-03-15

### New Features
- **Usage History Graph** (contributed by cwil2072) — toggleable line graph showing up to 7 days
  of session and weekly usage history. Snapshots persisted locally so history survives restarts.
  Session and Weekly lines always shown; Sonnet and Extra Usage appear when those sections are visible.
  Graph button highlights purple when active. Tooltip shows exact timestamp and value on hover.
- **Currency support** — Extra Usage row now displays the correct currency symbol (€, £, $)
  based on the user's account billing currency. Falls back to ISO code for unmapped currencies.

### Improvements
- Refresh icon animation — refresh button now spins during both manual and automatic refreshes
- Always show widget when logged in — shows 0% and "Not started" when no active session exists

### Bug Fixes
- macOS minimize/restore (contributed by cwil2072) — widget now minimizes to Dock and reliably
  restores when Dock icon is clicked
- Alert crash fix — missing Notification import caused uncaught ReferenceError
- Weekly limit always visible — fixed bug where widget showed "No Usage Yet" between session
  windows even when weekly usage data was present

---

## [1.6.0] — 2026-03-10

### New Features
- **Usage Alerts** — native OS desktop notifications when usage crosses configurable thresholds.
  Covers both Current Session and Weekly Limit. Smart startup seeding prevents surprise alerts
  on launch. Alerts reset automatically when a usage window resets. Enabled by default.
- **Compact Mode** — minimal view showing just the two progress bars.
  Collapse via chevron on left edge, expand via chevron on right edge of compact view.
  Also toggleable from Settings panel. Mode persisted across restarts.

---

## [1.5.3] — 2026-03-09

### Improvements
- Improved update check frequency — now checks at startup, at each 5-hour session reset,
  and once every 24 hours as a background fallback.

---

## [1.5.2] — 2026-03-09

### New Features
- **Linux support** — pre-built AppImage packages for x86_64 and arm64.

---

## [1.5.1] — 2026-03-07

### Improvements
- Extra Usage row now shows ON/OFF badge so users can see at a glance whether extra
  usage is enabled.

---

## [1.5.0] — 2026-03-07

### New Features
- **Update notifications** — widget checks for new releases at startup. Dismissible banner
  appears when a newer version is available. Current version shown in Settings panel.
- **Configurable date and time formats** — 12h/24h time, and flexible weekly reset date display.

### Bug Fixes
- Fixed Extra Usage row alignment
- Update banner correctly expands/contracts widget height
- Extra Usage section spacing improved when expanded

---

## [1.4.0] — 2026-03-02

### New Features
- **macOS support** — native build with tray icon, Dock support, auto-start via Login Items.
- **Settings panel** — launch at startup, always on top, hide from taskbar, theme selector,
  configurable warning thresholds.
- **Improved main widget layout** — 5-column grid with labeled headers.

---

## [1.3.0] — 2025-12-19

### New Features
- **Session auto-recovery** — silent reconnect attempt on session expiry.

---

## [1.2.0] — 2025-12-17

### New Features
- Window position remembered and restored on restart
- Clear messaging when usage data is unavailable

---

## [1.0.0] — 2025-12-15

Initial release.

### Features
- Real-time usage monitoring — current session and weekly limit
- Visual progress bars with color-coded warnings
- Live countdown timers
- Auto-refresh every 5 minutes
- Always-on-top floating widget, minimize to system tray
- Draggable window positioning
- Dark theme, secure login via Claude.ai
