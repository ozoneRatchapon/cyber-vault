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
    let mut token_mint = use_signal(|| "So11111111111111111111111111111111111111112".to_string()); // WSOL by default
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

        // Validate token mint
        let mint_str = token_mint.read().clone();
        if Pubkey::from_str(&mint_str).is_err() {
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
        let mint_clone = mint_str.clone();

        on_create_vault.call((
            beneficiary_clone,
            days as i64 * 86400,
            amount_value,
            mint_clone,
        ));

        // Reset form after a delay
        spawn(async move {
            gloo_timers::future::sleep(std::time::Duration::from_millis(2000)).await;
            beneficiary.set(String::new());
            amount.set(String::new());
            inactivity_days.set(30);
            is_creating.set(false);
        });
    };

    rsx! {
        div { class: "bg-gray-800 bg-opacity-90 backdrop-blur-sm rounded-xl p-6 shadow-xl border border-gray-700",
            div { class: "flex items-center space-x-3 mb-6",
                div { class: "text-3xl", "🏦" }
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-blue-400 to-purple-600 bg-clip-text text-transparent", "Create New Vault" }
            }

            if !form_error.read().is_empty() {
                div { class: "bg-red-500 bg-opacity-20 border border-red-500 text-red-200 p-4 rounded-lg mb-4 animate-pulse",
                    div { class: "flex items-center space-x-2",
                        span { class: "text-xl", "⚠️" }
                        span { "{form_error.read()}" }
                    }
                }
            }

            div { class: "space-y-6",
                // Beneficiary Address
                div {
                    label { class: "block text-sm font-semibold text-gray-300 mb-2 flex items-center space-x-2",
                        span { "👤" }
                        span { "Beneficiary Address" }
                    }
                    div { class: "relative",
                        input {
                            r#type: "text",
                            class: "w-full px-4 py-3 bg-gray-700 bg-opacity-50 border border-gray-600 rounded-lg focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 text-white placeholder-gray-400 transition-all duration-200",
                            placeholder: "Enter beneficiary public key",
                            value: "{beneficiary}",
                            oninput: move |e| beneficiary.set(e.value()),
                            disabled: *is_creating.read()
                        }
                        if !beneficiary.read().is_empty() {
                            div { class: "absolute right-3 top-3 text-green-400", "✓" }
                        }
                    }
                }

                // Token Mint
                div {
                    label { class: "block text-sm font-semibold text-gray-300 mb-2 flex items-center space-x-2",
                        span { "🪙" }
                        span { "Token Mint Address" }
                    }
                    div { class: "relative",
                        input {
                            r#type: "text",
                            class: "w-full px-4 py-3 bg-gray-700 bg-opacity-50 border border-gray-600 rounded-lg focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 text-white placeholder-gray-400 transition-all duration-200",
                            placeholder: "Token mint address",
                            value: "{token_mint}",
                            oninput: move |e| token_mint.set(e.value()),
                            disabled: *is_creating.read()
                        }
                    }
                    div { class: "flex items-center space-x-2 mt-2",
                        div { class: "bg-blue-500 bg-opacity-20 text-blue-300 px-2 py-1 rounded text-xs font-medium", "WSOL" }
                        p { class: "text-xs text-gray-400",
                            "Default: Wrapped SOL"
                        }
                    }
                }

                // Inactivity Period
                div {
                    label { class: "block text-sm font-semibold text-gray-300 mb-2 flex items-center space-x-2",
                        span { "⏰" }
                        span { "Inactivity Period: {inactivity_days.read()} days" }
                    }
                    div { class: "space-y-3",
                        input {
                            r#type: "range",
                            min: "1",
                            max: "365",
                            value: "{inactivity_days}",
                            class: "w-full h-2 bg-gray-600 rounded-lg appearance-none cursor-pointer slider",
                            oninput: move |e| inactivity_days.set(e.value().parse().unwrap_or(30)),
                            disabled: *is_creating.read()
                        }
                        div { class: "flex justify-between text-xs text-gray-400",
                            span { "1d" }
                            span { "30d" }
                            span { "90d" }
                            span { "180d" }
                            span { "365d" }
                        }
                        div { class: "bg-gray-700 bg-opacity-50 rounded-lg p-3 text-center",
                            span { class: "text-lg font-bold text-blue-400", "{inactivity_days.read()}" }
                            span { class: "text-sm text-gray-400 ml-1", "days" }
                        }
                    }
                }

                // Amount
                div {
                    label { class: "block text-sm font-semibold text-gray-300 mb-2 flex items-center space-x-2",
                        span { "💰" }
                        span { "Amount to Deposit" }
                    }
                    div { class: "relative",
                        input {
                            r#type: "number",
                            class: "w-full px-4 py-3 bg-gray-700 bg-opacity-50 border border-gray-600 rounded-lg focus:outline-none focus:border-blue-500 focus:ring-2 focus:ring-blue-500 focus:ring-opacity-50 text-white placeholder-gray-400 transition-all duration-200",
                            placeholder: "Enter amount",
                            value: "{amount}",
                            oninput: move |e| amount.set(e.value()),
                            disabled: *is_creating.read()
                        }
                        if !amount.read().is_empty() {
                            div { class: "absolute right-3 top-3 text-green-400", "✓" }
                        }
                    }
                }

                // Submit Button
                button {
                    class: "w-full bg-gradient-to-r from-blue-600 to-purple-600 hover:from-blue-700 hover:to-purple-700 active:from-blue-800 active:to-purple-800 disabled:from-gray-600 disabled:to-gray-600 disabled:cursor-not-allowed px-6 py-4 rounded-lg transition-all duration-200 transform hover:scale-105 font-semibold text-lg shadow-lg disabled:shadow-none",
                    onclick: handle_submit,
                    disabled: *is_creating.read() || beneficiary.read().is_empty() || amount.read().is_empty(),

                    if *is_creating.read() {
                        div { class: "flex items-center justify-center space-x-3",
                            div { class: "animate-spin rounded-full h-5 w-5 border-b-2 border-white" }
                            span { "Creating Vault..." }
                        }
                    } else {
                        div { class: "flex items-center justify-center space-x-2",
                            span { "🏦" }
                            span { "Create Vault" }
                        }
                    }
                }
            }

            // Info Box
            div { class: "mt-6 p-4 bg-gray-700 bg-opacity-50 rounded-lg border border-gray-600",
                h3 { class: "text-sm font-semibold text-yellow-400 mb-3 flex items-center space-x-2",
                    span { "⚠️" }
                    span { "Important Information" }
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                    div { class: "flex items-start space-x-2",
                        span { class: "text-green-400 text-xs mt-0.5", "✓" }
                        span { class: "text-xs text-gray-300", "Beneficiary can claim after inactivity period" }
                    }
                    div { class: "flex items-start space-x-2",
                        span { class: "text-blue-400 text-xs mt-0.5", "💓" }
                        span { class: "text-xs text-gray-300", "Send heartbeats to reset timer" }
                    }
                    div { class: "flex items-start space-x-2",
                        span { class: "text-red-400 text-xs mt-0.5", "🚨" }
                        span { class: "text-xs text-gray-300", "Emergency withdraw anytime" }
                    }
                    div { class: "flex items-start space-x-2",
                        span { class: "text-purple-400 text-xs mt-0.5", "🔒" }
                        span { class: "text-xs text-gray-300", "Minimum 1 day inactivity" }
                    }
                }
            }
        }
    }
}
