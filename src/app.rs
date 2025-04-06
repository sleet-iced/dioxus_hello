use dioxus::prelude::*;
use crate::account_selector::AccountSelector;
use crate::greeting_viewer::GreetingViewer;

const APP_CSS: Asset = asset!("src/css/app.css");



#[component]
pub fn MainApp() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: APP_CSS }
        Body {}
    }
}

#[component]
pub fn Body() -> Element {
    let mut network = use_signal(|| true);
    
    rsx! {
        h1 { "Hello, Dioxus!" }
        p { "🧬 A HELLO DIOXUS PROJECT BY SLEET" }
        p { "FOR INTERACTING WITH A HELLO CONRTACT ON NEAR" }
        NetworkToggle {
            onchange: move |val| network.set(val)
        }
        
        GreetingViewer {
            network: network()
        }

        AccountSelector {
            network: network()
        }
    }
}



#[component]
#[component]
pub fn NetworkToggle(onchange: EventHandler<bool>) -> Element {
    let mut is_mainnet = use_signal(|| true);

    rsx! {
        button {
            class: "NetworkToggle_button",
            onclick: move |_| {
                is_mainnet.set(!is_mainnet());
                onchange.call(is_mainnet());
            },
            if is_mainnet() { "MAINNET" } else { "TESTNET" }
        }
    }
}
