# Claude Usage Widget

A beautiful, standalone desktop widget for **Windows, macOS, and Linux** that displays your Claude.ai usage statistics in real-time.

Built with [Tauri 2](https://tauri.app) for a lightweight, native experience.

![Claude Usage Widget - Main](assets/screenshot-main.png)

---

## Features

- **Real-time Usage Tracking** — Monitor both session and weekly usage limits
- **Visual Progress Bars** — Clean, gradient progress indicators with configurable warning thresholds
- **Countdown Timers** — Circular timers showing time elapsed in the current session window
- **Auto-refresh** — Configurable interval (15s to 5min), with animated refresh indicator
- **Usage History Graph** — Toggleable 7-day chart showing session and weekly trends over time
- **Currency Support** — Extra usage displays your account's billing currency
- **Modern UI** — Sleek, draggable widget with dark and light themes
- **Secure** — Credentials stored locally only, no third-party servers
- **Always on Top** — User-controlled, stays visible across all workspaces
- **System Tray** — Minimizes to tray for easy access
- **Settings Panel** — Persistent preferences for startup, theme, tray, thresholds, and date/time formats
- **Usage Alerts** — Desktop notifications when usage crosses configurable warn/danger thresholds
- **Update Notifications** — Automatic check for new releases on startup
- **Configurable Date & Time Formats** — 12h/24h time, and flexible weekly reset date display
- **Compact Mode** — Minimal view for when you just need a quick glance

---

## Screenshots

### Usage History Graph

![Claude Usage Widget - Graph](assets/screenshot-graph.png)

- Displays up to **7 days** of session and weekly usage history
- History **persists across restarts**
- Sonnet and Extra Usage lines appear automatically when relevant
- **Adaptive x-axis labels** — shows times for short spans, dates for longer spans
- Respects your **12h/24h time format** setting

### Settings Panel

![Claude Usage Widget - Settings](assets/screenshot-settings.png)

---

## Installation

### Download Pre-built Release

**Windows:**
1. Download the latest `.exe` (NSIS installer) or `.msi` from [Releases](../../releases)
2. Run the installer
3. Launch "Claude Usage Widget" from the Start Menu

**macOS:**
1. Download the latest `.dmg` (arm64 for Apple Silicon, x64 for Intel) from [Releases](../../releases)
2. Open the DMG and drag the app to your Applications folder
3. Launch "Claude Usage Widget" from Applications

> **macOS Security Notice:** Because this app is not yet notarized with Apple, macOS Gatekeeper may show a warning. To fix this, run:
> ```
> xattr -cr /Applications/Claude\ Usage\ Widget.app
> ```

**Linux:**
1. Download the latest `.AppImage` or `.deb` from [Releases](../../releases)
2. AppImage: `chmod +x Claude-Usage-Widget-*.AppImage && ./Claude-Usage-Widget-*.AppImage`
3. Deb: `sudo dpkg -i Claude-Usage-Widget-*.deb`

---

### Build from Source

**Prerequisites:**
- [Rust](https://rustup.rs/) (stable)
- Node.js 18+ ([Download](https://nodejs.org))
- Platform-specific dependencies (see [Tauri prerequisites](https://v2.tauri.app/start/prerequisites/))

```bash
git clone https://github.com/SlavomirDurej/claude-usage-widget.git
cd claude-usage-widget
npm install
cargo tauri dev
```

To create a production build:

```bash
cargo tauri build
```

---

## Usage

### First Launch

1. Launch the widget
2. Click "Manual" when prompted
3. Paste your Claude.ai session key (from browser cookies)
4. Usage data will start displaying immediately

### Widget Controls

- **Drag** — Click and drag the title bar to move the widget
- **Refresh** — Click the refresh icon to update data immediately
- **Graph** — Click the graph icon to toggle usage history
- **Minimize** — Click the minus icon to hide to system tray / dock
- **Close** — Click the X to exit

### System Tray

Right-click the tray icon for: Show/Hide, Refresh, Log Out, Exit.

---

## Understanding the Display

### Current Session & Weekly Limit

| Column | Description |
|--------|-------------|
| Session Used | Progress bar showing usage from 0-100% |
| Elapsed | Circular timer showing how far through the window you are |
| Resets In | Countdown until the window resets |
| Resets At | Actual local clock time / date when the window resets |

**Color Coding:**
- Purple: Normal usage (below warning threshold, default 75%)
- Orange: High usage (above warning threshold)
- Red: Critical usage (above danger threshold, default 90%)

---

## Privacy & Security

- Credentials stored **locally only** using Tauri's secure store
- No data sent to any third-party servers
- Only communicates with the official Claude.ai API
- Logout clears all session data

---

## Troubleshooting

**"Login Required" keeps appearing** — Session may have expired. Paste a fresh session key.

**Widget not updating** — Check internet connection, click refresh manually, or try re-logging in.

**Build errors** — Ensure Rust and platform dependencies are installed:
```bash
rustup update stable
npm install
cargo tauri build
```

If issues persist, open a [Support discussion](../../discussions/categories/support) with your OS, Rust version, and full error output.

---

## Roadmap

- [x] macOS support
- [x] Linux support
- [x] Settings panel
- [x] Remember window position
- [x] Custom warning thresholds
- [x] Configurable date & time formats
- [x] Update notifications
- [x] Usage alerts at thresholds
- [x] Compact mode
- [x] Usage history graph
- [x] Currency support
- [ ] Multiple account support
- [ ] Keyboard shortcuts

---

*Built with Tauri 2 · [Releases](../../releases) · [Discussions](../../discussions)*
