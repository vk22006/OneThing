use chrono::Local;
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use tauri::{AppHandle, Manager, Runtime};
use tauri_plugin_notification::NotificationExt;

/// Shape of a pending notification persisted to disk.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersistedNotification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub scheduled_at: i64, // Unix timestamp in seconds
    pub notification_type: String,
}

/// Scheduler settings persisted by the frontend.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct SchedulerSettings {
    enabled: Option<bool>,
    daily_reminder_time: Option<String>, // "HH:MM" format
}

/// Read the store JSON file directly from disk (no webview needed).
fn read_store_file(app_data_dir: &PathBuf) -> Option<serde_json::Value> {
    let store_path = app_data_dir.join("settings.json");
    match std::fs::read_to_string(&store_path) {
        Ok(contents) => serde_json::from_str(&contents).ok(),
        Err(_) => None,
    }
}

/// Write the store JSON file back to disk.
fn write_store_file(app_data_dir: &PathBuf, value: &serde_json::Value) -> Result<(), String> {
    let store_path = app_data_dir.join("settings.json");
    let contents = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    std::fs::write(&store_path, contents).map_err(|e| e.to_string())
}

/// Get pending notifications from the store.
fn get_pending(store: &serde_json::Value) -> Vec<PersistedNotification> {
    store
        .get("pendingNotifications")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default()
}

/// Save pending notifications back to the store.
fn save_pending(
    app_data_dir: &PathBuf,
    store: &mut serde_json::Value,
    pending: &[PersistedNotification],
) -> Result<(), String> {
    let obj = store.as_object_mut().ok_or("store is not an object")?;
    obj.insert(
        "pendingNotifications".to_string(),
        serde_json::to_value(pending).map_err(|e| e.to_string())?,
    );
    write_store_file(app_data_dir, store)
}

/// Start the background notification scheduler.
/// This spawns a tokio task that runs forever (until the process exits).
pub fn start<R: Runtime>(app: AppHandle<R>) {
    tauri::async_runtime::spawn(async move {
        info!("Background notification scheduler started");

        loop {
            tokio::time::sleep(Duration::from_secs(30)).await;

            let app_data_dir = match app.path().app_data_dir() {
                Ok(dir) => dir,
                Err(e) => {
                    error!("Failed to get app data dir: {}", e);
                    continue;
                }
            };

            let mut store = match read_store_file(&app_data_dir) {
                Some(s) => s,
                None => continue, // No store file yet, nothing to do
            };

            let now = Local::now().timestamp();
            let pending = get_pending(&store);

            if pending.is_empty() {
                // Still check for daily digest even if no individual pending notifications
                check_daily_digest(&app, &app_data_dir, &mut store);
                continue;
            }

            let mut due: Vec<PersistedNotification> = Vec::new();
            let mut remaining: Vec<PersistedNotification> = Vec::new();

            for notif in pending {
                if notif.scheduled_at <= now {
                    due.push(notif);
                } else {
                    remaining.push(notif);
                }
            }

            // Fire due notifications
            for notif in &due {
                info!("Firing notification: {} - {}", notif.title, notif.body);
                if let Err(e) = app
                    .notification()
                    .builder()
                    .title(&notif.title)
                    .body(&notif.body)
                    .show()
                {
                    error!("Failed to send notification: {}", e);
                }
            }

            // Update the store if any were fired
            if !due.is_empty() {
                if let Err(e) = save_pending(&app_data_dir, &mut store, &remaining) {
                    error!("Failed to update pending notifications: {}", e);
                }
            }

            // Check daily digest
            check_daily_digest(&app, &app_data_dir, &mut store);
        }
    });
}

/// Check whether it's time to send the daily deadline digest.
fn check_daily_digest<R: Runtime>(
    app: &AppHandle<R>,
    app_data_dir: &PathBuf,
    store: &mut serde_json::Value,
) {
    let now = Local::now();
    let today = now.format("%Y-%m-%d").to_string();

    // Check if we already sent the digest today
    let last_sent = store
        .get("lastDailyDigestDate")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    if last_sent.as_deref() == Some(today.as_str()) {
        return;
    }

    // Check if notifications are enabled
    let settings: Option<SchedulerSettings> = store
        .get("notificationSettings")
        .and_then(|v| serde_json::from_value(v.clone()).ok());

    let enabled = settings
        .as_ref()
        .and_then(|s| s.enabled)
        .unwrap_or(true);

    if !enabled {
        return;
    }

    // Get the configured daily reminder time, default to "09:00"
    let reminder_time = settings
        .as_ref()
        .and_then(|s| s.daily_reminder_time.clone())
        .unwrap_or_else(|| "09:00".to_string());

    // Parse the reminder time
    let parts: Vec<&str> = reminder_time.split(':').collect();
    if parts.len() != 2 {
        return;
    }
    let target_hour: u32 = match parts[0].parse() {
        Ok(h) => h,
        Err(_) => return,
    };
    let target_min: u32 = match parts[1].parse() {
        Ok(m) => m,
        Err(_) => return,
    };

    let current_hour = now.hour();
    let current_min = now.minute();

    // Fire if we've passed the target time (within today)
    if current_hour > target_hour || (current_hour == target_hour && current_min >= target_min) {
        // Count upcoming deadlines from the pending notifications
        let pending = get_pending(store);
        let deadline_count = pending
            .iter()
            .filter(|n| n.notification_type == "deadline" && n.scheduled_at > now.timestamp())
            .count();

        let body = if deadline_count > 0 {
            format!(
                "You have {} deadline{} coming up today. Open your tasks to review.",
                deadline_count,
                if deadline_count == 1 { "" } else { "s" }
            )
        } else {
            "Review your deadlines for today. Open your tasks to check what's coming up.".to_string()
        };

        info!("Sending daily digest notification");
        if let Err(e) = app
            .notification()
            .builder()
            .title("Daily Deadline Reminder")
            .body(&body)
            .show()
        {
            error!("Failed to send daily digest: {}", e);
        }

        // Mark as sent today
        if let Some(obj) = store.as_object_mut() {
            obj.insert(
                "lastDailyDigestDate".to_string(),
                serde_json::Value::String(today),
            );
            if let Err(e) = write_store_file(app_data_dir, store) {
                error!("Failed to update last digest date: {}", e);
            }
        }
    }
}

use chrono::Timelike;
