use dioxus::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TokenInfo {
    pub symbol: String,
    pub name: String,
    pub mint: String,
    pub decimals: u8,
    pub icon: String,
}

impl TokenInfo {
    pub fn wrapped_sol() -> Self {
        Self {
            symbol: "SOL".to_string(),
            name: "Solana".to_string(),
            mint: "So11111111111111111111111111111111111111112".to_string(),
            decimals: 9,
            icon: "◎".to_string(),
        }
    }

    pub fn usdc() -> Self {
        Self {
            symbol: "USDC".to_string(),
            name: "USD Coin".to_string(),
            mint: "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string(),
            decimals: 6,
            icon: "$".to_string(),
        }
    }

    pub fn usdt() -> Self {
        Self {
            symbol: "USDT".to_string(),
            name: "Tether USD".to_string(),
            mint: "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB".to_string(),
            decimals: 6,
            icon: "₮".to_string(),
        }
    }
}

pub fn get_common_tokens() -> Vec<TokenInfo> {
    vec![
        TokenInfo::wrapped_sol(),
        TokenInfo::usdc(),
        TokenInfo::usdt(),
    ]
}

#[component]
pub fn TokenSelector(
    selected_token: Option<TokenInfo>,
    on_token_select: EventHandler<TokenInfo>,
    disabled: bool,
) -> Element {
    let mut is_open = use_signal(|| false);
    let tokens = get_common_tokens();
    let selected_display = selected_token
        .as_ref()
        .map_or("Select token".to_string(), |t| {
            format!("{} {}", t.icon, t.symbol)
        });

    rsx! {
        div { class: "relative",
            button {
                class: "cyber-input w-full text-left flex items-center justify-between bg-[#1e2433] border-gray-600 hover:border-cyan-400",
                onclick: move |_| {
                    if !disabled {
                        let current = *is_open.read();
                        is_open.set(!current);
                    }
                },
                disabled: disabled,

                div { class: "flex items-center space-x-3",
                    if let Some(token) = &selected_token {
                        span { class: "text-2xl", "{token.icon}" }
                        span { class: "font-medium", "{token.symbol}" }
                        span { class: "text-xs text-gray-400", "{token.name}" }
                    } else {
                        span { class: "text-gray-400", "🪙 Select token..." }
                    }
                }

                span { class: "text-gray-400", "▼" }
            }

            if *is_open.read() {
                div { class: "absolute z-10 w-full mt-2 cyber-card bg-[#141925] border-cyan-400 max-h-60 overflow-y-auto",
                    div { class: "p-2 border-b border-gray-700",
                        p { class: "text-xs text-gray-400 font-medium", "POPULAR TOKENS" }
                    }

                    {tokens.iter().map(|token| {
                        let token_clone = token.clone();
                        let is_selected = selected_token.as_ref().map_or(false, |t| t.mint == token.mint);

                        rsx! {
                            button {
                                class: if is_selected {
                                    "w-full cyber-card bg-[#1e2433] border-cyan-400 text-left"
                                } else {
                                    "w-full cyber-card bg-[#141925] border-gray-600 hover:bg-[#1e2433] hover:border-cyan-400 text-left"
                                },
                                onclick: move |_| {
                                    on_token_select.call(token_clone.clone());
                                    is_open.set(false);
                                },

                                div { class: "flex items-center justify-between",
                                    div { class: "flex items-center space-x-3",
                                        span { class: "text-2xl", "{token.icon}" }
                                        div {
                                            div { class: "font-medium text-gray-200", "{token.symbol}" }
                                            div { class: "text-xs text-gray-400", "{token.name}" }
                                        }
                                    }
                                    if is_selected {
                                        span { class: "text-green-400", "✓" }
                                    }
                                }
                            }
                        }
                    })}

                    div { class: "p-2 border-t border-gray-700 mt-2",
                        button {
                            class: "w-full text-xs text-gray-400 hover:text-cyan-300 text-left",
                            onclick: move |_| {
                                // TODO: Implement custom token input
                                is_open.set(false);
                            },
                            "+ Add custom token"
                        }
                    }
                }
            }
        }
    }
}
