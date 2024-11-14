#[derive(Debug)]
pub struct Transaction {
    //pub r#type: TransactionType,
    pub source_account_id: u32,
    pub destination_account_id: u32,
    pub amount: f32,
    pub date: String,
    pub description: String,
}


/*
impl TryFrom<&Record> for Transaction {
    type Error = Box<dyn std::error::Error>;
    fn try_from(record: &Record) -> Result<Self, Self::Error> {
       let tx_type = match record.amount.starts_with("-") {
           true => TransactionType::Withdrawal,
           false => TransactionType::Deposit,
       };

       let amount: f32 = record.amount.replace(",", "").parse()?;

       let token = env::var("FIREFLY_TOKEN").unwrap();

       let (source_account_id, destination_account_id) = match tx_type {
           TransactionType::Withdrawal => {
               if let Ok(account_id) = create_account_if_not_exists(
                   &token,
                   &record.account,
                   &record.account,
                   "expense",
                ) {
                   if let Ok(dest_acc_id) = create_account_if_not_exists(
                        &token,
                       &record.name,
                       &record.counterparty,
                       "expense",
                    ){
                       (account_id, dest_acc_id)
                   } else {
                       (0, 0)
                   }
               } else {
                   (0, 0)
               }
           },
           TransactionType::Deposit => {
               if let Ok(account_id) = create_account_if_not_exists(
                   &token,
                   &record.name,
                   &record.counterparty,
                   "expense",
                ) {
                   if let Ok(dest_acc_id) = create_account_if_not_exists(
                        &token,
                        &record.account,
                        &record.account,
                        "expense",
                    ) {
                       (account_id, dest_acc_id)
                   } else {
                        (0,0)
                   }
               } else {
                   (0, 0)
               }

           },
       };

       let tx = Transaction{
           r#type: tx_type,
           amount,
           date: record.date.clone(),
           description: record.description.clone(),
           source_account_id,
           destination_account_id,
       };
       Ok(tx)
   } 
}

#[derive(Debug)]
pub enum TransactionType {
    Withdrawal,
    Deposit,
}
*/
/*
pub fn create_transaction(token: &str, tx: &Transaction) -> Result<(), Box<dyn std::error::Error>> {
    let tx_type: &str = match tx.r#type {
        TransactionType::Withdrawal => "withdrawal",
        TransactionType::Deposit => "deposit",
    };

    println!("{:#?}", ureq::json!({
        "data": {
            "transactions": [{
                "type": tx_type,
                "date": tx.date,
                "amount": tx.amount,
                "description": tx.description,
                "source_id": tx.source_account_id,
                 "source_name": "Stuff And Things",
                "destination_name": tx.description,
               "destination_id": tx.destination_account_id,
            }]
        }}));

    let resp: String = ureq::post("https://accounting.boeschen.me/api/v1/transactions")
        .set("Authorization", &format!("Bearer {}", token))
        .set("accept", "application/vnd.api+json")
        .set("Content-Type", "application/json")
        .send_json(ureq::json!({
            "data": {
            "transactions": [{
                "type": tx_type,
                "date": tx.date,
                "amount": tx.amount,
                "description": tx.description,
//                "source_id": tx.source_account_id,
 //               "destination_id": tx.destination_account_id,
                "source_name": "Stuff And Things",
                "destination_name": tx.description,
            }]
            }}))?
        .into_string()?;

    println!("{:#?}", resp);

    Ok(())
}
*/
