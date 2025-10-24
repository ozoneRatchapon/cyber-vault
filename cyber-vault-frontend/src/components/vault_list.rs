use dioxus::prelude::*;

#[component]
pub fn VaultList(
    vaults: Vec<crate::VaultInfo>,
    selected_vault: Option<crate::VaultInfo>,
    on_select: EventHandler<crate::VaultInfo>,
) -> Element {
    rsx! {
        div { class: "bg-gray-800 bg-opacity-90 backdrop-blur-sm rounded-xl p-6 shadow-xl border border-gray-700",
            div { class: "flex items-center space-x-3 mb-6",
                div { class: "text-3xl", "🏦" }
                h2 { class: "text-2xl font-bold bg-gradient-to-r from-blue-400 to-purple-600 bg-clip-text text-transparent", "Your Vaults" }
                div { class: "bg-blue-500 bg-opacity-20 text-blue-300 px-3 py-1 rounded-full text-sm font-semibold", "{vaults.len()} Total" }
            }

            if vaults.is_empty() {
                div { class: "text-center py-12 text-gray-400",
                    div { class: "text-6xl mb-4 animate-bounce", "🏦" }
                    h3 { class: "text-xl font-semibold mb-2", "No Vaults Found" }
                    p { class: "text-sm", "Create your first vault to get started" }
                    div { class: "mt-6",
                        div { class: "inline-flex items-center space-x-2 bg-gray-700 bg-opacity-50 px-4 py-2 rounded-lg border border-gray-600",
                            span { class: "text-2xl", "📝" }
                            span { "Use the form to create your first vault" }
                        }
                    }
                }
            } else {
                div { class: "space-y-4",
                    for (index, vault) in vaults.iter().enumerate() {
                        VaultCard {
                            vault: vault.clone(),
                            index,
                            selected: selected_vault.as_ref().map_or(false, |v| v.pubkey == vault.pubkey),
                            on_select: on_select.clone(),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn VaultCard(
    vault: crate::VaultInfo,
    index: usize,
    selected: bool,
    on_select: EventHandler<crate::VaultInfo>,
) -> Element {
    let is_expired =
        vault.last_heartbeat + vault.inactivity_period <= js_sys::Date::now() as i64 / 1000;

    rsx! {
        div {
            class: "bg-gray-700 bg-opacity-50 rounded-xl p-5 border cursor-pointer",
            onclick: move |_| on_select.call(vault.clone()),
            h3 { "Vault #{index + 1}" }
        }
    }
}
