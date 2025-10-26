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
            match wallet_provider_clone.write().connect().await {
                Ok(pubkey) => {
                    state_clone.write().wallet.connected = true;
                    state_clone.write().wallet.public_key = Some(pubkey);
                    state_clone.write().error = None;
                    state_clone.write().error =
                        Some(format!("Successfully connected to wallet: {}", pubkey));
                }
                Err(e) => {
                    state_clone.write().error = Some(format!("Failed to connect wallet: {}", e));
                }
            }
        });
    });

    // Handle wallet disconnection
    let handle_wallet_disconnect = Callback::new(move |_| {
        let mut state_clone = state.clone();
        let mut wallet_provider_clone = wallet_provider.clone();
        spawn(async move {
            match wallet_provider_clone.write().disconnect().await {
                Ok(_) => {
                    state_clone.write().wallet.connected = false;
                    state_clone.write().wallet.public_key = None;
                    state_clone.write().error =
                        Some("Wallet disconnected successfully".to_string());
                }
                Err(e) => {
                    state_clone.write().error = Some(format!("Failed to disconnect wallet: {}", e));
                }
            }
        });
    });

    // Simple callback handlers (stub implementations for now)
    let handle_create_vault = Callback::new(
        move |(beneficiary, period, amount, mint): (String, i64, u64, String)| {
            state.write().error = Some(format!(
                "Create vault: beneficiary={}, period={}, amount={}, mint={}",
                beneficiary, period, amount, mint
            ));
        },
    );

    let handle_heartbeat = Callback::new(
        move |(owner, beneficiary, mint): (String, String, String)| {
            state.write().error = Some(format!(
                "Heartbeat: owner={}, beneficiary={}, mint={}",
                owner, beneficiary, mint
            ));
        },
    );

    let handle_claim = Callback::new(
        move |(owner, beneficiary, mint): (String, String, String)| {
            state.write().error = Some(format!(
                "Claim: owner={}, beneficiary={}, mint={}",
                owner, beneficiary, mint
            ));
        },
    );

    let handle_emergency_withdraw = Callback::new(
        move |(owner, beneficiary, mint, amount): (String, String, String, u64)| {
            state.write().error = Some(format!(
                "Emergency withdraw: owner={}, beneficiary={}, mint={}, amount={}",
                owner, beneficiary, mint, amount
            ));
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
                // Success/Error notifications
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
                    div { class: "text-center space-y-8 py-16",
                        div { class: "space-y-4",
                            h1 {
                                class: "text-5xl font-bold text-cyan-300",
                                "Cyber Vault"
                            }
                            p {
                                class: "text-xl text-gray-400",
                                "Dead Man's Switch on Solana"
                            }
                        }

                        div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6 max-w-6xl mx-auto",
                            div { class: "cyber-card bg-[#141925]",
                                div { class: "text-4xl mb-4 text-cyan-300", "🔐" }
                                h3 { class: "text-lg font-semibold mb-2 text-gray-200", "Create a Vault" }
                                p { class: "text-gray-400 text-sm", "Designate a beneficiary and set your terms" }
                            }
                            div { class: "cyber-card bg-[#141925]",
                                div { class: "text-4xl mb-4 text-pink-500", "⏰" }
                                h3 { class: "text-lg font-semibold mb-2 text-gray-200", "Set Inactivity Period" }
                                p { class: "text-gray-400 text-sm", "Choose how long before funds can be claimed" }
                            }
                            div { class: "cyber-card bg-[#141925]",
                                div { class: "text-4xl mb-4 text-cyan-300", "💰" }
                                h3 { class: "text-lg font-semibold mb-2 text-gray-200", "Deposit Assets" }
                                p { class: "text-gray-400 text-sm", "Secure your tokens in the smart contract" }
                            }
                            div { class: "cyber-card bg-[#141925]",
                                div { class: "text-4xl mb-4 text-green-400", "💓" }
                                h3 { class: "text-lg font-semibold mb-2 text-gray-200", "Send Heartbeats" }
                                p { class: "text-gray-400 text-sm", "Regular signals to keep your vault active" }
                            }
                            div { class: "cyber-card bg-[#141925]",
                                div { class: "text-4xl mb-4", "🔄" }
                                h3 { class: "text-lg font-semibold mb-2 text-gray-200", "Auto-Claim" }
                                p { class: "text-gray-400 text-sm", "Beneficiary can claim after inactivity" }
                            }
                            div { class: "cyber-card bg-[#141925]",
                                div { class: "text-4xl mb-4 text-red-400", "🚨" }
                                h3 { class: "text-lg font-semibold mb-2 text-gray-200", "Emergency Withdraw" }
                                p { class: "text-gray-400 text-sm", "Always retain access to your funds" }
                            }
                        }

                        div { class: "max-w-md mx-auto",
                            button {
                                class: "cyber-button w-full py-4 text-lg border-cyan-400 text-cyan-300 hover:bg-cyan-400 hover:text-black",
                                onclick: handle_wallet_connect,
                                "🔗 Connect Your Wallet to Get Started"
                            }
                        }

                        div { class: "mt-8 text-center",
                            p { class: "text-gray-400 text-sm", "Supported wallets: Phantom, Solflare, Backpack, Glow" }
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
