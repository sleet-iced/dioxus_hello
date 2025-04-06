/// ❄️
/// dioxus Hello by Sleet.near
use dioxus::prelude::*;

mod app;
mod account_selector;
mod near_credentials;
mod greeting_viewer;
mod greeting_updater;
use app::MainApp as MainApp;


const MAIN_CSS: Asset = asset!("src/css/main.css");
const FAVICON: Asset = asset!("src/img/sleet_code_icon_trans.png");



pub fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        MainApp {}
    }
}