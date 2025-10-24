use dioxus::prelude::*;

#[component]
pub fn LoadingSpinner(
    size: Option<String>,
    color: Option<String>,
    text: Option<String>,
) -> Element {
    let size_class = size.unwrap_or("h-8 w-8".to_string());
    let color_class = color.unwrap_or("border-blue-500".to_string());
    let loading_text = text.unwrap_or("Loading...".to_string());

    rsx! {
        div { class: "flex flex-col items-center justify-center space-y-3",
            div { class: "relative",
                div { class: "{size_class} {color_class} border-4 border-opacity-20 rounded-full animate-spin" }
                div { class: "absolute top-0 left-0 {size_class} {color_class} border-4 border-t-transparent rounded-full animate-spin" }
            }
            if !loading_text.is_empty() {
                p { class: "text-sm text-gray-400 animate-pulse", "{loading_text}" }
            }
        }
    }
}

#[component]
pub fn FullScreenLoader(text: Option<String>) -> Element {
    let loading_text = text.unwrap_or("Processing...".to_string());

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-60 backdrop-blur-sm flex items-center justify-center z-50",
            div { class: "bg-gray-800 bg-opacity-90 rounded-2xl p-8 shadow-2xl border border-gray-700",
                LoadingSpinner {
                    size: "h-12 w-12".to_string(),
                    color: "border-blue-500".to_string(),
                    text: loading_text,
                }
            }
        }
    }
}
