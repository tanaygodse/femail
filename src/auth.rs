use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredToken {
    pub access_token: String,
}

pub fn get_config_dir() -> Result<PathBuf> {
    let home_dir = std::env::var("HOME")
        .map_err(|_| anyhow!("Could not determine home directory"))?;
    let app_config_dir = PathBuf::from(home_dir).join(".config").join("femail");
    std::fs::create_dir_all(&app_config_dir)?;
    Ok(app_config_dir)
}

pub fn save_token(token: &StoredToken) -> Result<()> {
    let config_dir = get_config_dir()?;
    let token_path = config_dir.join("token.json");
    let token_content = serde_json::to_string_pretty(token)?;
    std::fs::write(&token_path, token_content)?;
    Ok(())
}

pub fn load_token() -> Result<StoredToken> {
    let config_dir = get_config_dir()?;
    let token_path = config_dir.join("token.json");
    
    if !token_path.exists() {
        return Err(anyhow!("Token file not found. Please run 'femail auth' first."));
    }

    let token_content = std::fs::read_to_string(&token_path)?;
    let token: StoredToken = serde_json::from_str(&token_content)?;
    Ok(token)
}