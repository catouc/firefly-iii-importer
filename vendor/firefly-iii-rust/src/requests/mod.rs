use serde::Serialize;
use serde::de::DeserializeOwned;
use std::borrow::Cow;
use std::collections::HashMap;
use crate::http::Method;

/// This trait is the generic implementation for every request
/// we make to the Firefly III API, you can set the Body to `()`
/// in case the request does not send a body.
/// Responses are in the response module.
pub trait Request {
    type Body: Serialize + std::fmt::Debug;
    type Response: DeserializeOwned + Default;
    const METHOD: Method;
    fn endpoint(&self) -> Cow<str>;
    
    fn headers(&self) -> HashMap<String, String> {
        HashMap::new()
    }

    fn body(&self) -> Option<Self::Body> {
        None
    }
}

