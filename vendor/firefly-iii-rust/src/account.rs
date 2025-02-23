use serde::{Serialize,Deserialize};
use std::borrow::Cow;
use crate::http::Method;
use crate::requests::Request;
use crate::response::*;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Data {
    pub attributes: Attributes,
    pub id: String,
    pub r#type: String,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Attributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_decimal_places: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_symbol: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_balance_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_debt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_net_worth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_payment_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balance_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u64>,
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_balance: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Get {
    id: String,
}

impl Request for Get {
    type Body = ();
    type Response = Response<Data>;
    const METHOD: Method = Method::Get;
    fn endpoint(&self) -> Cow<str> {
        format!("/v1/accounts/{}", self.id).into()
    }
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
pub struct Create {
    pub name: String,
    pub r#type: String, // should be an enum
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_balance_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub virtual_balance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_net_worth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_card_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_payment_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub liability_direction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interest_period: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

impl Request for Create {
    type Body = Create;
    type Response = Response<Data>;
    const METHOD: Method = Method::Post;
    fn endpoint(&self) -> Cow<str> {
        format!("/v1/accounts").into()
    }
    fn body(&self) -> Option<Self::Body> {
        Some(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Delete {
    pub id: String,
}

impl Request for Delete {
    type Body = ();
    type Response = EmptyResponse;
    const METHOD: Method = Method::Delete;
    fn endpoint(&self) -> Cow<str> {
        format!("/v1/accounts/{}", self.id).into()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct List {
    pub current_page: u64,
    pub total_pages: u64,
}

impl Request for List {
    type Body = ();
    type Response = PaginatedResponse<Vec<Data>>;
    const METHOD: Method = Method::Get;
    fn endpoint(&self) -> Cow<str> {
        let mut endpoint = "/v1/accounts".to_string();
        endpoint.push_str("?");
        endpoint.push_str(&format!("page={}", self.current_page));
        endpoint.push_str("&");
        endpoint.push_str("limit=50");
        endpoint.into()
    }
}

impl Paginated for List {
    fn set_page(&mut self, page: u64) {
        self.current_page = page;
    }

    fn get_page(&self) -> u64 {
        self.current_page
    }

    fn max_page(&self) -> u64 {
        self.total_pages
    }
}

