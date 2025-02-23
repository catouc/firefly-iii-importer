use serde::Deserialize;

/// A generic response that wraps the common fields the
/// Firefly III API sends in all endpoints.
/// Usually used with the data types from the endpoints.
#[derive(Debug, Deserialize, Default)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Deserialize, Default)]
pub struct EmptyResponse {}

/// This is the struct with the default fields for a
/// paginated response, this is usuall the listing
/// calls, this is paired with the `Paginated` trait
/// that enables the `client.fetch_all` function to
/// return a vector of all resources.
#[derive(Debug, Deserialize, Default)]
pub struct PaginatedResponse<T> {
    pub data: T,
    pub meta: Meta,
}

#[derive(Debug, Deserialize, Default)]
pub struct Meta {
    #[serde(default)]
    pub pagination: Pagination,
}

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub total: u64,
    pub count: u64,
    pub per_page: u64,
    pub current_page: u64,
    pub total_pages: u64,
}

/// Enables pagination for the requests that require it
/// this way the page information is encoded in the request
/// struct.
pub trait Paginated {
    fn set_page(&mut self, page: u64);
    fn get_page(&self) -> u64;
    fn max_page(&self) -> u64;
}

