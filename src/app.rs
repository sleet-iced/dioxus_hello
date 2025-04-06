use dioxus::prelude::*;

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
    rsx! {
        h1 { "Hello, Dioxus!" }
        p { "🧬 A HELLO DIOXUS PROJECT BY SLEET" }
        p { "FOR INTERACTING WITH A HELLO CONRTACT ON NEAR" }
        NetworkToggle {}
    }
}



#[component]
pub fn NetworkToggle() -> Element {
    let mut is_mainnet = use_signal(|| true);  // Declare as mutable

    rsx! {
        button {
            onclick: move |_| is_mainnet.set(!is_mainnet()),
            style: "padding: 8px 16px; border-radius: 4px; cursor: pointer; background-color: #0070f3; color: white; border: none;",
            if is_mainnet() { "Mainnet" } else { "Testnet" }
        }
    }
}
