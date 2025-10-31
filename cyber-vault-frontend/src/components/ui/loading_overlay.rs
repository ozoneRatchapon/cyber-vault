use dioxus::prelude::*;

#[component]
pub fn LoadingOverlay(message: Option<String>, show: bool) -> Element {
    if !show {
        return rsx! { "" };
    }

    let display_message = message.unwrap_or_else(|| "PROCESSING...".to_string());

    rsx! {
        div { class: "fixed inset-0 bg-black bg-opacity-80 backdrop-blur-sm z-50 flex items-center justify-center",
            div { class: "cypher-card bg-black border-green-400 p-8 max-w-sm w-full mx-4",
                div { class: "text-center space-y-4 font-mono",
                    div { class: "flex justify-center",
                        div { class: "cypher-loading" }
                    }
                    div {
                        h3 { class: "text-lg font-semibold text-green-400 mb-2 uppercase tracking-wider", "[PROCESSING]" }
                        p { class: "text-gray-400 text-sm", "{display_message}" }
                    }
                    div { class: "text-xs text-gray-600 font-mono",
                        "> Awaiting wallet approval..."
                    }
                    div { class: "mt-4 p-2 bg-black border border-gray-800 text-left",
                        div { class: "text-xs text-gray-600 font-mono space-y-1",
                            div { "status: pending" }
                            div { "transaction: initializing..." }
                            div { "network: solana_mainnet" }
                        }
                    }
                }
            }
        }
    }
}

