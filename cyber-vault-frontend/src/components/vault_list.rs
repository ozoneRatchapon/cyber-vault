use crate::components::token_selector::get_common_tokens;
use dioxus::prelude::*;

#[component]
pub fn VaultList(
    vaults: Vec<crate::VaultInfo>,
    selected_vault: Option<crate::VaultInfo>,
    on_select: EventHandler<crate::VaultInfo>,
) -> Element {
    rsx! {
        div { class: "cypher-card",
            div { class: "flex items-center justify-between mb-6 border-b border-gray-800 pb-4",
                h2 { class: "text-xl font-semibold text-white font-mono tracking-wider", "[VAULTS]" }
                div { class: "status-badge success", "TOTAL: {vaults.len()}" }
            }

            if vaults.is_empty() {
                div { class: "text-center py-12",
                    div { class: "text-4xl mb-4 text-gray-500 font-mono", "[EMPTY]" }
                    h3 { class: "text-lg font-medium mb-2 text-gray-400 font-mono", "NO_VAULTS_FOUND" }
                    p { class: "text-gray-500 font-mono text-sm", "> Create your first vault to begin protocol" }
                    div { class: "mt-6 text-sm text-gray-600 font-mono",
                        "$ vault create --beneficiary <address> --amount <amount>"
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
                        let token_symbol = token_info.map(|t| t.symbol.as_str()).unwrap_or("UNKNOWN");

                        // Calculate display balance based on token decimals
                        let display_balance = if let Some(token) = token_info {
                            vault.balance as f64 / 10_f64.powi(token.decimals as i32)
                        } else {
                            vault.balance as f64 / 1_000_000.0 // Default to 6 decimals
                        };

                        // Format beneficiary address - terminal style
                        let beneficiary_short = if vault.beneficiary.len() > 16 {
                            format!("{}...{}", &vault.beneficiary[..8], &vault.beneficiary[vault.beneficiary.len()-8..])
                        } else {
                            vault.beneficiary.clone()
                        };

                        rsx! {
                            div {
                                class: if is_selected {
                                    "cypher-card border-green-400 cursor-pointer bg-black"
                                } else {
                                    "cypher-card border-gray-800 hover:border-green-400 cursor-pointer bg-black"
                                },
                                onclick: move |_| on_select.call(vault_clone.clone()),

                                // Vault Header
                                div { class: "flex items-start justify-between mb-4 border-b border-gray-900 pb-3",
                                    div { class: "flex items-center space-x-3",
                                        span { class: "text-green-400 font-mono text-xs", "[VAULT_{index + 1:02}]" }
                                        span { class: "text-xs text-gray-600 font-mono", "ID: {&vault.pubkey[..8]}..." }
                                    }
                                    if is_selected {
                                        span { class: "status-badge success", "SELECTED" }
                                    }
                                }

                                // Vault Data
                                div { class: "space-y-3 font-mono text-sm",
                                    div { class: "flex items-center justify-between",
                                        span { class: "text-gray-500 uppercase text-xs", "balance:" }
                                        span { class: "text-green-400 font-semibold",
                                            "{display_balance:.6} {token_symbol}"
                                        }
                                    }

                                    div { class: "flex items-center justify-between",
                                        span { class: "text-gray-500 uppercase text-xs", "beneficiary:" }
                                        span { class: "text-xs text-gray-400 font-mono", "{beneficiary_short}" }
                                    }

                                    div { class: "flex items-center justify-between",
                                        span { class: "text-gray-500 uppercase text-xs", "token:" }
                                        span { class: "text-xs text-gray-400 font-mono", "{token_symbol}" }
                                    }

                                    div { class: "flex items-center justify-between",
                                        span { class: "text-gray-500 uppercase text-xs", "inactivity:" }
                                        span { class: "text-xs text-gray-400 font-mono", "{vault.inactivity_period / 86400} days" }
                                    }

                                    div { class: "flex items-center justify-between",
                                        span { class: "text-gray-500 uppercase text-xs", "last_hb:" }
                                        span { class: "text-xs text-gray-400 font-mono",
                                            if vault.last_heartbeat > 0 {
                                                "active"
                                            } else {
                                                "never"
                                            }
                                        }
                                    }
                                }

                                // Action Status Bar
                                div { class: "mt-4 pt-3 border-t border-gray-900 flex justify-between items-center",
                                    span { class: "text-xs text-gray-600 font-mono", "status: active" }
                                    if is_selected {
                                        span { class: "text-xs text-green-400 font-mono", "> ready for commands" }
                                    } else {
                                        span { class: "text-xs text-gray-600 font-mono", "> click to select" }
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
