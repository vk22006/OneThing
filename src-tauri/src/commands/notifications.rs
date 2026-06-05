use serde::{Deserialize, Serialize};
use tauri::{AppHandle, command, Manager, Runtime};
use tauri_plugin_notification::NotificationExt;
use std::time::Duration;
use chrono::Utc;
use log::{error, info};

use crate::notification_scheduler::PersistedNotification;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingNotification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub scheduled_at: i64, // Unix timestamp in seconds
    pub notification_type: String,
}

/// Read the store JSON file from disk.
fn read_store(app_data_dir: &std::path::Path) -> serde_json::Value {
    let store_path = app_data_dir.join("settings.json");
    match std::fs::read_to_string(&store_path) {
        Ok(contents) => serde_json::from_str(&contents).unwrap_or_else(|_| serde_json::json!({})),
        Err(_) => serde_json::json!({}),
    }
}

/// Write the store JSON file back to disk.
fn write_store(app_data_dir: &std::path::Path, value: &serde_json::Value) -> Result<(), String> {
    let store_path = app_data_dir.join("settings.json");
    let contents = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    std::fs::write(&store_path, contents).map_err(|e| e.to_string())
}

/// Persist a notification to the store file so the background scheduler can find it.
fn persist_notification<R: Runtime>(app: &AppHandle<R>, notif: &PersistedNotification) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let mut store = read_store(&app_data_dir);

    let mut pending: Vec<PersistedNotification> = store
        .get("pendingNotifications")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    // Replace if same ID exists, otherwise push
    if let Some(pos) = pending.iter().position(|n| n.id == notif.id) {
        pending[pos] = notif.clone();
    } else {
        pending.push(notif.clone());
    }

    let obj = store.as_object_mut().ok_or("store is not an object")?;
    obj.insert(
        "pendingNotifications".to_string(),
        serde_json::to_value(&pending).map_err(|e| e.to_string())?,
    );

    write_store(&app_data_dir, &store)
}

/// Remove a notification from the persisted store.
fn remove_persisted_notification<R: Runtime>(app: &AppHandle<R>, id: &str) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let mut store = read_store(&app_data_dir);

    let mut pending: Vec<PersistedNotification> = store
        .get("pendingNotifications")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or_default();

    pending.retain(|n| n.id != id);

    let obj = store.as_object_mut().ok_or("store is not an object")?;
    obj.insert(
        "pendingNotifications".to_string(),
        serde_json::to_value(&pending).map_err(|e| e.to_string())?,
    );

    write_store(&app_data_dir, &store)
}

#[command]
pub fn schedule_notification<R: Runtime>(
    app: AppHandle<R>,
    id: String,
    title: String,
    body: String,
    scheduled_at: i64,
    notification_type: String,
) -> Result<(), String> {
    let notif = PersistedNotification {
        id: id.clone(),
        title: title.clone(),
        body: body.clone(),
        scheduled_at,
        notification_type,
    };

    // Persist to disk so the background scheduler picks it up
    persist_notification(&app, &notif)?;

    info!("Scheduled notification '{}' for timestamp {}", id, scheduled_at);

    // Also fire via in-process async task for immediate responsiveness
    // (the background scheduler will also catch it, but a direct timer is more precise)
    let now = Utc::now().timestamp();
    let delay = if scheduled_at > now {
        (scheduled_at - now) as u64
    } else {
        0
    };

    let handle_clone = app.clone();
    let id_clone = id.clone();
    tauri::async_runtime::spawn(async move {
        if delay > 0 {
            tokio::time::sleep(Duration::from_secs(delay)).await;
        }

        // Fire notification
        if let Err(e) = handle_clone.notification().builder()
            .title(title)
            .body(body)
            .show()
        {
            error!("Failed to send scheduled notification: {}", e);
        }

        // Remove from persisted store after firing
        let _ = remove_persisted_notification(&handle_clone, &id_clone);
    });

    Ok(())
}

#[command]
pub fn cancel_notification<R: Runtime>(
    app: AppHandle<R>,
    id: String,
) -> Result<(), String> {
    info!("Cancelling notification '{}'", id);
    remove_persisted_notification(&app, &id)
}

#[command]
pub fn send_now<R: Runtime>(
    app: AppHandle<R>,
    title: String,
    body: String,
) -> Result<(), String> {
    app.notification()
        .builder()
        .title(title)
        .body(body)
        .show()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[command]
pub fn set_daily_reminder_time<R: Runtime>(
    app: AppHandle<R>,
    time: String,
) -> Result<(), String> {
    let app_data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let mut store = read_store(&app_data_dir);

    // Update notificationSettings.dailyReminderTime
    let mut settings: serde_json::Value = store
        .get("notificationSettings")
        .cloned()
        .unwrap_or_else(|| serde_json::json!({}));

    if let Some(obj) = settings.as_object_mut() {
        obj.insert(
            "dailyReminderTime".to_string(),
            serde_json::Value::String(time.clone()),
        );
    }

    if let Some(store_obj) = store.as_object_mut() {
        store_obj.insert("notificationSettings".to_string(), settings);
    }

    write_store(&app_data_dir, &store)?;
    info!("Daily reminder time set to {}", time);
    Ok(())
}
