use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum StatusType {
    Active,
    Inactive,
    Pending,
    Expired,
    Success,
    Warning,
    Error,
}

impl StatusType {
    pub fn bg_class(&self) -> &'static str {
        match self {
            StatusType::Active => "bg-green-500 bg-opacity-20 border-green-600",
            StatusType::Inactive => "bg-gray-500 bg-opacity-20 border-gray-600",
            StatusType::Pending => "bg-yellow-500 bg-opacity-20 border-yellow-600",
            StatusType::Expired => "bg-red-500 bg-opacity-20 border-red-600",
            StatusType::Success => "bg-green-500 bg-opacity-20 border-green-600",
            StatusType::Warning => "bg-yellow-500 bg-opacity-20 border-yellow-600",
            StatusType::Error => "bg-red-500 bg-opacity-20 border-red-600",
        }
    }

    pub fn text_class(&self) -> &'static str {
        match self {
            StatusType::Active => "text-green-400",
            StatusType::Inactive => "text-gray-400",
            StatusType::Pending => "text-yellow-400",
            StatusType::Expired => "text-red-400",
            StatusType::Success => "text-green-400",
            StatusType::Warning => "text-yellow-400",
            StatusType::Error => "text-red-400",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            StatusType::Active => "🟢",
            StatusType::Inactive => "⚫",
            StatusType::Pending => "🟡",
            StatusType::Expired => "🔴",
            StatusType::Success => "✅",
            StatusType::Warning => "⚠️",
            StatusType::Error => "❌",
        }
    }

    pub fn text(&self) -> &'static str {
        match self {
            StatusType::Active => "Active",
            StatusType::Inactive => "Inactive",
            StatusType::Pending => "Pending",
            StatusType::Expired => "Expired",
            StatusType::Success => "Success",
            StatusType::Warning => "Warning",
            StatusType::Error => "Error",
        }
    }
}

#[component]
pub fn StatusBadge(
    status_type: StatusType,
    show_text: Option<bool>,
    size: Option<String>,
    animated: Option<bool>,
) -> Element {
    let should_show_text = show_text.unwrap_or(true);
    let size_class = size.unwrap_or("px-3 py-1 text-sm".to_string());
    let should_animate = animated.unwrap_or(false);

    let base_class = format!(
        "{} {} {} rounded-full font-semibold border transition-all duration-200",
        status_type.bg_class(),
        status_type.text_class(),
        size_class
    );

    let final_class = if should_animate {
        format!("{} animate-pulse", base_class)
    } else {
        base_class
    };

    let icon_size = if !should_show_text {
        "text-xs"
    } else {
        "text-sm"
    };

    rsx! {
        span {
            class: "{final_class}",
            title: status_type.text(),
            div { class: "flex items-center space-x-1",
                span { class: "{icon_size}", "{status_type.icon()}" }
                if should_show_text {
                    span { "{status_type.text()}" }
                }
            }
        }
    }
}

#[component]
pub fn ConnectionStatus(
    is_connected: bool,
    address: Option<String>,
    on_connect: EventHandler<MouseEvent>,
    on_disconnect: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "flex items-center space-x-3 bg-gray-700 bg-opacity-50 px-4 py-2 rounded-lg border border-gray-600",
            if is_connected {
                div { class: "flex items-center space-x-2",
                    StatusBadge {
                        status_type: StatusType::Active,
                        show_text: Some(true),
                        size: "px-2 py-1 text-xs".to_string(),
                        animated: Some(true),
                    }
                    if let Some(addr) = address {
                        div { class: "flex items-center space-x-2",
                            span { class: "text-gray-300 text-sm font-mono bg-gray-600 px-2 py-1 rounded",
                                "{addr}"
                            }
                            button {
                                class: "text-gray-400 hover:text-white transition-colors p-1",
                                title: "Copy address",
                                onclick: move |_| {
                                    // Copy to clipboard functionality would go here
                                },
                                "📋"
                            }
                        }
                    }
                    button {
                        class: "bg-red-600 hover:bg-red-700 active:bg-red-800 px-3 py-1 rounded-lg transition-all duration-200 transform hover:scale-105 text-sm font-medium flex items-center space-x-1",
                        onclick: on_disconnect,
                        span { "🔌" }
                        span { "Disconnect" }
                    }
                }
            } else {
                button {
                    class: "bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 active:from-blue-800 active:to-purple-800 px-4 py-2 rounded-lg transition-all duration-200 transform hover:scale-105 flex items-center space-x-2 font-semibold text-sm",
                    onclick: on_connect,
                    span { "🔗" }
                    span { "Connect Wallet" }
                }
            }
        }
    }
}

#[component]
pub fn LoadingStatus(message: Option<String>, progress: Option<f32>) -> Element {
    let loading_message = message.unwrap_or("Loading...".to_string());
    let progress_value = progress.unwrap_or(0.0);

    rsx! {
        div { class: "flex flex-col items-center space-y-3",
            div { class: "relative",
                div { class: "w-12 h-12 border-4 border-blue-500 border-opacity-20 rounded-full animate-spin" }
                div { class: "absolute top-0 left-0 w-12 h-12 border-4 border-blue-500 border-t-transparent rounded-full animate-spin" }
            }
            p { class: "text-sm text-gray-400 animate-pulse", "{loading_message}" }
            if progress_value > 0.0 {
                div { class: "w-full bg-gray-700 rounded-full h-2",
                    div {
                        class: "bg-blue-500 h-2 rounded-full transition-all duration-300",
                        style: "width: {progress_value}%",
                    }
                }
                p { class: "text-xs text-gray-500 mt-1", "{progress_value}% complete" }
            }
        }
    }
}
