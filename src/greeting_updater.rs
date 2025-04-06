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
    let new_greeting = use_signal(|| String::new());
    let transaction_preview = use_signal(|| None::<TransactionPreview>);

    let update_preview = move || {
        let network_name = if network { "mainnet" } else { "testnet" };
        transaction_preview.set(Some(TransactionPreview {
            network: network_name.to_string(),
            contract_id: if network { "hello.near" } else { "hello.testnet" }.to_string(),
            method_name: "set_greeting".to_string(),
            args: format!("{{\"
                greeting\":\"{}\"}}", new_greeting()),
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
                        new_greeting.set(evt.value.clone());
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
                if let Some(preview) = transaction_preview() {
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