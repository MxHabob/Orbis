use tauri::{AppHandle, Emitter, Manager, PhysicalPosition, Position};
#[cfg(target_os = "macos")]
use tauri::TitleBarStyle;
use tauri::{WebviewUrl, WebviewWindowBuilder};

use crate::state::AppState;

/// Creates the floating search window on-demand and hides it.
/// It will be shown/hidden via toggle_search_window.
pub async fn create_preloaded_search_window(
    app: &AppHandle,
    app_url: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let label = "search";
    let search_url = format!("{}/desktop/search", app_url);

    let builder = WebviewWindowBuilder::new(
        app,
        label,
        WebviewUrl::External(tauri::Url::parse(&search_url)?),
    )
    .title("Orbis Search")
    .inner_size(720.0, 450.0)
    .min_inner_size(720.0, 450.0)
    .resizable(false)
    .user_agent("Mozilla/5.0 (compatible; Orbis Desktop App)")
    .visible(false)
    .on_download(|_window, _event| true);

    #[cfg(target_os = "macos")]
    let builder = builder
        .decorations(false)
        .transparent(false)
        .shadow(true)
        .hidden_title(true)
        .title_bar_style(TitleBarStyle::Overlay);

    #[cfg(not(target_os = "macos"))]
    let builder = builder.decorations(true).transparent(false).shadow(true);

    let search_window = builder.build()?;
    search_window.center()?;

    // Auto-hide when focus is lost
    let window_clone = search_window.clone();
    search_window.on_window_event(move |event| {
        if let tauri::WindowEvent::Focused(false) = event {
            let _ = window_clone.emit("search-window-open", false);
            let _ = window_clone.set_always_on_top(false);
            let _ = window_clone.hide();
        }
    });

    Ok(())
}

/// Positions a window at the center of the monitor that contains the cursor.
pub fn position_on_active_monitor(
    app: &AppHandle,
    window: &tauri::WebviewWindow,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Ok(cursor) = app.cursor_position() {
        if let Ok(monitors) = app.available_monitors() {
            let monitor = monitors.iter().find(|m| {
                let p = m.position();
                let s = m.size();
                cursor.x >= p.x as f64
                    && cursor.x < (p.x + s.width as i32) as f64
                    && cursor.y >= p.y as f64
                    && cursor.y < (p.y + s.height as i32) as f64
            });

            if let Some(m) = monitor {
                let ws = window
                    .outer_size()
                    .unwrap_or(tauri::PhysicalSize { width: 720, height: 450 });
                let x = m.position().x + (m.size().width as i32 / 2) - (ws.width as i32 / 2);
                let y = m.position().y + (m.size().height as i32 / 2) - (ws.height as i32 / 2) + 15;
                window.set_position(Position::Physical(PhysicalPosition { x, y }))?;
                return Ok(());
            }
        }
    }
    window.center()?;
    Ok(())
}

/// Toggle the search spotlight window visibility.
pub fn toggle_search_window(
    app: &AppHandle,
    search_enabled: &std::sync::Arc<std::sync::Mutex<bool>>,
) -> Result<(), Box<dyn std::error::Error>> {
    let enabled = *search_enabled.lock().unwrap();

    if !enabled {
        if let Some(w) = app.get_webview_window("main") {
            w.show()?;
            w.set_focus()?;
        }
        return Ok(());
    }

    let label = "search";
    if let Some(w) = app.get_webview_window(label) {
        if w.is_visible()? {
            let _ = w.emit("search-window-open", false);
            w.set_always_on_top(false)?;
            w.hide()?;
        } else {
            w.set_always_on_top(true)?;
            position_on_active_monitor(app, &w)?;
            w.show()?;
            w.set_focus()?;
            let _ = w.emit("search-window-open", true);
        }
    } else {
        // Lazy-create on first use
        let app_clone = app.clone();
        let state = app.state::<std::sync::Arc<AppState>>();
        let _server_url = "http://localhost:3002".to_string(); // fallback only

        tauri::async_runtime::block_on(async move {
            let url = format!("http://localhost:3002");
            if create_preloaded_search_window(&app_clone, &url).await.is_ok() {
                if let Some(w) = app_clone.get_webview_window("search") {
                    let _ = w.set_always_on_top(true);
                    let _ = position_on_active_monitor(&app_clone, &w);
                    let _ = w.show();
                    let _ = w.set_focus();
                    let _ = w.emit("search-window-open", true);
                }
            }
        });
    }

    Ok(())
}
