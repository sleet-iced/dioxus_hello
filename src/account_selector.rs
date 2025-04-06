use dioxus::prelude::*;
use crate::near_credentials::load_near_credentials;

const ACCOUNT_SELECTOR_CSS: Asset = asset!("src/css/account_selector.css");


#[component]
pub fn AccountSelector(network: bool, onselect: EventHandler<String>) -> Element {
    let credentials = use_signal(|| load_near_credentials());
    let mut selected_account = use_signal(|| None::<String>);
    
    let network_name = if network { "mainnet" } else { "testnet" };
    let filtered_accounts = credentials()
        .into_iter()
        .filter(|cred| cred.network == network_name)
        .collect::<Vec<_>>();

    rsx! {
        document::Link { rel: "stylesheet", href: ACCOUNT_SELECTOR_CSS }
        div { class: "AccountSelector_container",
            select {
                class: "AccountSelector_select",
                onchange: move |evt| {
                    let account_id = evt.value().clone();
                    selected_account.set(Some(account_id.clone()));
                    onselect.call(account_id);
                },
                option { 
                    value: "",
                    disabled: true,
                    selected: selected_account().is_none(),
                    "Select an account"
                }
                for cred in filtered_accounts {
                    option {
                        key: "{cred.account_id}",
                        value: "{cred.account_id}",
                        selected: selected_account().as_ref() == Some(&cred.account_id),
                        "{cred.account_id}"
                    }
                }
            }
        }
    }
}