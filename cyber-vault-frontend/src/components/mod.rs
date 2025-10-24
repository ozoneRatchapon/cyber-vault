//! The components module contains all shared components for our app. Components are the building blocks of dioxus apps.
//! They can be used to defined common UI elements like buttons, forms, and modals.

mod create_vault_form;
mod header;
pub mod ui;
mod vault_list;
mod wallet_connector;

pub use create_vault_form::CreateVaultForm;
pub use header::Header;
pub use vault_list::VaultList;
// pub use wallet_connector::WalletConnector; // Currently unused

// Re-export UI components for easier access
pub use ui::{LoadingSpinner, NotificationCard, StatusBadge};
