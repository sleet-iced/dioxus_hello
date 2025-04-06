use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use near_jsonrpc_client::{JsonRpcClient, methods};
use near_primitives::borsh;
use near_primitives::types::AccountId;
use near_primitives::transaction::{Action, FunctionCallAction, Transaction, SignedTransaction};
use near_crypto::SecretKey;
use near_primitives::views::FinalExecutionOutcomeView;
use near_crypto::{InMemorySigner, PublicKey};
use std::str::FromStr;
use serde_json::json;
use crate::near_credentials::NearCredential;
use std::sync::LazyLock;
use crate::account_selector::AccountSelector;

const GREETING_UPDATER_CSS: Asset = asset!("src/css/greeting_updater.css");

#[derive(Serialize, Deserialize)]
pub struct TransactionPreview {
    pub network: String,
    pub contract_id: String,
    pub method_name: String,
    pub args: String,
    pub gas: String,
    pub deposit: String,
}

async fn submit_transaction(
    network: bool,
    new_greeting: &str,
    credential: &NearCredential,
) -> Result<FinalExecutionOutcomeView, String> {
    let config = if network { CONFIG_MAINNET.clone() } else { CONFIG_TESTNET.clone() };
    let contract_id = config["contract_id"].as_str().unwrap();
    let rpc_url = config["rpc_url"].as_str().unwrap();
    let client = JsonRpcClient::connect(rpc_url);

    let contract_account_id = AccountId::from_str(contract_id)
        .map_err(|e| format!("Invalid contract ID: {}", e))?;

    let signer_account_id = AccountId::from_str(&credential.account_id)
        .map_err(|e| format!("Invalid signer account ID: {}", e))?;

    let public_key = PublicKey::from_str(&credential.public_key)
        .map_err(|e| format!("Invalid public key: {}", e))?;

    let private_key = credential.private_key.as_ref()
        .ok_or_else(|| "Private key not found".to_string())?;

    let secret_key = SecretKey::from_str(private_key)
        .map_err(|e| format!("Invalid secret key: {}", e))?;
    let signer = InMemorySigner::from_secret_key(signer_account_id.clone(), secret_key);

    let args = json!({
        "greeting": new_greeting
    });

    let _action = Action::FunctionCall(Box::new(FunctionCallAction {
        method_name: "set_greeting".to_string(),
        args: args.to_string().into_bytes(),
        gas: 30_000_000_000_000, // 30 TGas
        deposit: 0,
    }));

    let access_key_query = methods::query::RpcQueryRequest {
        block_reference: near_primitives::types::Finality::Final.into(),
        request: near_primitives::views::QueryRequest::ViewAccessKey {
            account_id: signer_account_id.clone(),
            public_key: public_key.clone(),
        },
    };

    let access_key_response = client.call(access_key_query).await
        .map_err(|e| format!("Failed to fetch access key: {}", e))?;

    let block_hash = access_key_response.block_hash;

    let access_key_view = match access_key_response.kind {
        near_jsonrpc_primitives::types::query::QueryResponseKind::AccessKey(view) => view,
        _ => return Err("Failed to get access key view".to_string()),
    };

    let transaction = Transaction::new(
        signer_account_id,
        public_key,
        contract_account_id,
        access_key_view.nonce + 1,
        block_hash
    );

    let transaction_bytes = borsh::to_vec(&transaction).map_err(|e| e.to_string())?;
    let hash = near_primitives::hash::hash(&transaction_bytes);
    let signature = signer.secret_key.sign(hash.as_ref());
    let signed_transaction = SignedTransaction::new(signature, transaction);

    client
        .call(methods::broadcast_tx_commit::RpcBroadcastTxCommitRequest {
            signed_transaction,
        })
        .await
        .map_err(|e| format!("Failed to submit transaction: {}", e))
}

#[component]
pub fn GreetingUpdater(network: bool, selected_account: Option<NearCredential>) -> Element {
    let mut new_greeting = use_signal(|| String::new());
    let mut transaction_preview = use_signal(|| None::<TransactionPreview>);
    let mut transaction_status = use_signal(|| None::<String>);

    let mut update_preview = move || {
        let network_name = if network { "mainnet" } else { "testnet" };
        let config = if network { CONFIG_MAINNET.clone() } else { CONFIG_TESTNET.clone() };
        let contract_id = config["contract_id"].as_str().unwrap();
        transaction_preview.set(Some(TransactionPreview {
            network: network_name.to_string(),
            contract_id: contract_id.to_string(),
            method_name: "set_greeting".to_string(),
            args: format!("{{\"{}\": \"{}\"}}", "greeting", new_greeting()),
            gas: "30 TGas".to_string(),
            deposit: "0 NEAR".to_string(),
        }));
    };

    rsx! {
        link { rel: "stylesheet", href: GREETING_UPDATER_CSS }
        div { class: "greeting-updater",
            h2 { "Update Greeting" }
            AccountSelector { network: network }
            div { class: "input-group",
                input {
                    class: "greeting-input",
                    placeholder: "Enter new greeting",
                    value: "{new_greeting()}",
                    oninput: move |evt| {
                        new_greeting.set(evt.value().to_string());
                        update_preview();
                    }
                }
                button {
                    class: "update-button",
                    disabled: new_greeting().is_empty() || selected_account.is_none(),
                    onclick: move |_| {
                        let account = selected_account.clone();
                        if let Some(account) = account {
                            transaction_status.set(Some("Preparing transaction...".to_string()));
                            let config = if network {
                                toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                                    .unwrap()["mainnet"].clone()
                            } else {
                                toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                                    .unwrap()["testnet"].clone()
                            };
                            let contract_id = config["contract_id"].as_str().unwrap();

                            to_owned![new_greeting, transaction_status];
                            spawn(async move {
                                match submit_transaction(
                                    network,
                                    &new_greeting(),
                                    &account,
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
                {transaction_status().map(|status| rsx!(
                    div { class: "preview-item status",
                        span { class: "label", "Status: " }
                        span { class: "value", "{status}" }
                    }
                ))}
                h3 { "Transaction Preview" }
                {transaction_preview.read().as_ref().map(|preview| rsx!(
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
                ))}
            }
        }
    }
}

const CONFIG_MAINNET: LazyLock<toml::Value> = LazyLock::new(|| {
    toml::from_str::<toml::Value>(include_str!("network_config.toml")).unwrap()["mainnet"].clone()
});

const CONFIG_TESTNET: LazyLock<toml::Value> = LazyLock::new(|| {
    toml::from_str::<toml::Value>(include_str!("network_config.toml")).unwrap()["testnet"].clone()
});
