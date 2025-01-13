use std::collections::HashMap;
use std::io;
use csv;
use bank_data::Record;

pub mod bank_data;

fn main() {
    let firefly_client = firefly_iii_rust::client::new(
        &std::env::var("FIREFLY_III_BASE_URL")
            .expect("FIREFLY_III_BASE_URL needs to be set."),
        &std::env::var("FIREFLY_III_ACCESS_TOKEN")
            .expect("FIREFLY_III_ACCESS_TOKEN needs to be set."),
    );

    let account_list = firefly_client.fetch_all(firefly_iii_rust::account::List{
        current_page: 1,
        total_pages: 2,
    }).expect("we need to get all the accounts");

    let mut accounts = HashMap::new();
    let mut iban_index = HashMap::new();

    let _ = account_list.iter().for_each(
        |a| {
            if let Some(name) = &a.attributes.name {
                accounts.insert(name.clone(), a.clone());
            }

            if let Some(iban) = &a.attributes.iban {
                iban_index.insert(iban, a.clone());
            }
        }
    );

    let mut rdr = csv::Reader::from_reader(io::stdin());
    let records: Vec<Record> = rdr.deserialize()
        .filter_map(|r| r.map_err(|e| eprintln!("failed to parse record: {}", e)).ok())
        .collect();

    for record in records {
        let mut account_name = record.account.clone();
        if accounts.get(&record.account).is_none()
            && iban_index.get(&record.account).is_none()
        {
                let mut acc_request_create = firefly_iii_rust::account::Create::default();
                acc_request_create.name = record.account.clone();
                acc_request_create.r#type = "asset".to_string();
                acc_request_create.account_role = Some("defaultAsset".to_string());
                acc_request_create.iban = Some(record.account.clone());
                let _ = firefly_client.call(acc_request_create);
        } else if let Some(account) = iban_index.get(&record.account) {
            if let Some(name) = &account.attributes.name {
                account_name = name.to_string();
            }
        }

        let mut counterparty_name = format!("{}", &record.name);
        if record.counterparty == "" {
            counterparty_name.push_str(&record.counterparty);
        }
        if accounts.get(&counterparty_name).is_none() {
                let mut acc_request_create = firefly_iii_rust::account::Create::default();
                acc_request_create.name = record.account.clone();
                acc_request_create.r#type = "asset".to_string();
                acc_request_create.account_role = Some("defaultAsset".to_string());
                acc_request_create.iban = Some(record.account.clone());
                let _ = firefly_client.call(acc_request_create);
        };

        let tx_type: String = match record.amount.starts_with("-") {
            true => "withdrawal".to_string(),
            false => "deposit".to_string(),
        };

        let absolute_amount = match record.amount.starts_with("-") {
            true => record.amount.replace(",", "").replace("-", ""),
            false => record.amount,
        };

        let mut description = record.description.clone();
        if record.description == "" {
            description = "Imported".to_string();    
        }

        let mut tx = firefly_iii_rust::transaction::CreateTransaction{
            amount: absolute_amount,
            description: description.to_string(),
            date: record.date,
            r#type: tx_type,
            destination_id: None,
            destination_name: None,
            source_id: None,
            source_name: None,
        };

        println!("{}", account_name);

        if tx.r#type == "withdrawal" {
            tx.source_name = Some(account_name);
            tx.destination_name = Some(counterparty_name);
        } else {
            tx.source_name = Some(counterparty_name);
            tx.destination_name = Some(account_name);
        }

        let _ = firefly_client.call(firefly_iii_rust::transaction::Create{
            group_title: None,
            transactions: vec![tx]
        }).expect("transaction failed to create");
    }
}

