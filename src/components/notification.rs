use crate::theme::use_theme;
use leptos::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotificationPosition {
    TopLeft,
    TopRight,
    TopCenter,
    BottomLeft,
    BottomRight,
    BottomCenter,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NotificationColor {
    Info,
    Success,
    Warning,
    Error,
}

impl NotificationColor {
    fn to_color_name(&self) -> &str {
        match self {
            NotificationColor::Info => "blue",
            NotificationColor::Success => "green",
            NotificationColor::Warning => "yellow",
            NotificationColor::Error => "red",
        }
    }

    fn default_icon(&self) -> &str {
        match self {
            NotificationColor::Info => "ℹ️",
            NotificationColor::Success => "✓",
            NotificationColor::Warning => "⚠️",
            NotificationColor::Error => "✕",
        }
    }
}

#[derive(Clone, Debug)]
pub struct NotificationData {
    pub id: usize,
    pub title: Option<String>,
    pub message: String,
    pub color: NotificationColor,
    pub icon: Option<String>,
    pub auto_close: Option<u32>, // milliseconds
}

type NotificationMap = RwSignal<HashMap<usize, NotificationData>>;
type NotificationIdCounter = RwSignal<usize>;

#[component]
pub fn NotificationProvider(
    #[prop(optional)] position: Option<NotificationPosition>,
    #[prop(optional)] max_notifications: Option<usize>,
    children: Children,
) -> impl IntoView {
    let position = position.unwrap_or(NotificationPosition::TopRight);
    let notifications = RwSignal::new(HashMap::new());
    let id_counter = RwSignal::new(0usize);

    provide_context::<NotificationMap>(notifications);
    provide_context::<NotificationIdCounter>(id_counter);
    provide_context::<Signal<NotificationPosition>>(Signal::derive(move || position));
    provide_context::<Signal<usize>>(Signal::derive(move || {
        max_notifications.unwrap_or(5)
    }));

    view! {
        <>
            {children()}
            <NotificationContainer />
        </>
    }
}

#[component]
fn NotificationContainer() -> impl IntoView {
    let theme = use_theme();
    let notifications =
        use_context::<NotificationMap>().unwrap_or_else(|| RwSignal::new(HashMap::new()));
    let position = use_context::<Signal<NotificationPosition>>()
        .unwrap_or_else(|| Signal::derive(move || NotificationPosition::TopRight));

    let container_styles = move || {
        let theme_val = theme.get();

        let (top, left, right, bottom, align_items) = match position.get() {
            NotificationPosition::TopLeft => ("1rem", "1rem", "auto", "auto", "flex-start"),
            NotificationPosition::TopRight => ("1rem", "auto", "1rem", "auto", "flex-end"),
            NotificationPosition::TopCenter => ("1rem", "50%", "auto", "auto", "center"),
            NotificationPosition::BottomLeft => ("auto", "1rem", "auto", "1rem", "flex-start"),
            NotificationPosition::BottomRight => ("auto", "auto", "1rem", "1rem", "flex-end"),
            NotificationPosition::BottomCenter => ("auto", "50%", "auto", "1rem", "center"),
        };

        let transform = if matches!(
            position.get(),
            NotificationPosition::TopCenter | NotificationPosition::BottomCenter
        ) {
            "translateX(-50%)"
        } else {
            "none"
        };

        format!(
            "position: fixed; \
             top: {}; \
             left: {}; \
             right: {}; \
             bottom: {}; \
             transform: {}; \
             display: flex; \
             flex-direction: column; \
             gap: {}; \
             align-items: {}; \
             z-index: 10000; \
             pointer-events: none;",
            top, left, right, bottom, transform, theme_val.spacing.sm, align_items
        )
    };

    view! {
        <div class="mingot-notification-container" style=container_styles>
            {move || {
                let notifs = notifications.get();
                notifs
                    .values()
                    .cloned()
                    .collect::<Vec<_>>()
                    .into_iter()
                    .map(|notification| {
                        view! { <NotificationItem notification=notification /> }
                    })
                    .collect::<Vec<_>>()
            }}

        </div>
    }
}

#[component]
fn NotificationItem(notification: NotificationData) -> impl IntoView {
    let theme = use_theme();
    let notifications =
        use_context::<NotificationMap>().unwrap_or_else(|| RwSignal::new(HashMap::new()));

    let id = notification.id;
    let is_visible = RwSignal::new(true);

    // Auto-close timer
    if let Some(duration) = notification.auto_close {
        set_timeout(
            move || {
                is_visible.set(false);
                // Wait for animation then remove
                set_timeout(
                    move || {
                        notifications.update(|n| {
                            n.remove(&id);
                        });
                    },
                    std::time::Duration::from_millis(300),
                );
            },
            std::time::Duration::from_millis(duration as u64),
        );
    }

    let notification_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let color_name = notification.color.to_color_name();

        let bg_color = scheme_colors
            .get_color(color_name, 0)
            .unwrap_or_else(|| "#e7f5ff".to_string());
        let border_color = scheme_colors
            .get_color(color_name, 6)
            .unwrap_or_else(|| "#228be6".to_string());
        let text_color = scheme_colors
            .get_color(color_name, 7)
            .unwrap_or_else(|| "#1c7ed6".to_string());

        let opacity = if is_visible.get() { "1" } else { "0" };
        let transform = if is_visible.get() {
            "translateX(0)"
        } else {
            "translateX(100px)"
        };

        format!(
            "display: flex; \
             gap: {}; \
             padding: {} {}; \
             background-color: {}; \
             color: {}; \
             border: 1px solid {}; \
             border-left: 4px solid {}; \
             border-radius: {}; \
             box-shadow: {}; \
             min-width: 300px; \
             max-width: 400px; \
             pointer-events: auto; \
             opacity: {}; \
             transform: {}; \
             transition: all 0.3s ease;",
            theme_val.spacing.sm,
            theme_val.spacing.md,
            theme_val.spacing.lg,
            bg_color,
            text_color,
            border_color,
            border_color,
            theme_val.radius.sm,
            theme_val.shadows.lg,
            opacity,
            transform
        )
    };

    let icon_styles = move || {
        let theme_val = theme.get();
        format!(
            "font-size: {}; \
             flex-shrink: 0;",
            theme_val.typography.font_sizes.lg
        )
    };

    let content_styles = "flex: 1; display: flex; flex-direction: column; gap: 0.25rem;".to_string();

    let title_styles = move || {
        let theme_val = theme.get();
        format!(
            "font-weight: {}; \
             font-size: {}; \
             margin: 0;",
            theme_val.typography.font_weights.bold, theme_val.typography.font_sizes.sm
        )
    };

    let message_styles = move || {
        let theme_val = theme.get();
        format!("font-size: {}; margin: 0;", theme_val.typography.font_sizes.sm)
    };

    let close_button_styles = move || {
        let theme_val = theme.get();
        format!(
            "background: none; \
             border: none; \
             font-size: {}; \
             cursor: pointer; \
             padding: 0; \
             opacity: 0.6; \
             transition: opacity 0.15s ease; \
             flex-shrink: 0;",
            theme_val.typography.font_sizes.md
        )
    };

    let handle_close = move |_| {
        is_visible.set(false);
        set_timeout(
            move || {
                notifications.update(|n| {
                    n.remove(&id);
                });
            },
            std::time::Duration::from_millis(300),
        );
    };

    let icon_display = notification
        .icon
        .unwrap_or_else(|| notification.color.default_icon().to_string());

    view! {
        <div class="mingot-notification" style=notification_styles>
            <div class="mingot-notification-icon" style=icon_styles>
                {icon_display}
            </div>

            <div class="mingot-notification-content" style=content_styles>
                {notification.title.as_ref().map(|t| {
                    view! { <div class="mingot-notification-title" style=title_styles>{t.clone()}</div> }
                })}
                <div class="mingot-notification-message" style=message_styles>
                    {notification.message}
                </div>
            </div>

            <button class="mingot-notification-close" style=close_button_styles on:click=handle_close>
                "✕"
            </button>
        </div>
    }
}

// Hook for showing notifications
pub fn use_notifications() -> impl Fn(NotificationData) {
    let notifications =
        use_context::<NotificationMap>().unwrap_or_else(|| RwSignal::new(HashMap::new()));
    let id_counter =
        use_context::<NotificationIdCounter>().unwrap_or_else(|| RwSignal::new(0));
    let max_notifications =
        use_context::<Signal<usize>>().unwrap_or_else(|| Signal::derive(move || 5));

    move |mut data: NotificationData| {
        let id = id_counter.get();
        id_counter.update(|c| *c += 1);
        data.id = id;

        notifications.update(|n| {
            // Remove oldest if at max
            if n.len() >= max_notifications.get() {
                if let Some(oldest_id) = n.keys().min().copied() {
                    n.remove(&oldest_id);
                }
            }
            n.insert(id, data);
        });
    }
}

// Helper to create notifications easily
pub fn show_notification(
    message: impl Into<String>,
    color: NotificationColor,
    title: Option<String>,
) -> NotificationData {
    NotificationData {
        id: 0, // Will be set by use_notifications
        title,
        message: message.into(),
        color,
        icon: None,
        auto_close: Some(5000), // 5 seconds default
    }
}
