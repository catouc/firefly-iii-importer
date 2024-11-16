use std::io;
use std::env;
use csv;
use bank_data::Record;
use firefly::transaction::Transaction;

pub mod firefly;
pub mod bank_data;


fn main() {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        let tx = Transaction::try_from(&record).unwrap();

        let token = &env::var("FIREFLY_TOKEN").unwrap();
        firefly::transaction::create(token,&tx).unwrap()
    }
}
