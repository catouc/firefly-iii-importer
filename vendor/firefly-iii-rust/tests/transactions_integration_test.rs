/// These tests are all built to be fully idempotent on success.
/// The failure case can leave resources behind since we're going
/// to delete the database in any case.
/// To avoid collisions, we should never assert internal IDs and
/// all object names need to have a random prefix.
/// Additionally we currently require the token to be exposed via
/// FIREFLY_III_ACCESS_TOKEN in the environment.
///
use rand::distributions::{Alphanumeric, DistString};
use firefly_iii_rust::client;
use firefly_iii_rust::account;
use firefly_iii_rust::transaction;

#[test]
fn transaction_create_list_delete() {
    let token = std::env::var("FIREFLY_III_ACCESS_TOKEN")
        .expect("FIREFLY_III_ACCESS_TOKEN needs to be set in the environment.");
    let base_url = std::env::var("FIREFLY_III_BASE_URL")
        .expect("FIREFLY_III_BASE_URL needs to be set in the environment.");
    let client = client::new(&base_url, &token);

    let asset_random_name: String = Alphanumeric.sample_string(
        &mut rand::thread_rng(), 32,
    );
    let mut asset_account_create_request = account::Create::default();
    asset_account_create_request.name = asset_random_name;
    asset_account_create_request.r#type = "asset".into();
    asset_account_create_request.account_role = Some("defaultAsset".into());
    let asset_account = client.call(asset_account_create_request).unwrap();

    let expense_random_name: String = Alphanumeric.sample_string(
        &mut rand::thread_rng(), 32,
    );
    let mut expense_account_create_request = account::Create::default();
    expense_account_create_request.name = expense_random_name;
    expense_account_create_request.r#type = "expense".into();
    expense_account_create_request.account_role = Some("defaultAsset".into());
    let expense_account = client.call(expense_account_create_request).unwrap();

    let tx_deposit = client.call(transaction::Create{
        group_title: None,
        transactions: vec![transaction::CreateTransaction{
            amount: "100".into(),
            r#type: "deposit".into(),
            date: "2024-10-10T01:00:00Z".into(),
            description: "test".into(),
            destination_id: Some(asset_account.data.id.clone()),
            destination_name: None,
            source_id: Some(expense_account.data.id.clone()),
            source_name: None,
        }],
    }).unwrap();

    let tx_withdrawal = client.call(transaction::Create{
        group_title: None,
        transactions: vec![transaction::CreateTransaction{
            amount: "100".into(),
            r#type: "withdrawal".into(),
            date: "2024-10-10T01:00:00Z".into(),
            description: "test".into(),
            destination_id: Some(expense_account.data.id.clone()),
            destination_name: None,
            source_id: Some(asset_account.data.id.clone()),
            source_name: None,
        }],
    }).unwrap();

    let _ = client.call(transaction::Delete{id: tx_deposit.data.id}).unwrap();
    let _ = client.call(transaction::Delete{id: tx_withdrawal.data.id}).unwrap();
    let _ = client.call(account::Delete{id: expense_account.data.id}).unwrap();
    let _ = client.call(account::Delete{id: asset_account.data.id}).unwrap();
}

