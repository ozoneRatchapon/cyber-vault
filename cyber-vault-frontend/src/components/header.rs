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
        header { class: "header-nav bg-[#141925] border-b border-[#2a3441]",
            div { class: "container mx-auto px-4 py-4",
                div { class: "flex justify-between items-center",
                    div { class: "flex items-center space-x-4",
                        div { class: "text-2xl text-cyan-300", "🛡️" }
                        div {
                            h1 { class: "text-xl font-semibold text-gray-200", "Cyber Vault" }
                            p { class: "text-gray-400 text-sm", "Decentralized Dead Man's Switch" }
                        }
                    }

                    div { class: "flex items-center space-x-4",
                        if wallet_connected {
                            div { class: "flex items-center space-x-3 bg-[#1e2433] px-4 py-2 rounded-none border border-[#2a3441]",
                                div { class: "flex items-center space-x-2",
                                    div { class: "w-2 h-2 bg-green-400 rounded-full" }
                                    span { class: "text-green-400 text-xs font-semibold", "ONLINE" }
                                }
                                if let Some(pubkey) = public_key {
                                    div { class: "flex items-center space-x-3 ml-4",
                                        span { class: "text-sm font-mono text-gray-400 bg-transparent px-3 py-2 border-b border-cyan-400",
                                            "{format_public_key(&pubkey)}"
                                        }
                                        button {
                                            class: "cyber-button secondary px-3 py-2 text-sm border-pink-500 text-pink-500 hover:bg-pink-500 hover:text-black",
                                            title: "Copy address",
                                            onclick: move |_| {
                                                // Copy to clipboard functionality would go here
                                            },
                                            "📋"
                                        }
                                    }
                                }
                                button {
                                    class: "cyber-button secondary px-4 py-2 border-pink-500 text-pink-500 hover:bg-pink-500 hover:text-black",
                                    onclick: on_disconnect,
                                    span { "🔌" }
                                    span { "Disconnect" }
                                }
                            }
                        } else {
                            button {
                                class: "cyber-button border-cyan-400 text-cyan-300 hover:bg-cyan-400 hover:text-black",
                                onclick: on_connect,
                                span { "🔗" }
                                span { "Connect Wallet" }
                            }
                        }
                    }
                }
            }
        }
    }
}
