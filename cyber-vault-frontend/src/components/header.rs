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
        header { class: "header-nav bg-black border-b border-gray-800",
            div { class: "container mx-auto px-4 py-4",
                div { class: "flex justify-between items-center",
                    div { class: "flex items-center space-x-4",
                        div { class: "text-2xl text-green-400", "[C]" }
                        div {
                            h1 { class: "text-xl font-semibold text-white font-mono tracking-wider", "CYPHER VAULT" }
                            p { class: "text-gray-400 text-sm font-mono", "> decentralized_dead_mans_switch.exe" }
                        }
                    }

                    div { class: "flex items-center space-x-4",
                        if wallet_connected {
                            div { class: "flex items-center space-x-3 bg-black px-4 py-2 border border-gray-800",
                                div { class: "flex items-center space-x-2",
                                    div { class: "w-2 h-2 bg-green-400" }
                                    span { class: "text-green-400 text-xs font-semibold font-mono uppercase", "ONLINE" }
                                }
                                if let Some(pubkey) = public_key {
                                    div { class: "flex items-center space-x-3 ml-4",
                                        span { class: "text-sm font-mono text-green-400 address-display",
                                            "{format_public_key(&pubkey)}"
                                        }
                                        button {
                                            class: "cypher-button secondary px-3 py-2 text-xs border-gray-600 text-gray-400 hover:bg-gray-900 hover:text-green-400",
                                            title: "Copy address",
                                            onclick: move |_| {
                                                // Copy to clipboard functionality would go here
                                            },
                                            "[COPY]"
                                        }
                                    }
                                }
                                button {
                                    class: "cypher-button secondary px-4 py-2 border-gray-600 text-gray-400 hover:bg-gray-900 hover:text-green-400",
                                    onclick: on_disconnect,
                                    "[DISCONNECT]"
                                }
                            }
                        } else {
                            button {
                                class: "cypher-button border-green-400 text-green-400 hover:bg-green-400 hover:text-black",
                                onclick: on_connect,
                                "[CONNECT]"
                            }
                        }
                    }
                }
            }
        }
    }
}
