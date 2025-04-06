use dioxus::prelude::*;
use crate::near_credentials::{NearCredential, load_near_credentials};

#[component]
pub fn AccountSelector(cx: Scope, network: bool) -> Element {
    let credentials = use_memo(cx, (), |_| load_near_credentials());
    let filtered_accounts = credentials.iter()
        .filter(|cred| {
            let network_name = if *network { "mainnet" } else { "testnet" };
            cred.network == network_name
        })
        .collect::<Vec<_>>();
    
    let selected_account = use_signal(cx, || None::<String>);

    rsx! {
        div { class: "AccountSelector_container",
            select {
                class: "AccountSelector_select",
                onchange: move |evt| selected_account.set(Some(evt.value.clone())),
                option { 
                    value: "",
                    disabled: true,
                    selected: selected_account().is_none(),
                    "Select an account"
                }
                filtered_accounts.iter().map(|cred| {
                    rsx! {
                        option {
                            key: "{cred.account_id}",
                            value: "{cred.account_id}",
                            selected: selected_account().as_ref() == Some(&cred.account_id),
                            "{cred.account_id}"
                        }
                    }
                })
            }
        }
    }
}