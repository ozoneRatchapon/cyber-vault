use dioxus::prelude::*;

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

                        rsx! {
                            div {
                                class: if is_selected {
                                    "cyber-card bg-[#1e2433] border-cyan-400"
                                } else {
                                    "cyber-card bg-[#141925] border-[#2a3441] hover:border-cyan-400 hover:transform hover:-translate-y-1"
                                },
                                onclick: move |_| on_select.call(vault_clone.clone()),

                                div { class: "text-xl mb-2 text-cyan-300", "🏦" }
                                div { class: "text-base font-medium text-gray-200", "Vault #{index + 1}" }
                                div { class: "text-sm text-gray-400", "Balance: {vault.balance / 1000000}" }
                                div { class: "text-xs text-pink-500 font-mono", "{&vault.beneficiary[..8]}..." }
                            }
                        }
                    })}
                }
            }
        }
    }
}
