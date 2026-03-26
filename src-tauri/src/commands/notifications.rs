use serde::{Deserialize, Serialize};
use tauri::{AppHandle, command, Runtime};
use tauri_plugin_notification::NotificationExt;
use std::time::Duration;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PendingNotification {
    pub id: String,
    pub title: String,
    pub body: String,
    pub scheduled_at: i64, // Unix timestamp in seconds
    pub notification_type: String,
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
    let _payload = PendingNotification {
        id,
        title: title.clone(),
        body: body.clone(),
        scheduled_at,
        notification_type,
    };

    // Calculate how long to wait
    let now = Utc::now().timestamp();
    let delay = if scheduled_at > now {
        (scheduled_at - now) as u64
    } else {
        0
    };

    let handle_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        if delay > 0 {
            tokio::time::sleep(Duration::from_secs(delay)).await;
        }

        // Trigger notification
        let _ = handle_clone.notification().builder()
            .title(title)
            .body(body)
            .show();
    });

    Ok(())
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
