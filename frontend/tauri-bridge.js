// tauri-bridge.js
// Provides the window.electronAPI interface using Tauri APIs.
// app.js uses window.electronAPI as its abstraction layer for all backend calls.

(function () {
    const { invoke } = window.__TAURI__.core;
    const { listen, emit } = window.__TAURI__.event;
    const { open } = window.__TAURI__.shell;
    const { getCurrentWebviewWindow } = window.__TAURI__.webviewWindow;

    const appWindow = getCurrentWebviewWindow();

    window.electronAPI = {
        // Credentials management
        getCredentials: () => invoke('get_credentials'),
        saveCredentials: (creds) => invoke('save_credentials', { payload: creds }),
        deleteCredentials: () => invoke('delete_credentials'),
        validateSessionKey: (sessionKey) => invoke('validate_session_key', { sessionKey }),

        // detect-session-key: In Tauri we can't easily open a login BrowserWindow
        // that captures cookies. Instead we prompt manual session key entry.
        detectSessionKey: () => {
            return Promise.resolve({
                success: false,
                error: 'Auto-login is not supported in this version. Please use Manual login.'
            });
        },

        // Window controls
        minimizeWindow: () => invoke('minimize_window'),
        closeWindow: () => invoke('close_window'),
        resizeWindow: (height) => invoke('resize_window', { height }),

        // Window position
        getWindowPosition: () => invoke('get_window_position'),
        setWindowPosition: (pos) => invoke('set_window_position', { x: pos.x, y: pos.y }),

        // Event listeners (Tauri events from backend)
        onRefreshUsage: (callback) => {
            listen('refresh-usage', () => callback());
        },
        onSessionExpired: (callback) => {
            listen('session-expired', () => callback());
        },

        // API
        fetchUsageData: () => invoke('fetch_usage_data'),
        getUsageHistory: () => invoke('get_usage_history'),
        openExternal: (url) => open(url),

        // Platform
        platform: navigator.platform.includes('Win') ? 'win32'
            : navigator.platform.includes('Mac') ? 'darwin'
            : 'linux',

        // Settings
        getSettings: () => invoke('get_settings'),
        saveSettings: (settings) => invoke('save_settings', { settings }),

        // Updates
        checkForUpdate: () => invoke('check_for_update'),
        getAppVersion: () => invoke('get_app_version'),

        // Notifications
        showNotification: (title, body) => invoke('show_notification', { title, body }),

        // Compact mode
        setCompactMode: (compact) => invoke('set_compact_mode', { compact }),
    };

    // Window drag support - Tauri uses data-tauri-drag-region attribute
    // We'll set it up after DOM loads
    document.addEventListener('DOMContentLoaded', () => {
        const titleBar = document.getElementById('titleBar');
        if (titleBar) {
            titleBar.setAttribute('data-tauri-drag-region', '');
        }
        const settingsHeader = document.querySelector('.settings-header');
        if (settingsHeader) {
            settingsHeader.setAttribute('data-tauri-drag-region', '');
        }

        // Save window position on move (debounced)
        let posTimer = null;
        // Tauri v2 doesn't have a direct window-move event from JS,
        // but we can periodically save position when the window is focused
        setInterval(async () => {
            try {
                const pos = await window.electronAPI.getWindowPosition();
                if (pos && pos.x !== undefined) {
                    // Only save if position changed
                    if (window._lastSavedPos &&
                        window._lastSavedPos.x === pos.x &&
                        window._lastSavedPos.y === pos.y) return;
                    window._lastSavedPos = pos;
                    await invoke('set_window_position', { x: pos.x, y: pos.y });
                }
            } catch (e) { /* ignore */ }
        }, 5000);
    });
})();
