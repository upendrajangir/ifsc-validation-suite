use crate::models::{BankData, ThirdPartyIfscResponse};
use chrono::Utc;
use reqwest::Client;
use std::time::Duration;

pub struct IfscService {
  client: Client,
}

impl IfscService {
  pub fn new() -> Self {
    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .expect("Failed to create HTTP client");

    Self { client }
  }

  pub async fn fetch_bank_data(&self, ifsc_code: &str) -> anyhow::Result<BankData> {
    let url = format!("https://ifsc.razorpay.com/{}", ifsc_code);

    let response = self
        .client
        .get(&url)
        .send()
        .await?;

    if !response.status().is_success() {
      return Err(anyhow::anyhow!("IFSC code not found or API error"));
    }

    let third_party_response: ThirdPartyIfscResponse = response.json().await?;

    let now = Utc::now();
    let bank_data = BankData {
      ifsc: third_party_response.ifsc,
      bank: third_party_response.bank,
      branch: third_party_response.branch,
      address: third_party_response.address,
      contact: third_party_response.contact,
      city: third_party_response.city,
      rtgs: third_party_response.rtgs,
      neft: third_party_response.neft,
      imps: third_party_response.imps,
      created_at: now,
      updated_at: now,
    };

    Ok(bank_data)
  }
}