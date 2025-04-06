use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use near_jsonrpc_client::JsonRpcClient;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::transaction::{Action, FunctionCallAction, Transaction, SignedTransaction};
use near_primitives::types::{AccountId, BlockReference, Finality};
use crate::near_credentials::NearCredential;
use std::str::FromStr;
use near_crypto;

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

#[component]
pub fn GreetingUpdater(cx: Scope, network: bool, selected_account: Option<NearCredential>) -> Element {
    let mut new_greeting = use_signal(|| String::new());
    let mut transaction_preview = use_signal(|| None::<TransactionPreview>);
    let mut transaction_status = use_signal(|| None::<String>);

    let mut update_preview = move || {
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
            args: format!("{{\"greeting\": \"{}\"}}", new_greeting()),
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
                        new_greeting.set(evt.value().clone());
                        update_preview();
                    }
                }
                button {
                    class: "update-button",
                    disabled: new_greeting().is_empty(),
                    onclick: move |_| {
                        if let Some(account) = selected_account.as_ref() {
                            let rpc_url = if network {
                                "https://rpc.mainnet.near.org"
                            } else {
                                "https://rpc.testnet.near.org"
                            };
                            
                            let rpc_client = JsonRpcClient::connect(rpc_url);
                            
                            let contract_id = if network {
                                toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                                    .unwrap()["mainnet"]["contract_id"]
                                    .as_str()
                                    .unwrap()
                            } else {
                                toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                                    .unwrap()["testnet"]["contract_id"]
                                    .as_str()
                                    .unwrap()
                            };

                            let args = format!("{{\"greeting\": \"{}\"}}", new_greeting());
                            transaction_status.set(Some("Preparing transaction...".to_string()));

                            let signer_id = AccountId::from_str(&account.account_id).unwrap();
                            let receiver_id = AccountId::from_str(contract_id).unwrap();
                            
                            // Create function call action
                            let action = Action::FunctionCall(FunctionCallAction {
                                method_name: "set_greeting".to_string(),
                                args: args.into_bytes(),
                                gas: 30_000_000_000_000, // 30 TGas
                                deposit: 0,
                            });

                            cx.spawn(async move {
                                // Get latest block hash
                                let block_hash = match rpc_client
                                    .block(BlockReference::Finality(Finality::Final))
                                    .await {
                                        Ok(block) => block.header.hash,
                                        Err(err) => {
                                            transaction_status.set(Some(format!("Failed to get block: {}", err)));
                                            return;
                                        }
                                    };

                                // Get current nonce
                                let access_key_query_response = match rpc_client
                                    .query(near_jsonrpc_primitives::types::query::RpcQueryRequest {
                                        block_reference: BlockReference::Finality(Finality::Final),
                                        request: near_primitives::views::QueryRequest::ViewAccessKey {
                                            account_id: signer_id.clone(),
                                            public_key: account.public_key.parse().unwrap(),
                                        },
                                    })
                                    .await {
                                        Ok(response) => response,
                                        Err(err) => {
                                            transaction_status.set(Some(format!("Failed to get access key: {}", err)));
                                            return;
                                        }
                                    };

                                let current_nonce = match access_key_query_response.kind {
                                    QueryResponseKind::AccessKey(access_key) => access_key.nonce,
                                    _ => {
                                        transaction_status.set(Some("Failed to get current nonce".to_string()));
                                        return;
                                    }
                                };

                                // Create transaction
                                let transaction = Transaction {
                                    signer_id,
                                    public_key: account.public_key.parse().unwrap(),
                                    nonce: current_nonce + 1,
                                    receiver_id,
                                    block_hash,
                                    actions: vec![action],
                                };

                                // Sign transaction
                                if let Some(private_key) = &account.private_key {
                                    let signer = near_crypto::InMemorySigner::from_secret_key(
                                        account.account_id.parse().unwrap(),
                                        private_key.parse().unwrap(),
                                    );

                                    let signed_transaction = SignedTransaction::sign(
                                        transaction,
                                        &signer,
                                        account.public_key.parse().unwrap(),
                                    );

                                    transaction_status.set(Some("Sending transaction...".to_string()));

                                    // Send transaction
                                    match rpc_client.broadcast_tx_commit(&signed_transaction).await {
                                        Ok(outcome) => {
                                            transaction_status.set(Some(format!(
                                                "Transaction successful! Hash: {}",
                                                outcome.transaction_outcome.id
                                            )));
                                        }
                                        Err(err) => {
                                            transaction_status.set(Some(format!("Transaction failed: {}", err)));
                                        }
                                    }
                                } else {
                                    transaction_status.set(Some("Private key not available".to_string()));
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
                if let Some(status) = transaction_status.read().as_ref() {
                    rsx!(
                        div { class: "preview-item status",
                            span { class: "label", "Status: " }
                            span { class: "value", "{status}" }
                        }
                    )
                }
                h3 { "Transaction Preview" }
                match transaction_preview.read().as_ref() {
                    Some(preview) => rsx!(
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
                    ),
                    None => rsx!()
                }
            }
        }
    }
}