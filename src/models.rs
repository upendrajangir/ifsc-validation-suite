use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankData {
  pub ifsc: String,
  pub bank: String,
  pub branch: String,
  pub address: String,
  pub contact: Option<String>,
  pub city: String,
  pub rtgs: bool,
  pub neft: bool,
  pub imps: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IfscValidationResponse {
  pub ifsc_code: String,
  pub valid: bool,
  pub bank_data: Option<BankData>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
  pub success: bool,
  pub data: Option<T>,
  pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct ThirdPartyIfscResponse {
  #[serde(rename = "IFSC")]
  pub ifsc: String,
  #[serde(rename = "BANK")]
  pub bank: String,
  #[serde(rename = "BRANCH")]
  pub branch: String,
  #[serde(rename = "ADDRESS")]
  pub address: String,
  #[serde(rename = "CONTACT")]
  pub contact: Option<String>,
  #[serde(rename = "CITY")]
  pub city: String,
  #[serde(rename = "RTGS")]
  pub rtgs: bool,
  #[serde(rename = "NEFT")]
  pub neft: bool,
  #[serde(rename = "IMPS")]
  pub imps: bool,
}