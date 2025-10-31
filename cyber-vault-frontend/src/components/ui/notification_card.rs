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
            NotificationType::Success => "bg-green-400 bg-opacity-10 border-green-400",
            NotificationType::Error => "bg-gray-400 bg-opacity-10 border-gray-400",
            NotificationType::Warning => "bg-gray-400 bg-opacity-10 border-gray-400",
            NotificationType::Info => "bg-gray-400 bg-opacity-10 border-gray-400",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            NotificationType::Success => "text-green-400",
            NotificationType::Error => "text-gray-400",
            NotificationType::Warning => "text-gray-400",
            NotificationType::Info => "text-gray-400",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            NotificationType::Success => "[+]",
            NotificationType::Error => "[!]",
            NotificationType::Warning => "[#]",
            NotificationType::Info => "[i]",
        }
    }

    pub fn prefix(&self) -> &'static str {
        match self {
            NotificationType::Success => "SUCCESS",
            NotificationType::Error => "ERROR",
            NotificationType::Warning => "WARNING",
            NotificationType::Info => "INFO",
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

    rsx! {
        div {
            class: "cypher-card border-l-4 backdrop-blur-sm transition-all duration-200",
            class: "{notification_type.bg_class()}",
            div { class: "flex justify-between items-start",
                div { class: "flex items-start space-x-3 flex-1",
                    span { class: "text-green-400 font-mono text-xs font-semibold",
                        "{notification_type.prefix()}" }
                    span { class: "{notification_type.text_class()} font-mono text-sm",
                        "{message}" }
                }
                button {
                    class: "cypher-button secondary px-2 py-1 text-xs border-gray-600 text-gray-600 hover:bg-gray-900 hover:text-green-400 font-mono",
                    onclick: on_close,
                    title: "Close notification",
                    "[×]"
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
        div { class: "fixed top-20 right-4 z-50 space-y-3 max-w-sm font-mono",
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
