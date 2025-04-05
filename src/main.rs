use dioxus::prelude::*;
mod app;
use app::App;

const MAIN_CSS: Asset = asset!("src/css/hello.css");

fn main() {
    dioxus::launch(App);
}
