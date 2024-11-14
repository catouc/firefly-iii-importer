use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Record {
    #[serde(alias = "Account")]
    pub account: String,
    #[serde(alias = "Counterparty")]
    pub counterparty: String,
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Description")]
    pub description: String,
    #[serde(alias = "Date")]
    pub date: String,
    #[serde(alias = "Amount")]
    pub amount: String,
    #[serde(alias = "Interest Date")]
    pub interest_date: String,
}

