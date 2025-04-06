use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

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
pub fn GreetingUpdater(network: bool) -> Element {
    let mut new_greeting = use_signal(|| String::new());
    let mut transaction_preview = use_signal(|| None::<TransactionPreview>);

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
            args: format!("{{\"greeting\":\"{}\"}}", new_greeting()),
            gas: "30 TGas".to_string(),
            deposit: "0 NEAR".to_string(),
        }));
    };

    rsx! {
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
                        // TODO: Implement update functionality
                    },
                    "Update Greeting"
                }
            }

            div { class: "transaction-preview",
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