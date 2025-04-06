use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use near_jsonrpc_client::{JsonRpcClient, methods};
use near_primitives::types::AccountId;
use near_primitives::transaction::{Action, Transaction};
use near_primitives::views::FinalExecutionOutcomeView;
use std::str::FromStr;
use serde_json::json;
use crate::near_credentials::NearCredential;

const GREETING_UPDATER_CSS: Asset = asset!("src/css/greeting_updater.css");

#[derive(Serialize, Deserialize)]
pub struct TransactionPreview {
    network: String,
    contract_id: String,
    method_name: String,
    args: String,
    gas: String,
    deposit: String,
}

async fn submit_transaction(
    network: bool,
    contract_id: &str,
    new_greeting: &str,
    credential: &NearCredential,
) -> Result<FinalExecutionOutcomeView, String> {
    let config = if network {
        toml::from_str::<toml::Value>(include_str!("network_config.toml"))
            .unwrap()["mainnet"]
            .clone()
    } else {
        toml::from_str::<toml::Value>(include_str!("network_config.toml"))
            .unwrap()["testnet"]
            .clone()
    };

    let rpc_url = config["rpc_url"].as_str().unwrap();
    let client = JsonRpcClient::connect(rpc_url);
    
    let contract_account_id = AccountId::from_str(contract_id)
        .map_err(|e| format!("Invalid contract ID: {}", e))?;
    
    let signer_account_id = AccountId::from_str(&credential.account_id)
        .map_err(|e| format!("Invalid signer account ID: {}", e))?;

    let args = json!({
        "greeting": new_greeting
    });

    let action = Action::FunctionCall(Box::new(near_primitives::transaction::FunctionCallAction {
        method_name: "set_greeting".to_string(),
        args: args.to_string().into_bytes(),
        gas: 30_000_000_000_000, // 30 TGas
        deposit: 0,
    }));

    let transaction = Transaction {
        signer_id: signer_account_id,
        public_key: credential.public_key.clone(),
        nonce: 0, // Will be set by the RPC
        receiver_id: contract_account_id,
        block_hash: Default::default(), // Will be set by the RPC
        actions: vec![action],
    };

    let private_key = credential.private_key.as_ref()
        .ok_or_else(|| "Private key not found".to_string())?;

    client
        .call(methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
            signed_transaction: transaction.sign(private_key),
        })
        .await
        .map_err(|e| format!("Failed to submit transaction: {}", e))
}

#[component]
pub fn GreetingUpdater(network: bool, selected_account: Option<NearCredential>) -> Element {
    let mut new_greeting = use_signal(|| String::new());
    let mut transaction_preview = use_signal(|| None::<TransactionPreview>);
    let mut transaction_status = use_signal(|| None::<String>);

    let update_preview = move || {
        let network_name = if network { "mainnet" } else { "testnet" };
        let config = if network {
            toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                .unwrap()["mainnet"]
                .clone()
        } else {
            toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                .unwrap()["testnet"]
                .clone()
        };
        let contract_id = config["contract_id"].as_str().unwrap();
        transaction_preview.set(Some(TransactionPreview {
            network: network_name.to_string(),
            contract_id: contract_id.to_string(),
            method_name: "set_greeting".to_string(),
            args: format!("{{\
                \"greeting\":\"{}\"}}", new_greeting()),
            gas: "30 TGas".to_string(),
            deposit: "0 NEAR".to_string(),
        }));
    };

    rsx! {
        link { rel: "stylesheet", href: GREETING_UPDATER_CSS }
        div { class: "greeting-updater",
            h2 { "Update Greeting" }
            div { class: "input-group",
                input {
                    class: "greeting-input",
                    placeholder: "Enter new greeting",
                    value: new_greeting,
                    oninput: move |evt| {
                        new_greeting.set(evt.value().to_string());
                        update_preview();
                    }
                }
                button {
                    class: "update-button",
                    disabled: new_greeting().is_empty() || selected_account.is_none(),
                    onclick: move |_| {
                        if let Some(account) = selected_account.as_ref() {
                            transaction_status.set(Some("Preparing transaction...".to_string()));
                            let config = if network {
                                toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                                    .unwrap()["mainnet"]
                                    .clone()
                            } else {
                                toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                                    .unwrap()["testnet"]
                                    .clone()
                            };
                            let contract_id = config["contract_id"].as_str().unwrap();
                            
                            to_owned![new_greeting, transaction_status];
                            spawn(async move {
                                match submit_transaction(
                                    network,
                                    contract_id,
                                    &new_greeting(),
                                    account,
                                ).await {
                                    Ok(_) => {
                                        transaction_status.set(Some("Transaction successful!".to_string()));
                                    }
                                    Err(e) => {
                                        transaction_status.set(Some(format!("Transaction failed: {}", e)));
                                    }
                                }
                            });
                        } else {
                            transaction_status.set(Some("Please select an account first".to_string()));
                        }
                    },
                    "Update Greeting"
                }
            }

            div { class: "transaction-preview",
                if let Some(status) = transaction_status() {
                    div { class: "preview-item status",
                        span { class: "label", "Status: " }
                        span { class: "value", "{status}" }
                    }
                }
                h3 { "Transaction Preview" }
                if let Some(preview) = transaction_preview.get() {
                    div { class: "preview-content",
                        div { class: "preview-item",
                            span { class: "label", "Network: " }
                            span { class: "value", "{preview.network}" }
                        }
                        div { class: "preview-item",
                            span { class: "label", "Contract: " }
                            span { class: "value", "{preview.contract_id}" }
                        }
                        div { class: "preview-item",
                            span { class: "label", "Method: " }
                            span { class: "value", "{preview.method_name}" }
                        }
                        div { class: "preview-item",
                            span { class: "label", "Arguments: " }
                            span { class: "value", "{preview.args}" }
                        }
                        div { class: "preview-item",
                            span { class: "label", "Gas: " }
                            span { class: "value", "{preview.gas}" }
                        }
                        div { class: "preview-item",
                            span { class: "label", "Deposit: " }
                            span { class: "value", "{preview.deposit}" }
                        }
                    }
                }
            }
        }
    }
}
