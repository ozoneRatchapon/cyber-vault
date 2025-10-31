// The dioxus prelude contains a ton of common items used in dioxus apps. It's a good idea to import wherever you
// need dioxus
use chrono::Utc;
use dioxus::document;
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Load the Tailwind CSS asset
static CSS: Asset = asset!("/assets/tailwind.css");

// Component imports
mod components;
mod vault_operations;
mod wallet;

use components::{CreateVaultForm, Header, VaultList};
use vault_operations::VaultOperations;
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
    // pub vault_ops: Option<VaultOperations>,
}

/// The main App component is the root of your application. Every component in Dioxus is a function
/// that takes some props and returns an Element. In this case, App takes no props because it is the root of our app.
///
/// Components should be annotated with `#[component]` to support props, better error messages, and autocomplete
#[component]
fn App() -> Element {
    // Cypherpunk theme - trust through code
    let _test_styles = "bg-black text-green-400 p-4 border border-green-400";

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
    let handle_create_vault = Callback::new({
        move |(beneficiary, period, amount, mint): (String, i64, u64, String)| {
            let mut state_clone = state.clone();
            let wallet_clone = wallet_provider.clone();

            spawn(async move {
                // Extract owner pubkey first to avoid borrowing issues
                let owner_pubkey = {
                    let state_read = state_clone.read();
                    match state_read.wallet.public_key {
                        Some(pk) => pk,
                        None => {
                            drop(state_read);
                            state_clone.write().error = Some("Wallet not connected".to_string());
                            state_clone.write().is_loading = false;
                            return;
                        }
                    }
                };

                // Validate inputs
                let beneficiary_pubkey = match Pubkey::from_str(&beneficiary) {
                    Ok(pk) => pk,
                    Err(_) => {
                        state_clone.write().error = Some("Invalid beneficiary address".to_string());
                        state_clone.write().is_loading = false;
                        return;
                    }
                };

                let mint_pubkey = match Pubkey::from_str(&mint) {
                    Ok(pk) => pk,
                    Err(_) => {
                        state_clone.write().error = Some("Invalid token mint address".to_string());
                        state_clone.write().is_loading = false;
                        return;
                    }
                };

                // Set loading state after validation
                state_clone.write().is_loading = true;
                state_clone.write().error = None;
                state_clone.write().success = None;

                // Create VaultOperations instance
                let vault_ops = match VaultOperations::new(wallet_clone.read().clone()) {
                    Ok(ops) => ops,
                    Err(e) => {
                        state_clone.write().error =
                            Some(format!("Failed to initialize vault operations: {}", e));
                        state_clone.write().is_loading = false;
                        return;
                    }
                };

                // Create vault instruction
                let instruction = match vault_ops.create_vault_instruction(
                    &owner_pubkey,
                    &beneficiary_pubkey,
                    &mint_pubkey,
                    period,
                    amount,
                ) {
                    Ok(inst) => inst,
                    Err(e) => {
                        state_clone.write().error =
                            Some(format!("Failed to create vault instruction: {}", e));
                        state_clone.write().is_loading = false;
                        return;
                    }
                };

                // Create and send transaction
                let transaction = match vault_ops
                    .create_and_sign_transaction(vec![instruction], &owner_pubkey)
                    .await
                {
                    Ok(tx) => tx,
                    Err(e) => {
                        state_clone.write().error =
                            Some(format!("Failed to create transaction: {}", e));
                        state_clone.write().is_loading = false;
                        return;
                    }
                };

                match vault_ops.send_transaction(transaction).await {
                    Ok(signature) => {
                        state_clone.write().success = Some(format!(
                            "🏦 Vault created successfully! Signature: {}",
                            signature
                        ));

                        // Add the new vault to the list (mock data for now)
                        let new_vault = VaultInfo {
                            pubkey: format!("vault_{}", &signature[..8]),
                            owner: owner_pubkey.to_string(),
                            beneficiary: beneficiary_pubkey.to_string(),
                            token_mint: mint_pubkey.to_string(),
                            balance: amount,
                            inactivity_period: period,
                            last_heartbeat: Utc::now().timestamp(),
                        };
                        state_clone.write().vaults.push(new_vault);
                    }
                    Err(e) => {
                        state_clone.write().error =
                            Some(format!("Failed to send transaction: {}", e));
                    }
                }

                state_clone.write().is_loading = false;
            });
        }
    });

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

        div { class: "min-h-screen bg-black text-gray-300 font-mono",
            Header {
                wallet_connected: state.read().wallet.connected,
                public_key: state.read().wallet.public_key,
                on_connect: handle_wallet_connect,
                on_disconnect: handle_wallet_disconnect,
            }

            main { class: "container mx-auto px-4 py-8",
                // Loading indicator
                if state.read().is_loading {
                    div { class: "cypher-card mb-6 bg-transparent border-l-4 border-green-400",
                        div { class: "flex items-center space-x-3",
                            div { class: "cypher-loading" }
                            span { class: "text-green-400", "PROCESSING..." }
                        }
                    }
                }

                // Success notifications
                if let Some(success) = &state.read().success {
                    div { class: "cypher-card mb-6 bg-transparent border-l-4 border-green-400",
                        div { class: "flex justify-between items-center",
                            div { class: "flex items-center space-x-3",
                                span { class: "text-green-400", "[+]" }
                                span { class: "text-green-400", "{success}" }
                            }
                            button {
                                class: "cypher-button secondary px-3 py-1 text-xs border-green-400 text-green-400 hover:bg-green-400 hover:text-black",
                                onclick: move |_| state.write().success = None,
                                "×"
                            }
                        }
                    }
                }

                // Error notifications
                if let Some(error) = &state.read().error {
                    div { class: "cypher-card mb-6 bg-transparent border-l-4 border-gray-400",
                        div { class: "flex justify-between items-center",
                            div { class: "flex items-center space-x-3",
                                span { class: "text-gray-400", "[!]" }
                                span { class: "text-gray-400", "{error}" }
                            }
                            button {
                                class: "cypher-button secondary px-3 py-1 text-xs border-gray-400 text-gray-400 hover:bg-gray-400 hover:text-black",
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
                        div { class: "cypher-card",
                            h2 {
                                class: "text-xl mb-4 text-green-400 font-semibold",
                                "[AUTH] WALLET_CONNECTED"
                            }
                            if let Some(pubkey) = &state.read().wallet.public_key {
                                div { class: "space-y-2",
                                    p { class: "text-gray-300", "PUBLIC_KEY:" }
                                    p { class: "font-mono text-sm break-all text-green-400 address-display",
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
                            div { class: "cypher-card",
                                h3 {
                                    class: "text-lg mb-6 text-green-400 font-semibold",
                                    "[VAULT] ACTIONS"
                                }

                                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4",
                                    button {
                                        class: "cypher-button",
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
                                        "> EXECUTE HEARTBEAT"
                                    }

                                    button {
                                        class: "cypher-button",
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
                                        "> CLAIM FUNDS"
                                    }

                                    button {
                                        class: "cypher-button danger",
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
                                        "> EMERGENCY WITHDRAW"
                                    }

                                    button {
                                        class: "cypher-button secondary",
                                        onclick: move |_| state.write().selected_vault = None,
                                        "> CLOSE"
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
                                class: "text-4xl font-bold text-green-400 tracking-wider",
                                "CYPHER VAULT"
                            }
                            p {
                                class: "text-lg text-gray-400 leading-relaxed font-mono",
                                "Decentralized dead man's switch. Trust through code, not corporations."
                            }
                            div { class: "flex justify-center space-x-4",
                                span { class: "status-badge success",
                                    "SECURE"
                                }
                                span { class: "status-badge success",
                                    "DECENTRALIZED"
                                }
                                span { class: "status-badge success",
                                    "TRUSTLESS"
                                }
                            }
                        }

                        div { class: "grid grid-cols-1 md:grid-cols-3 gap-8 max-w-5xl mx-auto",
                            div { class: "cypher-card hover:border-green-400 transition-all duration-300",
                                h3 { class: "text-lg font-bold mb-3 text-green-400", "[01] CREATE_VAULT" }
                                p { class: "text-gray-400 font-mono text-sm", "> Choose beneficiary and deposit tokens" }
                            }
                            div { class: "cypher-card hover:border-green-400 transition-all duration-300",
                                h3 { class: "text-lg font-bold mb-3 text-green-400", "[02] SEND_HEARTBEATS" }
                                p { class: "text-gray-400 font-mono text-sm", "> Regular signals maintain vault active state" }
                            }
                            div { class: "cypher-card hover:border-green-400 transition-all duration-300",
                                h3 { class: "text-lg font-bold mb-3 text-green-400", "[03] AUTO_PROTECTION" }
                                p { class: "text-gray-400 font-mono text-sm", "> Beneficiary claims on inactivity" }
                            }
                        }

                        div { class: "space-y-6 max-w-md mx-auto",
                            button {
                                class: "cypher-button w-full py-4 text-lg font-semibold border-green-400 text-green-400 hover:bg-green-400 hover:text-black",
                                onclick: handle_wallet_connect,
                                "> INITIALIZE CONNECTION"
                            }

                            div { class: "cypher-card border-gray-600",
                                div { class: "flex items-center justify-center space-x-6 text-sm text-gray-400 font-mono",
                                    span { "PHANTOM" }
                                    span { "SOLFLARE" }
                                    span { "BACKPACK" }
                                }
                            }
                        }

                        div { class: "cypher-card max-w-2xl mx-auto border-gray-600",
                            h3 { class: "text-lg font-semibold mb-3 text-green-400", "[SYSTEM] SPECIFICATION" }
                            div { class: "grid grid-cols-1 md:grid-cols-2 gap-4 text-sm",
                                div { class: "flex items-start space-x-2",
                                    span { class: "text-green-400", "[+]" }
                                    span { class: "text-gray-300", "Emergency withdrawal protocol" }
                                }
                                div { class: "flex items-start space-x-2",
                                    span { class: "text-green-400", "[+]" }
                                    span { class: "text-gray-300", "SOL, USDC, USDT supported" }
                                }
                                div { class: "flex items-start space-x-2",
                                    span { class: "text-green-400", "[+]" }
                                    span { class: "text-gray-300", "Customizable inactivity timer" }
                                }
                                div { class: "flex items-start space-x-2",
                                    span { class: "text-green-400", "[+]" }
                                    span { class: "text-gray-300", "Zero middlemen, zero fees" }
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
