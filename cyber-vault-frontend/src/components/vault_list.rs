use dioxus::prelude::*;
use crate::components::token_selector::{TokenInfo, get_common_tokens};

#[component]
pub fn VaultList(
    vaults: Vec<crate::VaultInfo>,
    selected_vault: Option<crate::VaultInfo>,
    on_select: EventHandler<crate::VaultInfo>,
) -> Element {
    rsx! {
        div { class: "cyber-card bg-[#141925]",
            div { class: "flex items-center justify-between mb-6",
                h2 { class: "text-xl font-semibold text-gray-200", "Your Vaults" }
                div { class: "bg-green-400 bg-opacity-10 text-green-400 px-3 py-1 text-xs font-semibold uppercase tracking-wider border border-green-400 border-opacity-30", "{vaults.len()} total" }
            }

            if vaults.is_empty() {
                div { class: "text-center py-12",
                    div { class: "text-5xl mb-4 text-cyan-300", "🏦" }
                    h3 { class: "text-lg font-medium mb-2 text-gray-200", "No Vaults Found" }
                    p { class: "text-gray-400", "Create your first vault to get started" }
                    div { class: "mt-6 text-sm text-gray-400",
                        "📝 Use form below to create your first vault"
                    }
                }
            } else {
                div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4",
                    {vaults.iter().enumerate().map(|(index, vault)| {
                        let is_selected = selected_vault.as_ref().map_or(false, |v| v.pubkey == vault.pubkey);
                        let vault_clone = vault.clone();

                        // Find token info for this mint
                        let tokens = get_common_tokens();
                        let token_info = tokens.iter().find(|t| t.mint == vault.token_mint);
                        let token_icon = token_info.map(|t| t.icon.as_str()).unwrap_or("🪙");
                        let token_symbol = token_info.map(|t| t.symbol.as_str()).unwrap_or("UNKNOWN");

                        // Calculate display balance based on token decimals
                        let display_balance = if let Some(token) = token_info {
                            vault.balance as f64 / 10_f64.powi(token.decimals as i32)
                        } else {
                            vault.balance as f64 / 1_000_000.0 // Default to 6 decimals
                        };

                        // Format beneficiary address
                        let beneficiary_short = if vault.beneficiary.len() > 8 {
                            format!("{}...{}", &vault.beneficiary[..4], &vault.beneficiary[vault.beneficiary.len()-4..])
                        } else {
                            vault.beneficiary.clone()
                        };

                        rsx! {
                            div {
                                class: if is_selected {
                                    "cyber-card bg-[#1e2433] border-cyan-400 cursor-pointer"
                                } else {
                                    "cyber-card bg-[#141925] border-[#2a3441] hover:border-cyan-400 hover:transform hover:-translate-y-1 cursor-pointer"
                                },
                                onclick: move |_| on_select.call(vault_clone.clone()),

                                div { class: "flex items-start justify-between mb-3",
                                    div { class: "flex items-center space-x-2",
                                        span { class: "text-2xl", "{token_icon}" }
                                        span { class: "text-lg font-bold text-cyan-300", "Vault #{index + 1}" }
                                    }
                                    if is_selected {
                                        span { class: "text-green-400 text-sm", "● SELECTED" }
                                    }
                                }

                                div { class: "space-y-2",
                                    div { class: "flex items-center justify-between",
                                        span { class: "text-sm text-gray-400", "Balance:" }
                                        span { class: "text-lg font-semibold text-gray-200",
                                            "{display_balance:.4} {token_symbol}"
                                        }
                                    }

                                    div { class: "flex items-center justify-between",
                                        span { class: "text-sm text-gray-400", "Beneficiary:" }
                                        span { class: "text-xs text-pink-500 font-mono", "{beneficiary_short}" }
                                    }

                                    div { class: "flex items-center justify-between",
                                        span { class: "text-sm text-gray-400", "Token:" }
                                        span { class: "text-sm text-gray-300", "{token_symbol}" }
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}
