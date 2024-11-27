use crate::bank_data::Record;
use crate::firefly::account::{Account,create_account_if_not_exists};
use std::env;

#[derive(Debug)]
pub struct Transaction {
    pub r#type: TransactionType,
    pub source_account: Account,
    pub destination_account: Account,
    pub amount: f32,
    pub date: String,
    pub description: String,
}


impl TryFrom<&Record> for Transaction {
    type Error = Box<dyn std::error::Error>;
    fn try_from(record: &Record) -> Result<Self, Self::Error> {
       let tx_type = match record.amount.starts_with("-") {
           true => TransactionType::Withdrawal,
           false => TransactionType::Deposit,
       };

       let amount: f32 = record.amount.replace(",", "").replace("-", "").parse()?;

       let token = env::var("FIREFLY_TOKEN").unwrap();

       let (source_account, destination_account) = match tx_type {
           TransactionType::Withdrawal => {
               let source_account = create_account_if_not_exists(
                   &token,
                   &record.account,
                   &record.account,
                   "asset",
               )?;

                let destination_account = create_account_if_not_exists(
                   &token,
                   &record.name,
                   &record.counterparty,
                   "expense",
               )?;
                (source_account, destination_account)
           },
           TransactionType::Deposit => {
                let source_account = create_account_if_not_exists(
                   &token,
                   &record.name,
                   &record.counterparty,
                   "expense",
               )?;

               let destination_account = create_account_if_not_exists(
                   &token,
                   &record.account,
                   &record.account,
                   "asset",
               )?;
                (source_account, destination_account)
           },
       };

       let tx = Transaction{
           r#type: tx_type,
           amount,
           date: record.date.clone(),
           description: record.description.clone(),
           source_account,
           destination_account,
       };
       Ok(tx)
   } 
}

#[derive(Debug)]
pub enum TransactionType {
    Withdrawal,
    Deposit,
}

pub fn create(token: &str, tx: &Transaction) -> Result<(), Box<dyn std::error::Error>> {
    let tx_type: &str = match tx.r#type {
        TransactionType::Withdrawal => "withdrawal",
        TransactionType::Deposit => "deposit",
    };

    let mut description = "No description";
    if tx.description != "" {
        description = &tx.description;
    }
    let _ = ureq::post("http://localhost:8080/api/v1/transactions")
        .set("Authorization", &format!("Bearer {}", token))
        .set("accept", "application/vnd.api+json")
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "transactions": [{
                "type": tx_type,
                "date": tx.date,
                "amount": tx.amount,
                "description": description,
                "source_id": tx.source_account.id,
                "destination_id": tx.destination_account.id,
            }]
            }))?;
    Ok(())
}
