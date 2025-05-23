use anyhow::Result;
use serde_json::Value;
use serde_json::json;
use base64::Engine;
use fips204::ml_dsa_44::{PrivateKey, PublicKey};
use fips204::traits::{SerDes, Signer, Verifier};
use crate::utils::base64::B64_ENCODER;

pub struct SessionIdentity {
    pub message: String,
    pub signature: String,
    pub target: String,
    pub session_id: String,
    pub user: Value
}

pub async fn session_identify(session_token: &str) -> Result<SessionIdentity> {
    let session_data_base64 = session_token.split('.').nth(0).ok_or_else(|| anyhow::anyhow!("Invalid session data format"))?;
    // println!("session_data_base64: {}", session_data_base64);
    let session_data: Value = serde_json::de::from_slice(&*B64_ENCODER.b64_decode_payload(session_data_base64).map_err(|e| anyhow::anyhow!("Failed to decode session data: {}", e))?).map_err(|e| anyhow::anyhow!("Failed to parse session data: {}", e))?;
    // println!("session_data: {:?}", session_data);


    let signature_base64 = session_token.split('.').nth(1).ok_or_else(|| anyhow::anyhow!("Invalid session token format"))?;
    // println!("signature_base64: {}", signature_base64);

    let target = session_data.get("aud")
        .and_then(|e| e.as_str())
        .ok_or_else(|| anyhow::anyhow!("Session data missing audience"))?;

    let target = target.parse::<String>().map_err(|e| anyhow::anyhow!("Failed to parse target to String: {}", e))?;

    let session_id = session_data.get("id")
        .and_then(|e| e.as_str())
        .ok_or_else(|| anyhow::anyhow!("Session data missing id"))?;

    let session_id = session_id.parse::<String>().map_err(|e| anyhow::anyhow!("Failed to parse session_id to String: {}", e))?;

    // let request_payload: Value = json!({
    //     "message": session_data_base64,
    //     "signature": signature_base64,
    //     "target": target,
    //     "session_id": session_id,
    // });

    let result = SessionIdentity {
        message: session_data_base64.to_string(),
        signature: signature_base64.to_string(),
        target,
        session_id,
        user: session_data.clone()
    };

    Ok(result)
}