use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum NotificationType {
    Success,
    Error,
    Warning,
    Info,
}

impl NotificationType {
    pub fn bg_class(&self) -> &'static str {
        match self {
            NotificationType::Success => "bg-green-500 bg-opacity-20 border-green-500",
            NotificationType::Error => "bg-red-500 bg-opacity-20 border-red-500",
            NotificationType::Warning => "bg-yellow-500 bg-opacity-20 border-yellow-500",
            NotificationType::Info => "bg-blue-500 bg-opacity-20 border-blue-500",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            NotificationType::Success => "text-green-200",
            NotificationType::Error => "text-red-200",
            NotificationType::Warning => "text-yellow-200",
            NotificationType::Info => "text-blue-200",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            NotificationType::Success => "✓",
            NotificationType::Error => "⚠️",
            NotificationType::Warning => "⚡",
            NotificationType::Info => "ℹ️",
        }
    }
}

#[component]
pub fn NotificationCard(
    notification_type: NotificationType,
    message: String,
    on_close: EventHandler<MouseEvent>,
    auto_close: Option<bool>,
    duration: Option<u64>,
) -> Element {
    let should_auto_close = auto_close.unwrap_or(true);
    let close_duration = duration.unwrap_or(5000);

    // Note: Auto-close functionality disabled to avoid MouseEvent complexity
    // Manual close button provides better user control over notification display

    rsx! {
        div {
            class: "cyber-card neon-border scan-line backdrop-blur-sm transition-all duration-300 hover:scale-105 cyber-hover-glow",
            div { class: "flex justify-between items-center",
                div { class: "flex items-center space-x-4",
                    span { class: "text-2xl neon-text-cyan", "{notification_type.icon()}" }
                    span { class: "neon-text-cyan font-cyber text-sm", "{message}" }
                }
                button {
                    class: "cyber-button px-3 py-2 text-sm neon-text-pink hover:neon-glow",
                    onclick: on_close,
                    title: "Close notification",
                    "×"
                }
            }
        }
    }
}

#[component]
pub fn NotificationContainer(
    notifications: Vec<(String, NotificationType)>,
    on_remove: EventHandler<usize>,
) -> Element {
    rsx! {
        div { class: "fixed top-20 right-4 z-50 space-y-4 max-w-sm",
            for (index, (message, notification_type)) in notifications.iter().enumerate() {
                NotificationCard {
                    notification_type: notification_type.clone(),
                    message: message.clone(),
                    on_close: move |_| on_remove.call(index),
                    auto_close: Some(true),
                    duration: Some(5000),
                }
            }
        }
    }
}
