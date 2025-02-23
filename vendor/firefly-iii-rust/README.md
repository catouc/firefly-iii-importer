# Firefly III Rust API wrapper

This is a uncomplete API wrapper for [Firefly III](https://www.firefly-iii.org/).
It is built against the API documentation [here](https://api-docs.firefly-iii.org).

## Usage

The crate exposes every endpoint under a module, currently I have
* account
* transaction

There is a generic client that consumes the request types from the endpoint modules:

```rust
use firefly_iii_rust::account::{Create, Delete};

let token = std::env::var("FIREFLY_III_ACCESS_TOKEN")
    .expect("FIREFLY_III_ACCESS_TOKEN needs to be set in the environment.");
let base_url = std::env::var("FIREFLY_III_BASE_URL")
    .expect("FIREFLY_III_BASE_URL needs to be set in the environment.");
let client = client::new(&base_url, &token);
let account_create_request = account::Create::default();

let account = client.call(account_create_request).unwrap();

let _ = client.call(account::Delete{id: account.id});
```

