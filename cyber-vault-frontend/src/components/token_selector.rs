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
        .map_or("SELECT_TOKEN".to_string(), |t| {
            format!("{} {}", t.symbol, t.mint[..8].to_string())
        });

    rsx! {
        div { class: "relative",
            button {
                class: "cypher-input w-full text-left flex items-center justify-between bg-black border-gray-800 hover:border-green-400 font-mono",
                onclick: move |_| {
                    if !disabled {
                        let current = *is_open.read();
                        is_open.set(!current);
                    }
                },
                disabled: disabled,

                div { class: "flex items-center space-x-3 flex-1",
                    if let Some(token) = &selected_token {
                        span { class: "text-green-400 text-lg", "{token.icon}" }
                        span { class: "font-mono text-green-400", "{token.symbol}" }
                        span { class: "text-xs text-gray-600 font-mono", "0x{&token.mint[..8]}..." }
                    } else {
                        span { class: "text-gray-600 font-mono", "> SELECT_TOKEN" }
                    }
                }

                span { class: "text-gray-600 font-mono text-xs", "[▼]" }
            }

            if *is_open.read() {
                div { class: "absolute z-10 w-full mt-2 cypher-card bg-black border border-green-400 max-h-60 overflow-y-auto",
                    div { class: "px-3 py-2 border-b border-gray-800 bg-black",
                        p { class: "text-xs text-green-400 font-mono uppercase tracking-wider", "[TOKEN_REGISTRY]" }
                    }

                    {tokens.iter().enumerate().map(|(index, token)| {
                        let token_clone = token.clone();
                        let is_selected = selected_token.as_ref().map_or(false, |t| t.mint == token.mint);

                        rsx! {
                            button {
                                class: if is_selected {
                                    "w-full cypher-card border-green-400 bg-black text-left border-l-4"
                                } else {
                                    "w-full cypher-card border-gray-800 hover:border-green-400 hover:bg-black text-left border-l-4 border-l-transparent"
                                },
                                onclick: move |_| {
                                    on_token_select.call(token_clone.clone());
                                    is_open.set(false);
                                },

                                div { class: "flex items-center justify-between p-3",
                                    div { class: "flex items-center space-x-3 flex-1",
                                        span { class: "text-green-400 text-lg", "{token.icon}" }
                                        div { class: "flex-1",
                                            div { class: "font-mono text-white text-sm", "{token.symbol}" }
                                            div { class: "text-xs text-gray-600 font-mono", "{token.name}" }
                                            div { class: "text-xs text-gray-700 font-mono mt-1", "0x{&token.mint[..16]}..." }
                                        }
                                    }
                                    if is_selected {
                                        span { class: "text-green-400 font-mono text-xs", "[SELECTED]" }
                                    } else {
                                        span { class: "text-gray-600 font-mono text-xs", "[{index + 1:02}]" }
                                    }
                                }
                            }
                        }
                    })}

                    div { class: "px-3 py-2 border-t border-gray-800 bg-black mt-2",
                        button {
                            class: "w-full text-xs text-gray-600 hover:text-green-400 text-left font-mono",
                            onclick: move |_| {
                                // TODO: Implement custom token input
                                is_open.set(false);
                            },
                            "> ADD_CUSTOM_TOKEN [+] "
                        }
                    }
                }
            }
        }
    }
}
