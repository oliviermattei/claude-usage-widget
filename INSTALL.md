# Installation Instructions

## For End Users

### Option 1: Download Installer (Recommended)

**Windows:**
1. Download `Claude-Usage-Widget-*.exe` or `.msi` from [Releases](../../releases)
2. Run the installer
3. Launch from Start Menu

**macOS:**
1. Download `Claude-Usage-Widget-*-arm64.dmg` (Apple Silicon) or `*-x64.dmg` (Intel)
2. Open the DMG and drag to Applications
3. Run `xattr -cr /Applications/Claude\ Usage\ Widget.app` if Gatekeeper blocks it

**Linux:**
1. Download `Claude-Usage-Widget-*.AppImage` or `.deb`
2. AppImage: `chmod +x *.AppImage && ./*.AppImage`
3. Deb: `sudo dpkg -i *.deb`

### Option 2: Build from Source
```bash
# Install Rust: https://rustup.rs
# Install Node.js: https://nodejs.org
# Install platform dependencies: https://v2.tauri.app/start/prerequisites/

cd claude-usage-widget
npm install
cargo tauri build
```

## First Time Setup

1. **Launch the widget** — A frameless window appears
2. **Click "Manual"** — Paste your Claude.ai session key
3. **Widget activates** — Usage data appears automatically
4. **Minimize to tray** — Click the minus icon

## System Requirements

- **OS:** Windows 10+, macOS 10.15+, or Linux (glibc 2.31+)
- **RAM:** ~50 MB
- **Disk:** ~20 MB
- **Internet:** Required for Claude.ai API

## Security Notes

- Your data stays local — credentials stored securely on your machine
- Open source — code is available for review
- No telemetry — no usage data sent anywhere
- Direct API — only communicates with claude.ai

---

For development setup, see [QUICKSTART.md](QUICKSTART.md)
For full documentation, see [README.md](README.md)
