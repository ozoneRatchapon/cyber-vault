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
        div { class: "cyber-card bg-[#141925]",
            div { class: "flex items-center space-x-3 mb-6",
                div { class: "text-3xl text-cyan-300", "🏦" }
                h2 { class: "text-2xl font-semibold text-gray-200", "Create New Vault" }
            }

            if !form_error.read().is_empty() {
                div { class: "cyber-card mb-4 bg-transparent border-l-4 border-pink-500",
                    div { class: "flex items-center space-x-2",
                        span { class: "text-lg text-pink-500", "⚠️" }
                        span { class: "text-pink-500", "{form_error.read()}" }
                    }
                }
            }

            div { class: "space-y-6",
                // Beneficiary Address
                div {
                    label { class: "form-label flex items-center space-x-2 text-gray-200",
                        span { class: "text-lg", "👤" }
                        span { "Beneficiary Address" }
                    }
                    div { class: "relative",
                        input {
                            r#type: "text",
                            class: "cyber-input border-b border-cyan-400 bg-transparent",
                            placeholder: "Enter beneficiary public key...",
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
                    label { class: "form-label flex items-center space-x-2 text-gray-200",
                        span { class: "text-lg", "🪙" }
                        span { "Token Mint Address" }
                    }
                    div { class: "relative",
                        input {
                            r#type: "text",
                            class: "cyber-input border-b border-cyan-400 bg-transparent",
                            placeholder: "Token mint address...",
                            value: "{token_mint}",
                            oninput: move |e| token_mint.set(e.value()),
                            disabled: *is_creating.read()
                        }
                    }
                    div { class: "flex items-center space-x-3 mt-3",
                        div { class: "cyber-button px-3 py-1 text-xs border-cyan-400 text-cyan-300 hover:bg-cyan-400 hover:text-black", "WSOL" }
                        p { class: "text-xs text-gray-400",
                            "Default: Wrapped SOL"
                        }
                    }
                }

                // Inactivity Period
                div {
                    label { class: "form-label flex items-center space-x-2 text-gray-200",
                        span { class: "text-lg", "⏰" }
                        span { "Inactivity Period: {inactivity_days.read()} days" }
                    }
                    div { class: "space-y-4",
                        input {
                            r#type: "range",
                            min: "1",
                            max: "365",
                            value: "{inactivity_days}",
                            class: "w-full h-3 bg-[#1e2433] rounded-none appearance-none cursor-pointer border-cyan-400",
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
                        div { class: "cyber-card text-center bg-[#1e2433]",
                            span { class: "text-xl font-semibold text-cyan-300", "{inactivity_days.read()}" }
                            span { class: "text-sm text-gray-400 ml-1", "days" }
                        }
                    }
                }

                // Amount
                div {
                    label { class: "form-label flex items-center space-x-2 text-gray-200",
                        span { class: "text-lg", "💰" }
                        span { "Amount to Deposit" }
                    }
                    div { class: "relative",
                        input {
                            r#type: "number",
                            class: "cyber-input border-b border-cyan-400 bg-transparent",
                            placeholder: "Enter amount...",
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
                    class: "cyber-button w-full py-3 text-base font-medium disabled:opacity-50 disabled:cursor-not-allowed border-cyan-400 text-cyan-300 hover:bg-cyan-400 hover:text-black",
                    onclick: handle_submit,
                    disabled: *is_creating.read() || beneficiary.read().is_empty() || amount.read().is_empty(),

                    if *is_creating.read() {
                        div { class: "flex items-center justify-center space-x-2",
                            div { class: "cyber-loading" }
                            span { "Creating vault..." }
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
            div { class: "cyber-card mt-6 bg-[#141925]",
                h3 { class: "text-sm font-semibold mb-3 text-pink-500 flex items-center space-x-2",
                    span { "text-lg", "⚠️" }
                    span { "Important Information" }
                }
                div { class: "grid grid-cols-1 md:grid-cols-2 gap-3",
                    div { class: "flex items-start space-x-2",
                        span { class: "text-green-400 text-lg mt-0.5", "✓" }
                        span { class: "text-sm text-gray-400", "Beneficiary can claim after inactivity period" }
                    }
                    div { class: "flex items-start space-x-2",
                        span { class: "text-lg mt-0.5", "💓" }
                        span { class: "text-sm text-gray-400", "Send heartbeats to reset timer" }
                    }
                    div { class: "flex items-start space-x-2",
                        span { class: "text-lg mt-0.5", "🚨" }
                        span { class: "text-sm text-gray-400", "Emergency withdraw anytime" }
                    }
                    div { class: "flex items-start space-x-2",
                        span { class: "text-pink-500 text-xs mt-0.5", "🔒" }
                        span { class: "text-xs text-gray-400", "Minimum 1 day inactivity" }
                    }
                }
            }
        }
    }
}
