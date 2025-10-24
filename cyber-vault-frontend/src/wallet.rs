use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{console, window};

#[derive(Clone, Debug)]
pub struct WalletProvider {
    pub connected: bool,
    pub public_key: Option<Pubkey>,
}

impl WalletProvider {
    pub fn new() -> Self {
        Self {
            connected: false,
            public_key: None,
        }
    }

    // Check if any wallet is available
    pub fn is_wallet_available() -> bool {
        console::log_1(&"Checking wallet availability...".into());

        let window = match window() {
            Some(w) => w,
            None => {
                console::log_1(&"No window object available".into());
                return false;
            }
        };

        // Check for Phantom wallet
        let phantom_available = js_sys::Reflect::get(&window, &JsValue::from_str("phantom"))
            .map(|phantom| {
                js_sys::Reflect::get(&phantom, &JsValue::from_str("solana"))
                    .map(|solana| !solana.is_undefined())
                    .unwrap_or(false)
            })
            .unwrap_or(false);

        // Check for other wallets
        let solflare_available = js_sys::Reflect::get(&window, &JsValue::from_str("solflare"))
            .map(|solflare| !solflare.is_undefined())
            .unwrap_or(false);

        console::log_1(&format!("Phantom available: {}", phantom_available).into());
        console::log_1(&format!("Solflare available: {}", solflare_available).into());

        phantom_available || solflare_available
    }

    pub async fn connect(&mut self) -> Result<Pubkey, String> {
        console::log_1(&"Starting wallet connection...".into());

        // First check if any wallet is available
        if !Self::is_wallet_available() {
            return Err(
                "No compatible wallet found. Please install Phantom or Solflare wallet extension."
                    .to_string(),
            );
        }

        let window = window().ok_or("No window object available")?;

        // Try Phantom first
        let mut wallet_result = match js_sys::Reflect::get(&window, &JsValue::from_str("phantom")) {
            Ok(phantom) => {
                console::log_1(&"Phantom wallet found".into());
                match js_sys::Reflect::get(&phantom, &JsValue::from_str("solana")) {
                    Ok(solana) => {
                        console::log_1(&"Phantom Solana object found".into());
                        self.connect_to_wallet(&solana, "Phantom").await
                    }
                    Err(e) => {
                        console::log_1(&format!("Phantom Solana object error: {:?}", e).into());
                        Err("Phantom Solana object not found".to_string())
                    }
                }
            }
            Err(e) => {
                console::log_1(&format!("Phantom wallet error: {:?}", e).into());
                Err("Phantom wallet not found".to_string())
            }
        };

        // If Phantom failed, try Solflare
        if wallet_result.is_err() {
            console::log_1(&"Trying Solflare wallet...".into());
            match js_sys::Reflect::get(&window, &JsValue::from_str("solflare")) {
                Ok(solflare) => {
                    console::log_1(&"Solflare wallet found".into());
                    wallet_result = self.connect_to_wallet(&solflare, "Solflare").await;
                }
                Err(e) => {
                    console::log_1(&format!("Solflare wallet error: {:?}", e).into());
                }
            }
        }

        wallet_result
    }

    async fn connect_to_wallet(
        &mut self,
        wallet: &JsValue,
        wallet_name: &str,
    ) -> Result<Pubkey, String> {
        console::log_1(&format!("Connecting to {} wallet...", wallet_name).into());
        // Check if already connected
        let is_connected = match js_sys::Reflect::get(wallet, &JsValue::from_str("isConnected")) {
            Ok(connected) => {
                let is_conn = connected.as_bool().unwrap_or(false);
                console::log_1(&format!("Wallet already connected: {}", is_conn).into());
                is_conn
            }
            Err(e) => {
                console::log_1(&format!("Error checking connection status: {:?}", e).into());
                false
            }
        };

        if is_connected {
            console::log_1(&"Wallet already connected, getting public key...".into());

            // Get public key if already connected
            let public_key_promise = js_sys::Reflect::get(wallet, &JsValue::from_str("publicKey"))
                .map_err(|e| {
                    console::log_1(&format!("Failed to get publicKey: {:?}", e).into());
                    format!("Failed to get publicKey: {:?}", e)
                })?;

            let public_key = JsFuture::from(js_sys::Promise::resolve(&public_key_promise))
                .await
                .map_err(|e| {
                    console::log_1(&format!("Failed to resolve public key: {:?}", e).into());
                    format!("Failed to resolve public key: {:?}", e)
                })?;

            // Try to get public key as string first, then try toString() method
            let public_key_str = if let Some(str_val) = public_key.as_string() {
                str_val
            } else {
                // Try to call toString() on the public key object
                let toString_result =
                    js_sys::Reflect::get(&public_key, &JsValue::from_str("toString"));
                match toString_result {
                    Ok(to_string_func) => {
                        let result = js_sys::Function::from(to_string_func).call0(&public_key);
                        match result {
                            Ok(str_val) => str_val.as_string().ok_or_else(|| {
                                console::log_1(&"toString() did not return a string".into());
                                "toString() did not return a string".to_string()
                            })?,
                            Err(e) => {
                                console::log_1(&format!("toString() call failed: {:?}", e).into());
                                return Err(format!("toString() call failed: {:?}", e));
                            }
                        }
                    }
                    Err(e) => {
                        console::log_1(&format!("No toString method found: {:?}", e).into());
                        return Err(format!(
                            "Public key object has no string representation: {:?}",
                            e
                        ));
                    }
                }
            };

            console::log_1(&format!("Got public key: {}", public_key_str).into());

            let pubkey = Pubkey::from_str(&public_key_str).map_err(|e| {
                console::log_1(&format!("Invalid public key: {}", e).into());
                format!("Invalid public key: {}", e)
            })?;

            self.connected = true;
            self.public_key = Some(pubkey);
            return Ok(pubkey);
        }

        console::log_1(&"Attempting to connect to wallet...".into());

        // Try to get connect method - different wallets might have different APIs
        let connect_promise = match js_sys::Reflect::get(wallet, &JsValue::from_str("connect")) {
            Ok(connect) => {
                console::log_1(&"Found connect method".into());
                connect
            }
            Err(e) => {
                console::log_1(
                    &format!("Connect method not found, trying alternative: {:?}", e).into(),
                );

                // Try alternative methods
                if let Ok(connect) =
                    js_sys::Reflect::get(wallet, &JsValue::from_str("connectWallet"))
                {
                    console::log_1(&"Found connectWallet method".into());
                    connect
                } else if let Ok(connect) =
                    js_sys::Reflect::get(wallet, &JsValue::from_str("request"))
                {
                    console::log_1(&"Found request method, trying connect...".into());
                    connect
                } else {
                    return Err(format!(
                        "No connect method found in {}: {:?}",
                        wallet_name, e
                    ));
                }
            }
        };

        // Call the connect method properly as a function
        let connect_result = if connect_promise.is_function() {
            let connect_func = js_sys::Function::from(connect_promise);
            let promise = connect_func.call0(wallet).map_err(|e| {
                console::log_1(&format!("Failed to call connect method: {:?}", e).into());
                format!("Failed to call connect method: {:?}", e)
            })?;
            JsFuture::from(js_sys::Promise::from(promise))
                .await
                .map_err(|e| {
                    console::log_1(&format!("Failed to connect to wallet: {:?}", e).into());
                    format!("Failed to connect to wallet: {:?}", e)
                })?
        } else {
            JsFuture::from(js_sys::Promise::resolve(&connect_promise))
                .await
                .map_err(|e| {
                    console::log_1(&format!("Failed to connect to wallet: {:?}", e).into());
                    format!("Failed to connect to wallet: {:?}", e)
                })?
        };

        console::log_1(&"Wallet connected successfully".into());

        let public_key =
            match js_sys::Reflect::get(&connect_result, &JsValue::from_str("publicKey")) {
                Ok(pk) => {
                    console::log_1(&"Found public key in connect result".into());
                    pk
                }
                Err(e) => {
                    console::log_1(&format!("No public key in connect result: {:?}", e).into());
                    return Err("No public key in connect result".to_string());
                }
            };

        // Try to get public key as string first, then try toString() method
        let public_key_str = if let Some(str_val) = public_key.as_string() {
            str_val
        } else {
            // Try to call toString() on the public key object
            let toString_result = js_sys::Reflect::get(&public_key, &JsValue::from_str("toString"));
            match toString_result {
                Ok(to_string_func) => {
                    let result = js_sys::Function::from(to_string_func).call0(&public_key);
                    match result {
                        Ok(str_val) => str_val.as_string().ok_or_else(|| {
                            console::log_1(&"toString() did not return a string".into());
                            "toString() did not return a string".to_string()
                        })?,
                        Err(e) => {
                            console::log_1(&format!("toString() call failed: {:?}", e).into());
                            return Err(format!("toString() call failed: {:?}", e));
                        }
                    }
                }
                Err(e) => {
                    console::log_1(&format!("No toString method found: {:?}", e).into());
                    return Err(format!(
                        "Public key object has no string representation: {:?}",
                        e
                    ));
                }
            }
        };

        console::log_1(&format!("Connected with public key: {}", public_key_str).into());

        let pubkey = Pubkey::from_str(&public_key_str).map_err(|e| {
            console::log_1(&format!("Invalid public key: {}", e).into());
            format!("Invalid public key: {}", e)
        })?;

        self.connected = true;
        self.public_key = Some(pubkey);

        Ok(pubkey)
    }

    pub async fn disconnect(&mut self) -> Result<(), String> {
        console::log_1(&"Disconnecting wallet...".into());

        let window = window().ok_or("No window object available")?;

        // Try to disconnect from any available wallet
        let mut disconnected = false;

        // Try Phantom
        if let Ok(phantom) = js_sys::Reflect::get(&window, &JsValue::from_str("phantom")) {
            if let Ok(solana) = js_sys::Reflect::get(&phantom, &JsValue::from_str("solana")) {
                if let Ok(disconnect_promise) =
                    js_sys::Reflect::get(&solana, &JsValue::from_str("disconnect"))
                {
                    match JsFuture::from(js_sys::Promise::resolve(&disconnect_promise)).await {
                        Ok(_) => {
                            console::log_1(&"Successfully disconnected from Phantom".into());
                            disconnected = true;
                        }
                        Err(e) => {
                            console::log_1(
                                &format!("Failed to disconnect from Phantom: {:?}", e).into(),
                            );
                        }
                    }
                }
            }
        }

        // Try Solflare
        if !disconnected {
            if let Ok(solflare) = js_sys::Reflect::get(&window, &JsValue::from_str("solflare")) {
                if let Ok(disconnect_promise) =
                    js_sys::Reflect::get(&solflare, &JsValue::from_str("disconnect"))
                {
                    match JsFuture::from(js_sys::Promise::resolve(&disconnect_promise)).await {
                        Ok(_) => {
                            console::log_1(&"Successfully disconnected from Solflare".into());
                            disconnected = true;
                        }
                        Err(e) => {
                            console::log_1(
                                &format!("Failed to disconnect from Solflare: {:?}", e).into(),
                            );
                        }
                    }
                }
            }
        }

        self.connected = false;
        self.public_key = None;
        console::log_1(&"Wallet disconnected successfully".into());
        Ok(())
    }

    pub async fn sign_transaction(&self, transaction: &[u8]) -> Result<Vec<u8>, String> {
        let window = window().ok_or("No window object available")?;
        let phantom = match js_sys::Reflect::get(&window, &JsValue::from_str("phantom")) {
            Ok(phantom) => phantom,
            Err(_) => return Err("Phantom wallet not found".to_string()),
        };

        let solana = match js_sys::Reflect::get(&phantom, &JsValue::from_str("solana")) {
            Ok(solana) => solana,
            Err(_) => return Err("Phantom Solana object not found".to_string()),
        };

        // Convert transaction to JS Uint8Array
        let js_array = js_sys::Uint8Array::new_with_length(transaction.len() as u32);
        for (i, &byte) in transaction.iter().enumerate() {
            js_array.set_index(i as u32, byte);
        }

        let transaction_obj = js_sys::Object::new();
        js_sys::Reflect::set(&transaction_obj, &JsValue::from_str("data"), &js_array)
            .map_err(|e| format!("Failed to set transaction data: {:?}", e))?;

        let sign_transaction_promise =
            js_sys::Reflect::get(&solana, &JsValue::from_str("signTransaction"))
                .map_err(|e| format!("Failed to get signTransaction method: {:?}", e))?;

        let sign_result = JsFuture::from(js_sys::Promise::resolve(&sign_transaction_promise))
            .await
            .map_err(|e| format!("Failed to sign transaction: {:?}", e))?;

        let signed_data = match js_sys::Reflect::get(&sign_result, &JsValue::from_str("data")) {
            Ok(data) => data,
            Err(_) => return Err("No data in signed transaction".to_string()),
        };

        let signed_array = js_sys::Uint8Array::new(&signed_data);
        let mut signed_vec = vec![0; signed_array.length() as usize];
        signed_array.copy_to(&mut signed_vec);

        Ok(signed_vec)
    }

    pub fn get_public_key(&self) -> Option<Pubkey> {
        self.public_key
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

// Utility function to check if Phantom wallet is available
pub fn is_phantom_available() -> bool {
    console::log_1(&"Checking Phantom availability...".into());

    if let Some(window) = window() {
        if let Ok(phantom) = js_sys::Reflect::get(&window, &JsValue::from_str("phantom")) {
            if let Ok(solana) = js_sys::Reflect::get(&phantom, &JsValue::from_str("solana")) {
                let available = !solana.is_undefined();
                console::log_1(&format!("Phantom available: {}", available).into());
                return available;
            }
        }
    }
    console::log_1(&"Phantom not available".into());
    false
}

// Utility function to format public key for display
pub fn format_public_key(pubkey: &Pubkey) -> String {
    let pubkey_str = pubkey.to_string();
    if pubkey_str.len() > 12 {
        format!(
            "{}...{}",
            &pubkey_str[..6],
            &pubkey_str[pubkey_str.len() - 6..]
        )
    } else {
        pubkey_str
    }
}
