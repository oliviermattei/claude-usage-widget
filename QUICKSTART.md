# Quick Start Guide

## Prerequisites

- [Rust](https://rustup.rs/) (stable toolchain)
- [Node.js 18+](https://nodejs.org)
- Platform-specific dependencies: see [Tauri v2 prerequisites](https://v2.tauri.app/start/prerequisites/)

## Installation & Development

### 1. Install Dependencies

```bash
cd claude-usage-widget
npm install
```

### 2. Run in Development Mode

```bash
cargo tauri dev
```

This will:
- Compile the Rust backend
- Launch the widget with DevTools available
- Hot-reload the frontend on file changes

### 3. Test the Application

**First Run:**
1. Widget appears (frameless window)
2. Click "Manual" to paste your session key
3. Usage data displays

**Features to Test:**
- [ ] Drag widget around screen
- [ ] Refresh button updates data
- [ ] Minimize to system tray
- [ ] Right-click tray icon shows menu
- [ ] Progress bars animate smoothly
- [ ] Timers count down correctly

### 4. Build for Production

```bash
cargo tauri build
```

Output is in `src-tauri/target/release/bundle/` (or `dist/tauri/` if `.cargo/config.toml` redirects).

## Development Tips

### Enable DevTools
In development mode, right-click the widget and choose "Inspect Element".

### Change Update Frequency
Edit `frontend/app.js`:
```javascript
const UPDATE_INTERVAL = 1 * 60 * 1000; // 1 minute for testing
```

### Mock API Response
For testing UI without API calls, add to `fetchUsageData()`:
```javascript
const mockData = {
  five_hour: { utilization: 45.5, resets_at: "2025-12-13T20:00:00Z" },
  seven_day: { utilization: 78.2, resets_at: "2025-12-17T07:00:00Z" }
};
updateUI(mockData);
return;
```

## File Structure

```
claude-usage-widget/
├── frontend/               # Widget UI (HTML/JS/CSS)
│   ├── index.html
│   ├── app.js              # Frontend logic
│   ├── styles.css
│   ├── tauri-bridge.js     # Maps window.electronAPI to Tauri invoke()
│   └── chart.umd.js        # Bundled Chart.js
├── src-tauri/              # Rust backend (Tauri convention)
│   ├── src/                # main.rs, commands.rs, store_helpers.rs
│   ├── Cargo.toml          # Rust dependencies
│   └── tauri.conf.json     # Tauri window, bundle, plugin config
├── assets/                 # App icons and screenshots
└── package.json            # Node dependencies (chart.js)
```

## Common Issues

### White Screen on Launch
Check DevTools console for errors. Usually means:
- Missing file paths
- JavaScript errors in app.js
- CSS not loading

### API Returns 401
Session expired. Paste a new session key.

### Build fails on Linux
Install the required system libraries:
```bash
sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf
```

## Adding Features

### Custom Themes
Edit `frontend/styles.css` — change gradient colors:
```css
.widget-container {
  background: linear-gradient(135deg, #your-color 0%, #another-color 100%);
}
```

## Debugging

### Console Logs
- Rust backend: Check the terminal where you ran `cargo tauri dev`
- Renderer: Check DevTools console (right-click > Inspect Element)

### Storage
Check stored credentials in DevTools console:
```javascript
await window.electronAPI.getCredentials()
```

## Publishing

1. Update version in `package.json`, `src-tauri/Cargo.toml`, and `src-tauri/tauri.conf.json`
2. Run `cargo tauri build`
3. Test the installer from `src-tauri/target/release/bundle/`
4. Create GitHub release
5. Upload the platform binaries

---

Happy coding!
