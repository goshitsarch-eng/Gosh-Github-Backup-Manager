use serde::Deserialize;

// Replace with your registered GitHub OAuth App client ID
pub const GITHUB_CLIENT_ID: &str = "Ov23liGgLL9AjIdQnRFt";

#[derive(Debug, Clone, Deserialize)]
pub struct DeviceCodeResponse {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: u64,
    pub interval: u64,
}

#[derive(Debug, Clone)]
pub enum OAuthPollResult {
    Pending,
    SlowDown,
    Success(String),
    Expired,
    AccessDenied,
    Error(String),
}

#[derive(Debug, Deserialize)]
struct TokenResponse {
    #[serde(default)]
    access_token: Option<String>,
    #[serde(default)]
    error: Option<String>,
}

pub async fn request_device_code(client_id: &str) -> Result<DeviceCodeResponse, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://github.com/login/device/code")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id),
            ("scope", "repo,read:user,read:org"),
        ])
        .send()
        .await
        .map_err(|e| format!("Failed to request device code: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("GitHub returned {}: {}", status, body));
    }

    resp.json::<DeviceCodeResponse>()
        .await
        .map_err(|e| format!("Failed to parse device code response: {}", e))
}

pub async fn poll_for_token(client_id: &str, device_code: &str) -> Result<OAuthPollResult, String> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", client_id),
            ("device_code", device_code),
            (
                "grant_type",
                "urn:ietf:params:oauth:grant-type:device_code",
            ),
        ])
        .send()
        .await
        .map_err(|e| format!("Failed to poll for token: {}", e))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(format!("GitHub returned {}: {}", status, body));
    }

    let token_resp: TokenResponse = resp
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    if let Some(token) = token_resp.access_token {
        return Ok(OAuthPollResult::Success(token));
    }

    match token_resp.error.as_deref() {
        Some("authorization_pending") => Ok(OAuthPollResult::Pending),
        Some("slow_down") => Ok(OAuthPollResult::SlowDown),
        Some("expired_token") => Ok(OAuthPollResult::Expired),
        Some("access_denied") => Ok(OAuthPollResult::AccessDenied),
        Some(other) => Ok(OAuthPollResult::Error(other.to_string())),
        None => Ok(OAuthPollResult::Error("Unknown response from GitHub".to_string())),
    }
}
