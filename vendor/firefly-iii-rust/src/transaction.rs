use serde::{Serialize,Deserialize};
use std::borrow::Cow;
use crate::http::Method;
use crate::requests::Request;
use crate::response::*;

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Data {
    pub r#type: String,
    pub id: String,
    pub attributes: Attributes,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Attributes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_title: Option<String>,
    pub transactions: Vec<Transaction>,
}

#[derive(Deserialize, Default, Debug, Clone)]
pub struct Transaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_journal_id: Option<String>,
    pub r#type: String,
    pub date: String,
    pub amount: String,
    pub description: String,
    pub source_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    pub destination_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Create {
    pub group_title: Option<String>,
    pub transactions: Vec<CreateTransaction>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateTransaction {
    pub r#type: String, // withdrawal, deposit, transfer, reconciliation, opening balance 
    pub date: String,
    pub amount: String,
    pub description: String,
    pub source_id: Option<String>,
    pub source_name: Option<String>,
    pub destination_id: Option<String>,
    pub destination_name: Option<String>,
}

impl Request for Create {
    type Body = Create;
    type Response = Response<Data>;
    const METHOD: Method = Method::Post;
    fn endpoint(&self) -> Cow<str> {
        format!("/v1/transactions").into()
    }
    fn body(&self) -> Option<Self::Body> {
        Some(self.clone())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Get {
    pub id: String,
}

impl Request for Get {
    type Body = ();
    type Response = Response<Data>;
    const METHOD: Method = Method::Get;
    fn endpoint(&self) -> Cow<str> {
        format!("/v1/transactions/{}", self.id).into()
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
        format!("/v1/transactions/{}", self.id).into()
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
        let mut endpoint = "/v1/transactions".to_string();
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

