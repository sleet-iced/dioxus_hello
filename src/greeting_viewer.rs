use dioxus::prelude::*;
use serde::Deserialize;
use near_jsonrpc_client::JsonRpcClient;
use near_primitives::types::FunctionArgs;
use near_primitives::views::QueryRequest;
use near_primitives::views::FunctionCallRequestView;
use base64::encode;
use serde_json::json;

const GREETING_CSS: Asset = asset!("src/css/greeting_viewer.css");

#[derive(Deserialize)]
struct GreetingResponse {
    result: Vec<u8>,
}

#[component]
pub fn GreetingViewer(network: bool) -> Element {
    let greeting = use_signal(|| String::from(""));
    let loading = use_signal(|| false);
    let error = use_signal(|| String::from(""));

    use_effect(move || {
        to_owned![greeting, loading, error];
        async move {
            loading.set(true);
            error.set(String::from(""));

            let config = if network {
                toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                    .unwrap()["mainnet"]
                    .clone()
            } else {
                toml::from_str::<toml::Value>(include_str!("network_config.toml"))
                    .unwrap()["testnet"]
                    .clone()
            };

            let rpc_url = config["rpc_url"].as_str().unwrap();
            let contract_id = config["contract_id"].as_str().unwrap();

            match JsonRpcClient::connect(rpc_url) {
                Ok(client) => {
                    let args = json!({}).to_string().into_bytes();
                    let query = QueryRequest::CallFunction {
                        account_id: contract_id.parse().unwrap(),
                        method_name: "get_greeting".to_string(),
                        args: FunctionArgs::from(encode(args)),
                    };

                    match client.call(query).await {
                        Ok(response) => {
                            if let Ok(result) = serde_json::from_slice::<GreetingResponse>(&response.result) {
                                if let Ok(message) = String::from_utf8(result.result) {
                                    greeting.set(message);
                                }
                            }
                        }
                        Err(e) => error.set(format!("Failed to fetch greeting: {}", e)),
                    }
                }
                Err(e) => error.set(format!("Failed to connect to RPC: {}", e)),
            }

            loading.set(false);
        }
    });

    rsx! {
        link { rel: "stylesheet", href: GREETING_CSS }
        div { class: "greeting-container",
            if loading() {
                div { class: "loading", "Loading..." }
            } else if !error().is_empty() {
                div { class: "error", "{error()}" }
            } else {
                div { class: "greeting", "{greeting()}" }
            }
        }
    }
}