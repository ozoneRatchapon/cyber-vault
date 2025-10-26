use dioxus::prelude::*;

#[component]
pub fn LoadingOverlay(message: Option<String>, show: bool) -> Element {
    if !show {
        return rsx! { "" };
    }

    let display_message = message.unwrap_or_else(|| "Processing...".to_string());

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-50 backdrop-blur-sm z-50 flex items-center justify-center",
            div { class: "cyber-card bg-[#141925] border-cyan-400 p-8 max-w-sm w-full mx-4",
                div { class: "text-center space-y-4",
                    div { class: "flex justify-center",
                        div { class: "cyber-loading text-4xl" }
                    }
                    div {
                        h3 { class: "text-lg font-semibold text-cyan-300 mb-2", "Processing Transaction" }
                        p { class: "text-gray-400 text-sm", "{display_message}" }
                    }
                    div { class: "text-xs text-gray-500",
                        "Please check your wallet and approve the transaction"
                    }
                }
            }
        }
    }
}
