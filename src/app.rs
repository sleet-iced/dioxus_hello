use dioxus::prelude::*;


#[component]
pub fn App() -> Element {
    rsx! {
        div { 
            class: "hello-container",
            h1 { "Hello, Dioxus!" }
            p { "Welcome to your simplified Dioxus application." }
        }
    }
}