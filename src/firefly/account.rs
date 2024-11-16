use serde::Deserialize;
pub type AccountID = u32;

#[derive(Deserialize)]
struct AccountResponse {
    data: Account,
}

pub fn create_account_if_not_exists(
    token: &str,
    account_name: &str,
    account_iban: &str,
    account_type: &str,
) -> Result<Account, Box<dyn std::error::Error>> {
    let accounts = list_all(token).unwrap();
    if let Some(account) = accounts.iter().find(|a| {
        if let Some(iban) = &a.attributes.iban {
            return iban == account_iban
        }
        a.attributes.name == account_name
    }) {
        return Ok(account.clone())
    } else {
        return create_account(token, account_name, account_iban, account_type)
    }
}

pub fn create_account(
    token: &str,
    account_name: &str,
    account_iban: &str,
    account_type: &str,
) -> Result<Account, Box<dyn std::error::Error>> {
    let request_body = match account_type {
        "asset" => ureq::json!({
            "name": format!("{} ({})", account_name, account_iban),
            "type": account_type,
            "iban": account_iban,
            "account_role": "defaultAsset",
        }),
        _ => ureq::json!({
            "name": format!("{} ({})", account_name, account_iban),
            "type": account_type,
            "iban": account_iban,
        }),
    };

    let resp: AccountResponse = ureq::post(
        "http://localhost:8080/api/v1/accounts"
    )
        .set("Authorization", &format!("Bearer {}", token))
        .set("accept", "application/vnd.api+json")
        .set("Content-Type", "application/json")
        .send_json(request_body)?
        .into_json()?;
    Ok(resp.data)
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

#[derive(Deserialize, Debug, Clone)]
pub struct Account {
    pub r#type: String,
    pub id: String,
    pub attributes: AccountAttributes,
}

#[derive(Deserialize, Debug, Clone)]
pub struct AccountAttributes {
    pub active: bool,
    pub name: String,
    pub iban: Option<String>,
    pub r#type: String,
}

pub fn list_all(token: &str) -> Result<Vec<Account>, Box<dyn std::error::Error>> {
    let mut current_page = 0;
    let mut total_pages = 1;
    let mut accounts: Vec<Account> = Vec::new();

    while current_page < total_pages {
        let resp: ListAccountResponse = ureq::get("http://localhost:8080/api/v1/accounts")
            .set("Authorization", &format!("Bearer {}", token))
            .set("accept", "application/vnd.api+json")
            .set("Content-Type", "application/json")
            .call()?
            .into_json()?;


        for account in resp.data.into_iter() {
            accounts.push(account);
        }
        
        current_page = resp.meta.pagination.current_page;
        total_pages = resp.meta.pagination.total_pages;
    }

    Ok(accounts)
}

