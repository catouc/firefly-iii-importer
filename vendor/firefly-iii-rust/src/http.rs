/// This module is basically just a crutch to make my Request type
/// less reliant on random strings. It does not support the entire
/// HTTP standard, only the subset that Firefly III cares about.

/// Method is the enum of HTTP method that we require for our
/// interaction with the Firefly III API.
pub enum Method {
    Head,
    Get,
    Post,
    Put,
    Delete,
}

