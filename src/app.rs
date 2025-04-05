use dioxus::prelude::*;

const APP_CSS: Asset = asset!("src/css/app.css");


#[component]
pub fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: APP_CSS }

            h1 { "Hello, Dioxus!" }
            p { "Welcome to your simplified Dioxus application." }
    }
}