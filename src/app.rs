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
            p {"FOR INTERACTING WITH A HELLO CONRTACT ON NEAR" }
    }
}
