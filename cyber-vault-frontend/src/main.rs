// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use dioxus::document;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;

// Load the Tailwind CSS asset
static CSS: Asset = asset!("/assets/tailwind.css");

// Component imports
mod components;
mod vault_operations;
mod wallet;

use components::{CreateVaultForm, Header, VaultList};
use wallet::{format_public_key, WalletProvider};

// Wallet connection state
#[derive(Clone, Debug)]
pub struct WalletState {
    pub connected: bool,
    pub public_key: Option<Pubkey>,
}

impl Default for WalletState {
    fn default() -> Self {
        Self {
            connected: false,
            public_key: None,
        }
    }
}

// Vault data structure from IDL
#[derive(Clone, Debug, PartialEq)]
pub struct VaultInfo {
    pub pubkey: String,
    pub owner: String,
    pub beneficiary: String,
    pub token_mint: String,
    pub balance: u64,
    pub inactivity_period: i64,
    pub last_heartbeat: i64,
}

// Application state
#[derive(Clone, Default)]
pub struct AppState {
    pub wallet: WalletState,
    pub vaults: Vec<VaultInfo>,
    pub selected_vault: Option<VaultInfo>,
    pub error: Option<String>,
    pub success: Option<String>,
    pub is_loading: bool,
}

/// The main App component is the root of your application. Every component in Dioxus is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // Simple Tailwind test - remove this after confirming it works
    let _test_styles = "bg-red-500 text-white p-4 rounded-lg";

    let mut state = use_signal(AppState::default);
    let wallet_provider = use_signal(|| wallet::WalletProvider::new());

    // Check wallet availability on startup
    {
        let mut state_clone = state.clone();
        spawn(async move {
            let is_available = WalletProvider::is_wallet_available();
            if !is_available {
                state_clone.write().error = Some(
                    "No wallet detected. Please install Phantom or Solflare wallet extension."
                        .to_string(),
                );
            }
        });
    }

    // Handle wallet connection
    let handle_wallet_connect = Callback::new(move |_| {
        let mut state_clone = state.clone();
        let mut wallet_provider_clone = wallet_provider.clone();
        spawn(async move {
            state_clone.write().is_loading = true;
            match wallet_provider_clone.write().connect().await {
                Ok(pubkey) => {
                    state_clone.write().wallet.connected = true;
                    state_clone.write().wallet.public_key = Some(pubkey);
                    state_clone.write().error = None;
                    state_clone.write().success =
                        Some("Wallet connected successfully! 🎉".to_string());
                }
                Err(e) => {
                    state_clone.write().error =
                        Some("Failed to connect wallet. Please try again.".to_string());
                }
            }
            state_clone.write().is_loading = false;
        });
    });

    // Handle wallet disconnection
    let handle_wallet_disconnect = Callback::new(move |_| {
        let mut state_clone = state.clone();
        let mut wallet_provider_clone = wallet_provider.clone();
        spawn(async move {
            state_clone.write().is_loading = true;
            match wallet_provider_clone.write().disconnect().await {
                Ok(_) => {
                    state_clone.write().wallet.connected = false;
                    state_clone.write().wallet.public_key = None;
                    state_clone.write().vaults.clear();
                    state_clone.write().selected_vault = None;
                    state_clone.write().success =
                        Some("Wallet disconnected successfully".to_string());
                }
                Err(e) => {
                    state_clone.write().error =
                        Some("Failed to disconnect wallet. Please try again.".to_string());
                }
            }
            state_clone.write().is_loading = false;
        });
    });

    // Action handlers with better UX feedback
    let handle_create_vault = Callback::new(
        move |(beneficiary, period, amount, mint): (String, i64, u64, String)| {
            state.write().success = Some(
                "🏦 Vault creation initiated! Check your wallet for confirmation.".to_string(),
            );
            // TODO: Implement actual vault creation
        },
    );

    let handle_heartbeat = Callback::new(
        move |(_owner, _beneficiary, _mint): (String, String, String)| {
            state.write().success =
                Some("💓 Heartbeat sent successfully! Timer reset.".to_string());
            // TODO: Implement actual heartbeat
        },
    );

    let handle_claim = Callback::new(
        move |(_owner, _beneficiary, _mint): (String, String, String)| {
            state.write().success =
                Some("🔄 Claim initiated! Check your wallet for confirmation.".to_string());
            // TODO: Implement actual claim
        },
    );

    let handle_emergency_withdraw = Callback::new(
        move |(_owner, _beneficiary, _mint, _amount): (String, String, String, u64)| {
            state.write().success =
                Some("🚨 Emergency withdrawal initiated! Check your wallet.".to_string());
            // TODO: Implement actual emergency withdraw
        },
    );

    rsx! {
        document::Stylesheet { href: CSS }

        div { class: "min-h-screen bg-[#0A0F1A] text-gray-200",
            Header {
                wallet_connected: state.read().wallet.connected,
                public_key: state.read().wallet.public_key,
                on_connect: handle_wallet_connect,
                on_disconnect: handle_wallet_disconnect,
            }

            main { class: "container mx-auto px-4 py-8",
                // Loading indicator
                if state.read().is_loading {
                    div { class: "cyber-card mb-6 bg-transparent border-l-4 border-cyan-400",
                        div { class: "flex items-center space-x-3",
                            div { class: "cyber-loading" }
                            span { class: "text-cyan-300", "Processing..." }
                        }
                    }
                }

                // Success notifications
                if let Some(success) = &state.read().success {
                    div { class: "cyber-card mb-6 bg-transparent border-l-4 border-green-400",
                        div { class: "flex justify-between items-center",
                            div { class: "flex items-center space-x-3",
                                span { class: "text-xl text-green-400", "✓" }
                                span { class: "text-green-400", "{success}" }
                            }
                            button {
                                class: "cyber-button secondary px-3 py-1 text-xs border-green-400 text-green-400 hover:bg-green-400 hover:text-black",
                                onclick: move |_| state.write().success = None,
                                "×"
                            }
                        }
                    }
                }

                // Error notifications
                if let Some(error) = &state.read().error {
                    div { class: "cyber-card mb-6 bg-transparent border-l-4 border-pink-500",
                        div { class: "flex justify-between items-center",
                            div { class: "flex items-center space-x-3",
                                span { class: "text-xl text-pink-500", "⚠️" }
                                span { class: "text-pink-500", "{error}" }
                            }
                            button {
                                class: "cyber-button secondary px-3 py-1 text-xs border-pink-500 text-pink-500 hover:bg-pink-500 hover:text-black",
                                onclick: move |_| state.write().error = None,
                                "×"
                            }
                        }
                    }
                }

                // Main content
                if state.read().wallet.connected {
                    div { class: "space-y-8",
                        // Connected wallet info
                        div { class: "cyber-card bg-[#141925]",
                            h2 {
                                class: "text-2xl mb-4 text-cyan-300 font-semibold",
                                "✓ Wallet Connected"
                            }
                            if let Some(pubkey) = &state.read().wallet.public_key {
                                div { class: "space-y-2",
                                    p { class: "text-gray-300", "Public Key:" }
                                    p { class: "font-mono text-sm break-all text-cyan-300",
                                        "{format_public_key(pubkey)}"
                                    }
                                }
                            }
                        }

                        // Vault Management
                        // Vault List (when available)
                        VaultList {
                            vaults: state.read().vaults.clone(),
                            selected_vault: state.read().selected_vault.clone(),
                            on_select: Callback::new(move |vault: VaultInfo| {
                                state.write().selected_vault = Some(vault);
                            }),
                        }

                        // Create Vault Form
                        CreateVaultForm {
                            public_key: state.read().wallet.public_key,
                            on_create_vault: handle_create_vault,
                        }

                        // Vault Actions (when a vault is selected)
                        if let Some(selected_vault) = state.read().selected_vault.clone() {
                            div { class: "cyber-card bg-[#141925]",
                                h3 {
                                    class: "text-xl mb-6 text-pink-500 font-semibold",
                                    "🎯 Vault Actions"
                                }

                                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4",
                                    button {
                                        class: "cyber-button",
                                        onclick: {
                                            let vault = selected_vault.clone();
                                            move |_| {
                                                handle_heartbeat.call((
                                                    vault.owner.clone(),
                                                    vault.beneficiary.clone(),
                                                    vault.token_mint.clone(),
                                                ));
                                            }
                                        },
                                        "💓 Send Heartbeat"
                                    }

                                    button {
                                        class: "cyber-button",
                                        onclick: {
                                            let vault = selected_vault.clone();
                                            move |_| {
                                                handle_claim.call((
                                                    vault.owner.clone(),
                                                    vault.beneficiary.clone(),
                                                    vault.token_mint.clone(),
                                                ));
                                            }
                                        },
                                        "🔄 Claim Funds"
                                    }

                                    button {
                                        class: "cyber-button",
                                        onclick: {
                                            let vault = selected_vault.clone();
                                            move |_| {
                                                handle_emergency_withdraw.call((
                                                    vault.owner.clone(),
                                                    vault.beneficiary.clone(),
                                                    vault.token_mint.clone(),
                                                    1000000, // 0.001 SOL
                                                ));
                                            }
                                        },
                                        "🚨 Emergency Withdraw"
                                    }

                                    button {
                                        class: "cyber-button secondary",
                                        onclick: move |_| state.write().selected_vault = None,
                                        "Close"
                                    }
                                }
                            }
                        }
                    }
                } else {
                    // Not connected state
                    div { class: "text-center space-y-12 py-16",
                        div { class: "space-y-6 max-w-3xl mx-auto",
                            h1 {
                                class: "text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-300 to-pink-500",
                                "Cyber Vault"
                            }
                            p {
                                class: "text-2xl text-gray-400 leading-relaxed",
                                "Your digital safety net. Protect your assets with a dead man's switch on Solana."
                            }
                            div { class: "flex justify-center space-x-4",
                                span { class: "inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-green-400 bg-opacity-10 text-green-400 border border-green-400 border-opacity-30",
                                    "✓ Secure"
                                }
                                span { class: "inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-cyan-400 bg-opacity-10 text-cyan-300 border border-cyan-400 border-opacity-30",
                                    "✓ Decentralized"
                                }
                                span { class: "inline-flex items-center px-3 py-1 rounded-full text-xs font-medium bg-pink-400 bg-opacity-10 text-pink-400 border border-pink-400 border-opacity-30",
                                    "✓ Trustless"
                                }
                            }
                        }

                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto",
                            div { class: "cyber-card bg-[#141925] hover:border-cyan-400 hover:transform hover:-translate-y-2 transition-all duration-300",
                                div { class: "text-5xl mb-4 text-cyan-300", "🏦" }
                                h3 { class: "text-xl font-bold mb-3 text-gray-200", "1. Create Vault" }
                                p { class: "text-gray-400", "Choose your beneficiary and deposit tokens securely" }
                            }
                            div { class: "cyber-card bg-[#141925] hover:border-pink-500 hover:transform hover:-translate-y-2 transition-all duration-300",
                                div { class: "text-5xl mb-4 text-pink-500", "💓" }
                                h3 { class: "text-xl font-bold mb-3 text-gray-200", "2. Send Heartbeats" }
                                p { class: "text-gray-400", "Regular signals keep your vault active and safe" }
                            }
                            div { class: "cyber-card bg-[#141925] hover:border-green-400 hover:transform hover:-translate-y-2 transition-all duration-300",
                                div { class: "text-5xl mb-4 text-green-400", "🎯" }
                                h3 { class: "text-xl font-bold mb-3 text-gray-200", "3. Auto-Protection" }
                                p { class: "text-gray-400", "Beneficiary can claim if you become inactive" }
                            }
                        }

                        div { class: "space-y-6 max-w-md mx-auto",
                            button {
                                class: "cyber-button w-full py-5 text-lg font-semibold border-cyan-400 text-cyan-300 hover:bg-cyan-400 hover:text-black glow",
                                onclick: handle_wallet_connect,
                                "🚀 Launch Your Vault"
                            }

                            div { class: "cyber-card bg-[#1e2433] border-gray-600",
                                div { class: "flex items-center justify-center space-x-6 text-sm text-gray-400",
                                    span { class: "flex items-center space-x-2",
                                        span { "👻" }
                                        span { "Phantom" }
                                    }
                                    span { class: "flex items-center space-x-2",
                                        span { "🌟" }
                                        span { "Solflare" }
                                    }
                                    span { class: "flex items-center space-x-2",
                                        span { "🎒" }
                                        span { "Backpack" }
                                    }
                                }
                            }
                        }

                        div { class: "cyber-card bg-[#141925] max-w-2xl mx-auto border-gray-600",
                            h3 { class: "text-lg font-semibold mb-3 text-cyan-300", "🛡️ How It Works" }
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 text-sm",
                                div { class: "flex items-start space-x-2",
                                    span { class: "text-green-400", "✓" }
                                    span { class: "text-gray-300", "Emergency withdrawal anytime" }
                                }
                                div { class: "flex items-start space-x-2",
                                    span { class: "text-green-400", "✓" }
                                    span { class: "text-gray-300", "Supports SOL, USDC, USDT" }
                                }
                                div { class: "flex items-start space-x-2",
                                    span { class: "text-green-400", "✓" }
                                    span { class: "text-gray-300", "Custom inactivity periods" }
                                }
                                div { class: "flex items-start space-x-2",
                                    span { class: "text-green-400", "✓" }
                                    span { class: "text-gray-300", "No middlemen or fees" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
