use serde_json::Value;
use tauri::AppHandle;
use tauri_plugin_store::StoreExt;

const STORE_FILE: &str = "settings.json";

pub fn get_store_value(app: &AppHandle, key: &str) -> Option<Value> {
    let store = app.store(STORE_FILE).ok()?;
    store.get(key)
}

pub fn set_store_value(app: &AppHandle, key: &str, value: Value) {
    if let Ok(store) = app.store(STORE_FILE) {
        store.set(key, value);
    }
}

pub fn delete_store_value(app: &AppHandle, key: &str) {
    if let Ok(store) = app.store(STORE_FILE) {
        store.delete(key);
    }
}

pub fn get_setting_bool(app: &AppHandle, key: &str, default: bool) -> bool {
    get_store_value(app, key)
        .and_then(|v| v.as_bool())
        .unwrap_or(default)
}

pub fn get_setting_string(app: &AppHandle, key: &str, default: &str) -> String {
    get_store_value(app, key)
        .and_then(|v| v.as_str().map(String::from))
        .unwrap_or_else(|| default.to_string())
}

pub fn get_setting_i64(app: &AppHandle, key: &str, default: i64) -> i64 {
    get_store_value(app, key)
        .and_then(|v| v.as_i64())
        .unwrap_or(default)
}

pub fn get_window_position(app: &AppHandle) -> Option<(i32, i32)> {
    let val = get_store_value(app, "windowPosition")?;
    let x = val.get("x")?.as_i64()? as i32;
    let y = val.get("y")?.as_i64()? as i32;
    Some((x, y))
}

pub fn set_window_position(app: &AppHandle, x: i32, y: i32) {
    set_store_value(
        app,
        "windowPosition",
        serde_json::json!({ "x": x, "y": y }),
    );
}

pub fn delete_credentials(app: &AppHandle) -> Result<(), String> {
    delete_store_value(app, "sessionKey");
    delete_store_value(app, "organizationId");
    Ok(())
}
