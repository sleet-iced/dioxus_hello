use dioxus::prelude::*;
use serde::Deserialize;
use near_jsonrpc_client::JsonRpcClient;
use near_primitives::types::AccountId;
use near_jsonrpc_primitives::types::query::QueryResponseKind;
use near_primitives::views::{QueryRequest, FunctionResult};
use std::str::FromStr;
use serde_json::json;
use base64::encode;

const GREETING_CSS: Asset = asset!("src/css/greeting_viewer.css");

#[derive(Deserialize)]
struct GreetingResponse {
    greeting: String,
}

#[component]
pub fn GreetingViewer(network: bool) -> Element {
    let greeting = use_signal(|| String::from(""));
    let loading = use_signal(|| false);
    let error = use_signal(|| String::from(""));

    use_effect(move || {
        to_owned![greeting, loading, error];
        loading.set(true);
        error.set(String::from(""));

        spawn(async move {
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

            let client = JsonRpcClient::connect(rpc_url);
            let account_id = match AccountId::from_str(contract_id) {
                Ok(id) => id,
                Err(e) => {
                    error.set(format!("Invalid contract ID: {}", e));
                    loading.set(false);
                    return;
                }
            };

            let args = json!({});
            let query = QueryRequest::CallFunction {
                account_id,
                method_name: "get_greeting".to_string(),
                args: args.to_string().into_bytes(),
            };

            match client.query(query).await {
                Ok(response) => {
                    if let QueryResponseKind::CallResult(FunctionResult { result, .. }) = response.kind {
                        match serde_json::from_slice::<GreetingResponse>(&result) {
                            Ok(response) => greeting.set(response.greeting),
                            Err(_) => {
                                match String::from_utf8(result.to_vec()) {
                                    Ok(raw_greeting) => greeting.set(raw_greeting.trim_matches('"').to_string()),
                                    Err(e) => error.set(format!("Failed to parse response: {}", e)),
                                }
                            }
                        }
                    } else {
                        error.set("Unexpected response type".to_string());
                    }
                }
                Err(e) => error.set(format!("Failed to fetch greeting: {}", e)),
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