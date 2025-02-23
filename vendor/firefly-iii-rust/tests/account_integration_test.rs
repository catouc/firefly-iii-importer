/// These tests are all built to be fully idempotent on success.
/// The failure case can leave resources behind since we're going
/// to delete the database in any case.
/// To avoid collisions, we should never assert internal IDs and
/// all object names need to have a random prefix.
/// Additionally we currently require the token to be exposed via
/// FIREFLY_III_ACCESS_TOKEN in the environment.

use rand::distributions::{Alphanumeric, DistString};
use firefly_iii_rust::client;
use firefly_iii_rust::account::{Create, List, Delete};

#[test]
fn account_expense_create_list_delete() {
    let token = std::env::var("FIREFLY_III_ACCESS_TOKEN")
        .expect("FIREFLY_III_ACCESS_TOKEN needs to be set in the environment.");
    let base_url = std::env::var("FIREFLY_III_BASE_URL")
        .expect("FIREFLY_III_BASE_URL needs to be set in the environment.");
    let client = client::new(&base_url, &token);

    for i in 0..60 {
        let mut random_name: String = Alphanumeric.sample_string(
            &mut rand::thread_rng(), 32,
        );
        random_name.push(i.into());

        let mut create_request = Create::default();
        create_request.name = random_name;
        create_request.r#type = "expense".into();
        let create_response = client.call(create_request).unwrap();

        assert_eq!(create_response.data.attributes.r#type, "expense");
    }
    let accounts = client.fetch_all(List{
        current_page: 1,
        total_pages: 2,
    }).unwrap();
    accounts.iter().for_each(|a| {
        client.call(Delete{
            id: a.id.clone(),
        }).expect("Delete call did not successfully exit");
    });
}

