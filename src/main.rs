use std::io;
use std::env;
use csv;
use bank_data::Record;

pub mod firefly;
pub mod bank_data;


fn main() {
    let token = &env::var("FIREFLY_TOKEN").unwrap();
    let accounts = firefly::list_accounts_by_name(token).unwrap();
    println!("{:#?}", accounts)
    // let _ = firefly::create_account(token, "Test From Rust 2", "", "expense").unwrap();

    /*
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        let tx = firefly::Transaction::try_from(&record).unwrap();

        let token = &env::var("FIREFLY_TOKEN").unwrap();
        firefly::create_transaction(
            token,
            &tx)
            .unwrap()
    }
    */
}
