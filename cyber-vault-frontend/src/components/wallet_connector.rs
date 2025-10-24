use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

#[component]
pub fn WalletConnector(
    wallet_connected: bool,
    public_key: Option<Pubkey>,
    on_connect: EventHandler<MouseEvent>,
    on_disconnect: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "bg-gray-800 rounded-lg p-6 text-center",
            div { class: "mb-6",
                div { class: "text-6xl mb-4", "🔐" }
                h2 { class: "text-2xl font-bold text-blue-400 mb-2", "Wallet Connection" }
                p { class: "text-gray-400", "Connect your wallet to access Cyber Vault features" }
            }

            if wallet_connected {
                div { class: "space-y-4",
                    div { class: "bg-green-900 border border-green-700 rounded-lg p-4",
                        div { class: "flex items-center justify-center space-x-2 mb-2",
                            div { class: "w-3 h-3 bg-green-500 rounded-full animate-pulse" }
                            span { class: "text-green-400 font-semibold", "Connected" }
                        }
                        if let Some(pubkey) = public_key {
                            p { class: "text-gray-300 font-mono text-sm", "{pubkey}" }
                        }
                    }

                    button {
                        class: "bg-red-600 hover:bg-red-700 px-6 py-3 rounded-lg transition-colors font-medium",
                        onclick: on_disconnect,
                        "Disconnect Wallet"
                    }
                }
            } else {
                div { class: "space-y-4",
                    div { class: "bg-gray-700 border border-gray-600 rounded-lg p-4",
                        h3 { class: "text-yellow-400 font-semibold mb-2", "Supported Wallets" }
                        div { class: "grid grid-cols-2 gap-3 text-sm",
                            div { class: "flex items-center space-x-2",
                                span { "👻" }
                                span { "Phantom" }
                            }
                            div { class: "flex items-center space-x-2",
                                span { "🦊" }
                                span { "Solflare" }
                            }
                            div { class: "flex items-center space-x-2",
                                span { "💎" }
                                span { "Backpack" }
                            }
                            div { class: "flex items-center space-x-2",
                                span { "🌊" }
                                span { "Glow" }
                            }
                        }
                    }

                    button {
                        class: "bg-blue-600 hover:bg-blue-700 px-8 py-3 rounded-lg transition-colors font-medium text-lg",
                        onclick: on_connect,
                        "🔗 Connect Wallet"
                    }

                    p { class: "text-xs text-gray-500 mt-4",
                        "Make sure you have a Solana wallet installed and connected to the correct network"
                    }
                }
            }
        }
    }
}
