use crate::store_helpers;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tauri::{AppHandle, Emitter, Manager, WebviewWindow};

// ── Always-on-top helper ─────────────────────────────────────

pub fn apply_always_on_top(window: &WebviewWindow, enabled: bool) {
    let _ = window.set_always_on_top(enabled);
}

const CHROME_USER_AGENT: &str = "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36";
const HISTORY_RETENTION_DAYS: i64 = 30;
const WIDGET_WIDTH: u32 = 560;
const WIDGET_HEIGHT: u32 = 155;
const GITHUB_OWNER: &str = "SlavomirDurej";
const GITHUB_REPO: &str = "claude-usage-widget";

// ── Credentials ──────────────────────────────────────────────

#[derive(Serialize)]
pub struct Credentials {
    #[serde(rename = "sessionKey")]
    session_key: Option<String>,
    #[serde(rename = "organizationId")]
    organization_id: Option<String>,
}

#[tauri::command]
pub fn get_credentials(app: AppHandle) -> Credentials {
    Credentials {
        session_key: store_helpers::get_store_value(&app, "sessionKey")
            .and_then(|v| v.as_str().map(String::from)),
        organization_id: store_helpers::get_store_value(&app, "organizationId")
            .and_then(|v| v.as_str().map(String::from)),
    }
}

#[derive(Deserialize)]
pub struct SaveCredentialsPayload {
    #[serde(rename = "sessionKey")]
    session_key: String,
    #[serde(rename = "organizationId")]
    organization_id: Option<String>,
}

#[tauri::command]
pub fn save_credentials(app: AppHandle, payload: SaveCredentialsPayload) -> bool {
    store_helpers::set_store_value(&app, "sessionKey", Value::String(payload.session_key));
    if let Some(org_id) = payload.organization_id {
        store_helpers::set_store_value(&app, "organizationId", Value::String(org_id));
    }
    true
}

#[tauri::command]
pub fn delete_credentials(app: AppHandle) -> bool {
    let _ = store_helpers::delete_credentials(&app);
    true
}

// ── Session Validation ───────────────────────────────────────

#[derive(Serialize)]
pub struct ValidationResult {
    success: bool,
    #[serde(rename = "organizationId")]
    organization_id: Option<String>,
    error: Option<String>,
}

#[tauri::command]
pub async fn validate_session_key(
    session_key: String,
) -> Result<ValidationResult, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get("https://claude.ai/api/organizations")
        .header("User-Agent", CHROME_USER_AGENT)
        .header("Cookie", format!("sessionKey={}", session_key))
        .send()
        .await
        .map_err(|e| e.to_string())?;

    if !resp.status().is_success() {
        return Ok(ValidationResult {
            success: false,
            organization_id: None,
            error: Some(format!("HTTP {}", resp.status())),
        });
    }

    let body: Value = resp.json().await.map_err(|e| e.to_string())?;

    if let Some(arr) = body.as_array() {
        if let Some(first) = arr.first() {
            let org_id = first
                .get("uuid")
                .or_else(|| first.get("id"))
                .and_then(|v| v.as_str())
                .map(String::from);
            if org_id.is_some() {
                return Ok(ValidationResult {
                    success: true,
                    organization_id: org_id,
                    error: None,
                });
            }
        }
    }

    Ok(ValidationResult {
        success: false,
        organization_id: None,
        error: Some("No organization found".to_string()),
    })
}

// ── Fetch Usage Data ─────────────────────────────────────────

#[tauri::command]
pub async fn fetch_usage_data(app: AppHandle) -> Result<Value, String> {
    let session_key = store_helpers::get_store_value(&app, "sessionKey")
        .and_then(|v| v.as_str().map(String::from))
        .ok_or("Missing sessionKey")?;

    let organization_id = store_helpers::get_store_value(&app, "organizationId")
        .and_then(|v| v.as_str().map(String::from))
        .ok_or("Missing organizationId")?;

    let client = reqwest::Client::new();
    let cookie = format!("sessionKey={}", session_key);

    let usage_url = format!(
        "https://claude.ai/api/organizations/{}/usage",
        organization_id
    );
    let overage_url = format!(
        "https://claude.ai/api/organizations/{}/overage_spend_limit",
        organization_id
    );
    let prepaid_url = format!(
        "https://claude.ai/api/organizations/{}/prepaid/credits",
        organization_id
    );

    // Fetch all endpoints in parallel
    let (usage_res, overage_res, prepaid_res) = tokio::join!(
        client
            .get(&usage_url)
            .header("User-Agent", CHROME_USER_AGENT)
            .header("Cookie", &cookie)
            .send(),
        client
            .get(&overage_url)
            .header("User-Agent", CHROME_USER_AGENT)
            .header("Cookie", &cookie)
            .send(),
        client
            .get(&prepaid_url)
            .header("User-Agent", CHROME_USER_AGENT)
            .header("Cookie", &cookie)
            .send(),
    );

    // Usage is mandatory
    let usage_resp = usage_res.map_err(|e| e.to_string())?;
    let status = usage_resp.status();
    let body_text = usage_resp.text().await.map_err(|e| e.to_string())?;

    // Check for Cloudflare blocks
    if body_text.contains("Just a moment")
        || body_text.contains("Enable JavaScript and cookies to continue")
    {
        // Clear session
        let _ = store_helpers::delete_credentials(&app);
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.emit("session-expired", ());
        }
        return Err("SessionExpired".to_string());
    }

    if !status.is_success() {
        if status.as_u16() == 401 || status.as_u16() == 403 {
            let _ = store_helpers::delete_credentials(&app);
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.emit("session-expired", ());
            }
            return Err("SessionExpired".to_string());
        }
        return Err(format!("HTTP {}: {}", status, &body_text[..200.min(body_text.len())]));
    }

    let mut data: Value = serde_json::from_str(&body_text)
        .map_err(|e| format!("InvalidJSON: {}", e))?;

    // Merge overage data
    if let Ok(resp) = overage_res {
        if resp.status().is_success() {
            if let Ok(overage) = resp.json::<Value>().await {
                let limit = overage
                    .get("monthly_credit_limit")
                    .or_else(|| overage.get("spend_limit_amount_cents"))
                    .and_then(|v| v.as_f64());
                let used = overage
                    .get("used_credits")
                    .or_else(|| overage.get("balance_cents"))
                    .and_then(|v| v.as_f64());
                let is_enabled = overage.get("is_enabled").and_then(|v| v.as_bool());
                let enabled = is_enabled.unwrap_or(limit.is_some());
                let currency = overage
                    .get("currency")
                    .and_then(|v| v.as_str())
                    .unwrap_or("USD");

                if enabled {
                    if let (Some(limit_val), Some(used_val)) = (limit, used) {
                        if limit_val > 0.0 {
                            data["extra_usage"] = serde_json::json!({
                                "utilization": (used_val / limit_val) * 100.0,
                                "resets_at": null,
                                "used_cents": used_val,
                                "limit_cents": limit_val,
                                "is_enabled": true,
                                "currency": currency,
                            });
                        }
                    }
                } else {
                    if data.get("extra_usage").is_none() {
                        data["extra_usage"] = serde_json::json!({});
                    }
                    data["extra_usage"]["is_enabled"] = Value::Bool(false);
                    data["extra_usage"]["currency"] = Value::String(currency.to_string());
                }
            }
        }
    }

    // Merge prepaid data
    if let Ok(resp) = prepaid_res {
        if resp.status().is_success() {
            if let Ok(prepaid) = resp.json::<Value>().await {
                if let Some(amount) = prepaid.get("amount").and_then(|v| v.as_f64()) {
                    if data.get("extra_usage").is_none() {
                        data["extra_usage"] = serde_json::json!({});
                    }
                    data["extra_usage"]["balance_cents"] = serde_json::json!(amount);
                    if data["extra_usage"].get("currency").is_none() {
                        if let Some(curr) = prepaid.get("currency").and_then(|v| v.as_str()) {
                            data["extra_usage"]["currency"] = Value::String(curr.to_string());
                        }
                    }
                }
            }
        }
    }

    // Store usage history
    store_usage_history(&app, &data);

    Ok(data)
}

fn store_usage_history(app: &AppHandle, data: &Value) {
    let timestamp = chrono::Utc::now().timestamp_millis();

    let entry = serde_json::json!({
        "timestamp": timestamp,
        "session": data.pointer("/five_hour/utilization").and_then(|v| v.as_f64()).unwrap_or(0.0),
        "weekly": data.pointer("/seven_day/utilization").and_then(|v| v.as_f64()).unwrap_or(0.0),
        "sonnet": data.pointer("/seven_day_sonnet/utilization").and_then(|v| v.as_f64()).unwrap_or(0.0),
        "extraUsage": data.pointer("/extra_usage/utilization").and_then(|v| v.as_f64()).unwrap_or(0.0),
    });

    let mut history: Vec<Value> = store_helpers::get_store_value(app, "usageHistory")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    history.push(entry);

    // Prune old entries
    let cutoff = timestamp - (HISTORY_RETENTION_DAYS * 24 * 60 * 60 * 1000);
    history.retain(|e| {
        e.get("timestamp")
            .and_then(|v| v.as_i64())
            .map(|t| t > cutoff)
            .unwrap_or(false)
    });

    store_helpers::set_store_value(app, "usageHistory", Value::Array(history));
}

// ── Usage History ────────────────────────────────────────────

#[tauri::command]
pub fn get_usage_history(app: AppHandle) -> Vec<Value> {
    let chart_days: i64 = 7;
    let cutoff = chrono::Utc::now().timestamp_millis() - (chart_days * 24 * 60 * 60 * 1000);

    let mut history: Vec<Value> = store_helpers::get_store_value(&app, "usageHistory")
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    history.retain(|e| {
        e.get("timestamp")
            .and_then(|v| v.as_i64())
            .map(|t| t > cutoff)
            .unwrap_or(false)
    });

    history.sort_by_key(|e| e.get("timestamp").and_then(|v| v.as_i64()).unwrap_or(0));

    history
}

// ── Settings ─────────────────────────────────────────────────

#[derive(Serialize, Deserialize)]
pub struct Settings {
    #[serde(rename = "autoStart")]
    auto_start: bool,
    #[serde(rename = "minimizeToTray")]
    minimize_to_tray: bool,
    #[serde(rename = "alwaysOnTop")]
    always_on_top: bool,
    theme: String,
    #[serde(rename = "warnThreshold")]
    warn_threshold: i64,
    #[serde(rename = "dangerThreshold")]
    danger_threshold: i64,
    #[serde(rename = "timeFormat")]
    time_format: String,
    #[serde(rename = "weeklyDateFormat")]
    weekly_date_format: String,
    #[serde(rename = "usageAlerts")]
    usage_alerts: bool,
    #[serde(rename = "compactMode")]
    compact_mode: bool,
    #[serde(rename = "refreshInterval")]
    refresh_interval: String,
    #[serde(rename = "graphVisible")]
    graph_visible: bool,
    #[serde(rename = "expandedOpen")]
    expanded_open: bool,
}

#[tauri::command]
pub fn get_settings(app: AppHandle) -> Settings {
    Settings {
        auto_start: store_helpers::get_setting_bool(&app, "settings.autoStart", false),
        minimize_to_tray: store_helpers::get_setting_bool(&app, "settings.minimizeToTray", false),
        always_on_top: store_helpers::get_setting_bool(&app, "settings.alwaysOnTop", true),
        theme: store_helpers::get_setting_string(&app, "settings.theme", "dark"),
        warn_threshold: store_helpers::get_setting_i64(&app, "settings.warnThreshold", 75),
        danger_threshold: store_helpers::get_setting_i64(&app, "settings.dangerThreshold", 90),
        time_format: store_helpers::get_setting_string(&app, "settings.timeFormat", "12h"),
        weekly_date_format: store_helpers::get_setting_string(
            &app,
            "settings.weeklyDateFormat",
            "date",
        ),
        usage_alerts: store_helpers::get_setting_bool(&app, "settings.usageAlerts", true),
        compact_mode: store_helpers::get_setting_bool(&app, "settings.compactMode", false),
        refresh_interval: store_helpers::get_setting_string(
            &app,
            "settings.refreshInterval",
            "300",
        ),
        graph_visible: store_helpers::get_setting_bool(&app, "settings.graphVisible", false),
        expanded_open: store_helpers::get_setting_bool(&app, "settings.expandedOpen", false),
    }
}

#[tauri::command]
pub fn save_settings(app: AppHandle, window: WebviewWindow, settings: Settings) -> bool {
    store_helpers::set_store_value(
        &app,
        "settings.autoStart",
        Value::Bool(settings.auto_start),
    );
    store_helpers::set_store_value(
        &app,
        "settings.minimizeToTray",
        Value::Bool(settings.minimize_to_tray),
    );
    store_helpers::set_store_value(
        &app,
        "settings.alwaysOnTop",
        Value::Bool(settings.always_on_top),
    );
    store_helpers::set_store_value(
        &app,
        "settings.theme",
        Value::String(settings.theme),
    );
    store_helpers::set_store_value(
        &app,
        "settings.warnThreshold",
        serde_json::json!(settings.warn_threshold),
    );
    store_helpers::set_store_value(
        &app,
        "settings.dangerThreshold",
        serde_json::json!(settings.danger_threshold),
    );
    store_helpers::set_store_value(
        &app,
        "settings.timeFormat",
        Value::String(settings.time_format),
    );
    store_helpers::set_store_value(
        &app,
        "settings.weeklyDateFormat",
        Value::String(settings.weekly_date_format),
    );
    store_helpers::set_store_value(
        &app,
        "settings.usageAlerts",
        Value::Bool(settings.usage_alerts),
    );
    store_helpers::set_store_value(
        &app,
        "settings.compactMode",
        Value::Bool(settings.compact_mode),
    );
    store_helpers::set_store_value(
        &app,
        "settings.refreshInterval",
        Value::String(settings.refresh_interval),
    );
    store_helpers::set_store_value(
        &app,
        "settings.graphVisible",
        Value::Bool(settings.graph_visible),
    );
    store_helpers::set_store_value(
        &app,
        "settings.expandedOpen",
        Value::Bool(settings.expanded_open),
    );

    // Apply always-on-top (uses Win32 API on Windows to stay above taskbar)
    apply_always_on_top(&window, settings.always_on_top);

    // Apply skip-taskbar
    let _ = window.set_skip_taskbar(settings.minimize_to_tray);

    true
}

// ── Window Controls ──────────────────────────────────────────

#[tauri::command]
pub fn resize_window(window: WebviewWindow, height: u32) {
    let _ = window.set_size(tauri::LogicalSize::new(
        WIDGET_WIDTH as f64,
        height as f64,
    ));
}

#[tauri::command]
pub fn minimize_window(window: WebviewWindow) {
    let _ = window.hide();
}

#[tauri::command]
pub fn close_window(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
pub fn set_window_position(app: AppHandle, window: WebviewWindow, x: i32, y: i32) -> bool {
    store_helpers::set_window_position(&app, x, y);
    let _ = window.set_position(tauri::PhysicalPosition::new(x, y));
    true
}

#[tauri::command]
pub fn get_window_position(window: WebviewWindow) -> Option<serde_json::Value> {
    window.outer_position().ok().map(|pos| {
        serde_json::json!({
            "x": pos.x,
            "y": pos.y,
            "width": WIDGET_WIDTH,
            "height": WIDGET_HEIGHT,
        })
    })
}

#[tauri::command]
pub fn set_compact_mode(window: WebviewWindow, compact: bool) {
    let width: u32 = if compact { 290 } else { WIDGET_WIDTH };
    let height: u32 = if compact { 105 } else { WIDGET_HEIGHT };
    let _ = window.set_size(tauri::LogicalSize::new(width as f64, height as f64));
}

// ── Notifications ────────────────────────────────────────────

#[tauri::command]
pub fn show_notification(app: AppHandle, title: String, body: String) {
    use tauri_plugin_notification::NotificationExt;
    let _ = app
        .notification()
        .builder()
        .title(&title)
        .body(&body)
        .show();
}

// ── App Version ──────────────────────────────────────────────

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// ── Update Check ─────────────────────────────────────────────

#[derive(Serialize)]
pub struct UpdateResult {
    #[serde(rename = "hasUpdate")]
    has_update: bool,
    version: Option<String>,
}

#[tauri::command]
pub async fn check_for_update() -> UpdateResult {
    let url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        GITHUB_OWNER, GITHUB_REPO
    );

    let client = reqwest::Client::new();
    let result = client
        .get(&url)
        .header("User-Agent", "claude-usage-widget")
        .header("Accept", "application/vnd.github+json")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await;

    match result {
        Ok(resp) => {
            if let Ok(data) = resp.json::<Value>().await {
                if let Some(tag) = data.get("tag_name").and_then(|v| v.as_str()) {
                    let remote = tag.trim_start_matches('v');
                    let current = env!("CARGO_PKG_VERSION");
                    if is_newer_version(remote, current) {
                        return UpdateResult {
                            has_update: true,
                            version: Some(remote.to_string()),
                        };
                    }
                }
            }
            UpdateResult {
                has_update: false,
                version: None,
            }
        }
        Err(_) => UpdateResult {
            has_update: false,
            version: None,
        },
    }
}

fn is_newer_version(remote: &str, local: &str) -> bool {
    let r: Vec<u32> = remote.split('.').filter_map(|s| s.parse().ok()).collect();
    let l: Vec<u32> = local.split('.').filter_map(|s| s.parse().ok()).collect();
    for i in 0..3 {
        let rv = r.get(i).copied().unwrap_or(0);
        let lv = l.get(i).copied().unwrap_or(0);
        if rv > lv {
            return true;
        }
        if rv < lv {
            return false;
        }
    }
    false
}
