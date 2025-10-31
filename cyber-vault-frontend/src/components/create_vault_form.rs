use crate::components::token_selector::{TokenInfo, TokenSelector};
use dioxus::prelude::*;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[component]
pub fn CreateVaultForm(
    public_key: Option<Pubkey>,
    on_create_vault: EventHandler<(String, i64, u64, String)>,
) -> Element {
    let mut beneficiary = use_signal(|| String::new());
    let mut inactivity_days = use_signal(|| 30);
    let mut amount = use_signal(|| String::new());
    let mut selected_token = use_signal(|| Some(TokenInfo::wrapped_sol()));
    let mut form_error = use_signal(|| String::new());
    let mut is_creating = use_signal(|| false);

    let handle_submit = move |_| {
        form_error.set(String::new());

        // Validate beneficiary
        let beneficiary_str = beneficiary.read().clone();
        if beneficiary_str.is_empty() {
            form_error.set("Beneficiary address is required".to_string());
            return;
        }

        if Pubkey::from_str(&beneficiary_str).is_err() {
            form_error.set("Invalid beneficiary address".to_string());
            return;
        }

        // Validate amount
        let amount_str = amount.read().clone();
        if amount_str.is_empty() {
            form_error.set("Amount is required".to_string());
            return;
        }

        let amount_value = match amount_str.parse::<u64>() {
            Ok(val) => val,
            Err(_) => {
                form_error.set("Invalid amount".to_string());
                return;
            }
        };

        if amount_value == 0 {
            form_error.set("Amount must be greater than 0".to_string());
            return;
        }

        // Validate selected token
        let token_info = match selected_token.read().as_ref() {
            Some(token) => token.clone(),
            None => {
                form_error.set("Please select a token".to_string());
                return;
            }
        };

        if Pubkey::from_str(&token_info.mint).is_err() {
            form_error.set("Invalid token mint address".to_string());
            return;
        }

        // Validate inactivity period
        let days = *inactivity_days.read();
        if days < 1 {
            form_error.set("Inactivity period must be at least 1 day".to_string());
            return;
        }

        // Check if beneficiary is not self
        if let Some(current_pubkey) = public_key {
            if beneficiary_str == current_pubkey.to_string() {
                form_error.set("Cannot set yourself as the beneficiary".to_string());
                return;
            }
        }

        is_creating.set(true);
        let beneficiary_clone = beneficiary_str.clone();
        let token_info_clone = token_info.clone();

        on_create_vault.call((
            beneficiary_clone,
            days as i64 * 86400,
            amount_value,
            token_info_clone.mint,
        ));

        // Reset form after a delay
        spawn(async move {
            gloo_timers::future::sleep(std::time::Duration::from_millis(2000)).await;
            beneficiary.set(String::new());
            amount.set(String::new());
            inactivity_days.set(30);
            selected_token.set(Some(TokenInfo::wrapped_sol()));
            is_creating.set(false);
        });
    };

    rsx! {
        div { class: "cypher-card",
            div { class: "flex items-center space-x-3 mb-6 border-b border-gray-800 pb-4",
                div { class: "text-2xl text-green-400 font-mono", "[CREATE]" }
                h2 { class: "text-xl font-semibold text-white font-mono tracking-wider", "VAULT_PROTOCOL" }
            }

            if !form_error.read().is_empty() {
                div { class: "cypher-card mb-4 bg-transparent border-l-4 border-gray-400",
                    div { class: "flex items-center space-x-2",
                        span { class: "text-sm text-gray-400 font-mono", "[ERROR]" }
                        span { class: "text-gray-400 font-mono text-sm", "{form_error.read()}" }
                    }
                }
            }

            div { class: "space-y-6",
                // Beneficiary Address
                div {
                    label { class: "form-label flex items-center space-x-2 text-gray-300",
                        span { class: "text-green-400", "[BENE]" }
                        span { "BENEFICIARY_ADDRESS" }
                    }
                    div { class: "relative",
                        input {
                            r#type: "text",
                            class: "cypher-input border-b border-green-400 bg-transparent",
                            placeholder: "0x...",
                            value: "{beneficiary}",
                            oninput: move |e| beneficiary.set(e.value()),
                            disabled: *is_creating.read()
                        }
                        if !beneficiary.read().is_empty() {
                            div { class: "absolute right-3 top-3 text-green-400 font-mono text-xs", "[+]" }
                        }
                    }
                }

                // Token Selection
                div {
                    label { class: "form-label flex items-center space-x-2 text-gray-300",
                        span { class: "text-green-400", "[TOKEN]" }
                        span { "SELECT_TOKEN" }
                    }
                    TokenSelector {
                        selected_token: selected_token.read().clone(),
                        on_token_select: Callback::new(move |token: TokenInfo| {
                            selected_token.set(Some(token));
                        }),
                        disabled: *is_creating.read()
                    }
                    div { class: "mt-2 text-xs text-gray-500 font-mono",
                        "> Choose token for vault deposit"
                    }
                }

                // Inactivity Period
                div {
                    label { class: "form-label flex items-center space-x-2 text-gray-300",
                        span { class: "text-green-400", "[TIME]" }
                        span { "INACTIVITY_PERIOD: {inactivity_days.read()}_DAYS" }
                    }
                    div { class: "space-y-4",
                        input {
                            r#type: "range",
                            min: "1",
                            max: "365",
                            value: "{inactivity_days}",
                            class: "w-full h-2 bg-black appearance-none cursor-pointer border border-gray-700",
                            oninput: move |e| inactivity_days.set(e.value().parse().unwrap_or(30)),
                            disabled: *is_creating.read()
                        }
                        div { class: "flex justify-between text-xs text-gray-500 font-mono",
                            span { "1D" }
                            span { "30D" }
                            span { "90D" }
                            span { "180D" }
                            span { "365D" }
                        }
                        div { class: "cypher-card text-center bg-black border border-gray-800",
                            span { class: "text-2xl font-semibold text-green-400 font-mono", "{inactivity_days.read()}" }
                            span { class: "text-sm text-gray-400 ml-2 font-mono", "DAYS" }
                        }
                    }
                }

                // Amount
                div {
                    label { class: "form-label flex items-center space-x-2 text-gray-300",
                        span { class: "text-green-400", "[AMNT]" }
                        span { "DEPOSIT_AMOUNT" }
                        if let Some(token) = selected_token.read().as_ref() {
                            span { class: "text-sm text-green-400 ml-2 font-mono", "({token.symbol})" }
                        }
                    }
                    div { class: "relative",
                        input {
                            r#type: "number",
                            class: "cypher-input border-b border-green-400 bg-transparent",
                            placeholder: "0.00",
                            value: "{amount}",
                            oninput: move |e| amount.set(e.value()),
                            disabled: *is_creating.read()
                        }
                        if !amount.read().is_empty() {
                            div { class: "absolute right-3 top-3 text-green-400 font-mono text-xs", "[+]" }
                        }
                    }
                }

                // Submit Button
                button {
                    class: "cypher-button w-full py-4 text-base font-medium disabled:opacity-30 disabled:cursor-not-allowed border-green-400 text-green-400 hover:bg-green-400 hover:text-black font-mono uppercase tracking-wider",
                    onclick: handle_submit,
                    disabled: *is_creating.read() || beneficiary.read().is_empty() || amount.read().is_empty(),

                    if *is_creating.read() {
                        div { class: "flex items-center justify-center space-x-2",
                            div { class: "cypher-loading" }
                            span { class: "font-mono", "INITIALIZING_VAULT..." }
                        }
                    } else {
                        div { class: "flex items-center justify-center space-x-2",
                            span { "[CREATE]" }
                            span { "VAULT" }
                        }
                    }
                }
            }

            // Protocol Information
            div { class: "cypher-card mt-6 bg-black border border-gray-800",
                h3 { class: "text-sm font-semibold mb-3 text-green-400 flex items-center space-x-2 font-mono uppercase tracking-wider",
                    span { "[PROTOCOL]" }
                    span { "SPECIFICATION" }
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                    div { class: "flex items-start space-x-2",
                        span { class: "text-green-400 text-sm mt-0.5 font-mono", "[+]" }
                        span { class: "text-xs text-gray-400 font-mono", "Beneficiary claims after inactivity" }
                    }
                    div { class: "flex items-start space-x-2",
                        span { class: "text-green-400 text-sm mt-0.5 font-mono", "[+]" }
                        span { class: "text-xs text-gray-400 font-mono", "Send heartbeats to reset timer" }
                    }
                    div { class: "flex items-start space-x-2",
                        span { class: "text-green-400 text-sm mt-0.5 font-mono", "[+]" }
                        span { class: "text-xs text-gray-400 font-mono", "Emergency withdrawal available" }
                    }
                    div { class: "flex items-start space-x-2",
                        span { class: "text-gray-400 text-xs mt-0.5 font-mono", "[!]" }
                        span { class: "text-xs text-gray-400 font-mono", "Minimum 1 day inactivity" }
                    }
                }
            }
        }
    }
}
