use crate::wallet::format_public_key;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

#[component]
pub fn Header(
    wallet_connected: bool,
    public_key: Option<Pubkey>,
    on_connect: EventHandler<MouseEvent>,
    on_disconnect: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        header { class: "bg-gray-800 bg-opacity-90 backdrop-blur-md shadow-2xl border-b border-gray-700 sticky top-0 z-40",
            div { class: "container mx-auto px-4 py-4",
                div { class: "flex justify-between items-center",
                    div { class: "flex items-center space-x-4",
                        div { class: "flex items-center space-x-3",
                            div { class: "text-3xl animate-pulse", "🛡️" }
                            div {
                                h1 { class: "text-3xl font-bold bg-gradient-to-r from-blue-400 to-purple-600 bg-clip-text text-transparent", "Cyber Vault" }
                                p { class: "text-gray-400 text-sm font-medium", "Decentralized Dead Man's Switch" }
                            }
                        }
                    }

                    div { class: "flex items-center space-x-4",
                        if wallet_connected {
                            div { class: "flex items-center space-x-3 bg-gray-700 bg-opacity-50 px-4 py-2 rounded-lg border border-gray-600",
                                div { class: "flex items-center space-x-2",
                                    div { class: "w-2 h-2 bg-green-500 rounded-full animate-pulse" }
                                    span { class: "bg-green-500 text-xs px-2 py-1 rounded-full font-semibold text-white",
                                        "Connected"
                                    }
                                }
                                if let Some(pubkey) = public_key {
                                    div { class: "flex items-center space-x-2 ml-2",
                                        span { class: "text-gray-300 text-sm font-mono bg-gray-600 px-2 py-1 rounded",
                                            "{format_public_key(&pubkey)}"
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
                                    class: "bg-red-600 hover:bg-red-700 active:bg-red-800 px-4 py-2 rounded-lg transition-all duration-200 transform hover:scale-105 flex items-center space-x-2 font-medium",
                                    onclick: on_disconnect,
                                    span { "🔌" }
                                    span { "Disconnect" }
                                }
                            }
                        } else {
                            button {
                                class: "bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 active:from-blue-800 active:to-purple-800 px-6 py-3 rounded-lg transition-all duration-200 transform hover:scale-105 flex items-center space-x-2 font-semibold shadow-lg",
                                onclick: on_connect,
                                span { class: "text-lg", "🔗" }
                                span { "Connect Wallet" }
                            }
                        }
                    }
                }
            }
        }
    }
}
