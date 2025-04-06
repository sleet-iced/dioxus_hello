use dioxus::prelude::*;
use crate::near_credentials::load_near_credentials;

#[component]
pub fn AccountSelector(network: bool) -> Element {
    let credentials = use_signal(|| load_near_credentials());
    let selected_account = use_signal(|| None::<String>);
    
    let network_name = if network { "mainnet" } else { "testnet" };
    let filtered_accounts = credentials()
        .into_iter()
        .filter(|cred| cred.network == network_name)
        .collect::<Vec<_>>();

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
                filtered_accounts.into_iter().map(|cred| {
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