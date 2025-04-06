use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct NearCredential {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "publicKey")]
    pub public_key: String,
    pub network: String,
    #[serde(rename = "privateKey")]
    pub private_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RawCredential {
    #[serde(rename = "public_key")]
    public_key: String,
    #[serde(rename = "private_key")]
    private_key: String,
    #[serde(skip)]
    _other: serde_json::Value,
}

pub fn load_near_credentials() -> Vec<NearCredential> {
    let mut credentials = Vec::new();
    
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => return credentials,
    };

    let near_credentials_dir = home_dir.join(".near-credentials");
    
    for network in ["mainnet", "testnet"] {
        let network_path = near_credentials_dir.join(network);
        if !network_path.is_dir() {
            continue;
        }

        if let Ok(files) = fs::read_dir(&network_path) {
            for file in files {
                if let Ok(file) = file {
                    let path = file.path();
                    if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("json") {
                        continue;
                    }

                    let account_id = match path.file_stem().and_then(|s| s.to_str()) {
                        Some(name) => name.to_string(),
                        None => continue,
                    };

                    if let Ok(content) = fs::read_to_string(&path) {
                        if let Ok(raw_cred) = serde_json::from_str::<RawCredential>(&content) {
                            credentials.push(NearCredential {
                                account_id,
                                public_key: raw_cred.public_key,
                                network: network.to_string(),
                                private_key: Some(raw_cred.private_key),
                            });
                        }
                    }
                }
            }
        }
    }
    credentials
}