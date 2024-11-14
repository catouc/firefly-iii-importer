use crate::Record;
use serde::Deserialize;
use std::env;
pub type AccountID = u32;

#[derive(Deserialize)]
struct AccountResponse {
    id: u32,
}

pub fn create_account(token: &str, account_name: &str, account_iban: &str, account_type: &str) -> Result<String, Box<dyn std::error::Error>> {
    let resp = ureq::post("http://localhost:8080/api/v1/accounts")
        .set("Authorization", &format!("Bearer {}", token))
        .set("accept", "application/vnd.api+json")
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "name": account_name,
            "type": account_type,
        }))?;
}

#[derive(Deserialize)]
struct ListAccountResponse {
    data: Vec<Account>,
    meta: ListAccountMetadata,
}

#[derive(Deserialize)]
struct ListAccountMetadata {
    pagination: Pagination
}

#[derive(Deserialize)]
struct Pagination {
    total: u32,
    count: u32,
    per_page: u16,
    current_page: u32,
    total_pages: u32,
}

#[derive(Deserialize)]
struct Account {
    r#type: String,
    id: String,
    attributes: AccountAttributes,
}

#[derive(Deserialize)]
struct AccountAttributes {
    active: bool,
    name: String,
    r#type: String,
}

pub fn list_accounts_by_name(token: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut current_page = 0;
    let mut total_pages = 1;
    let mut accounts: Vec<String> = Vec::new();

    while current_page < total_pages {
        let resp: ListAccountResponse = ureq::get("http://localhost:8080/api/v1/accounts")
            .set("Authorization", &format!("Bearer {}", token))
            .set("accept", "application/vnd.api+json")
            .set("Content-Type", "application/json")
            .call()?
            .into_json()?;


        for account in resp.data.into_iter() {
            accounts.push(account.attributes.name);
        }
        
        current_page = resp.meta.pagination.current_page;
        total_pages = resp.meta.pagination.total_pages;
    }

    Ok(accounts)
}
