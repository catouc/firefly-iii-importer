use crate::requests::*;
use crate::response::*;
use crate::error::Result;
use crate::http::Method;
use serde::de::DeserializeOwned;

/// The main entry point into this crate. Hold the access token
/// currently `base_url` needs to be post fixed with `/api` for
/// this library to actually work.
/// TODO: that should be fixed.
#[derive(Debug)]
pub struct Client {
    base_url: String,
    token: String,
    client: ureq::Agent,
}

pub fn new(base_url: &str, token: &str) -> Client {
    let client = ureq::Agent::new();
    Client{base_url: base_url.into(), token: token.into(), client}
}

impl Client {
    /// This is our generic calling function for the API, the error
    /// handling is somewhat ass since it doesn't use https://docs.rs/ureq/latest/ureq/enum.Error.html#examples
    /// because the error enum in this repository somehow caused an
    /// inifinite loop with the transformations.
    ///
    /// TODO: The error should properly parse the actually useful error
    /// responses from the API.
    pub fn call<R: Request>(&self, request: R) -> Result<R::Response> {
        let url = format!("{}{}", self.base_url, request.endpoint());
        let mut req = match R::METHOD {
            Method::Head => self.client.head(&url),
            Method::Get => self.client.get(&url),
            Method::Post => self.client.post(&url),
            Method::Put => self.client.put(&url),
            Method::Delete => self.client.delete(&url),
        };

        req = req.set("Authorization", &format!("Bearer {}", self.token));
        req = req.set("accept", "application/vnd.api+json");
        req = req.set("Content-Type", "application/json");

        for (k, v) in request.headers().iter() {
            req = req.set(k,v);
        }

        let resp = match request.body() {
            Some(body) => {
                req.send_json(body)
            },
            None => req.call(),
        };

        match resp {
            Ok(resp) => {
                if resp.status() == 204 {
                    Ok(R::Response::default()) 
                } else {
                    /*
                    let str_result = resp.into_string().unwrap();
                    println!("{:#?}", str_result);
                    Ok(R::Response::default())
                    */
                    let result: R::Response = resp.into_json()
                       .expect("json conversion failed on us");
                    Ok(result)
                }
            },
            Err(err) => {
                if let Some(body) = request.body() {
                    eprintln!("Body: {:#?}", body);
                }
                Err(err.into())
            },
        }
    }

    /// Consumes all pages of the API in sequence and assembles a flat Vec of
    /// all pages.
    pub fn fetch_all<R, T>(&self, mut request: R) -> Result<Vec<T>>
    where
        R: Request<Response = PaginatedResponse<Vec<T>>> + Paginated + Clone,
        T: DeserializeOwned,
    {
        let mut current_page = 1;
        let mut all_items = Vec::new();

        loop {
            request.set_page(current_page);
            let response = self.call(request.clone())?;
            all_items.extend(response.data.into_iter());

            if request.get_page() < request.max_page() {
                current_page += 1;
            } else {
                break; // we're done
            }
        }

        Ok(all_items)
    }
}

