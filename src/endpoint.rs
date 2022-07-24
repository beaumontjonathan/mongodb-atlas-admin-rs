use async_trait::async_trait;
use digest_auth::AuthContext;
use http::{
    header::{AUTHORIZATION, CONTENT_TYPE, WWW_AUTHENTICATE},
    HeaderValue, Method,
};
use reqwest::{RequestBuilder, Response};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[async_trait]
pub trait Endpoint {
    type Data;
    type Error: std::error::Error;

    fn method() -> Method;

    fn path(&self) -> Cow<'static, str>;

    fn body(&self) -> Result<Option<Vec<u8>>, Self::Error> {
        Ok(None)
    }

    async fn get_data(res: Response) -> Result<Self::Data, Self::Error>;
}

pub trait EndpointError {}

fn apply_data(request: RequestBuilder, data: Option<Vec<u8>>) -> RequestBuilder {
    if let Some(vec) = data {
        request
            .header(CONTENT_TYPE, HeaderValue::from_static("application/json"))
            .body(vec)
    } else {
        request
    }
}

pub struct Client {
    username: String,
    password: String,
}

impl Client {
    pub fn new<U: Into<String>, P: Into<String>>(username: U, password: P) -> Self {
        Self {
            username: username.into(),
            password: password.into(),
        }
    }

    pub async fn execute_endpoint<T: Endpoint + 'static>(
        &self,
        a: T,
    ) -> Result<T::Data, Box<dyn std::error::Error>> {
        let path = T::path(&a);

        let uri = format!("/api/atlas/v1.0{}", &path);
        let url = format!("https://cloud.mongodb.com{}", &uri);

        let resp = reqwest::get(&url).await?;

        let www_authenticate = resp.headers().get(WWW_AUTHENTICATE).unwrap().to_str()?;
        let body = T::body(&a)?;
        let context = AuthContext::new_with_method(
            &self.username,
            &self.password,
            uri,
            body.as_ref(),
            T::method().into(),
        );

        let mut prompt = digest_auth::parse(www_authenticate).unwrap();
        let answer = prompt.respond(&context).unwrap().to_string();
        let client = reqwest::Client::new();
        let req = client
            .request(T::method(), url)
            .header(AUTHORIZATION, answer);

        let req = apply_data(req, body);

        let resp = req.send().await?;

        let result = T::get_data(resp).await?;

        Ok(result)
    }
}

#[derive(Debug, Serialize, Deserialize, thiserror::Error)]
#[error("Error {error}: {reason}")]
pub struct RequestError {
    detail: String,
    error: i32,
    #[serde(rename = "errorCode")]
    error_code: String,
    parameters: Vec<String>,
    reason: String,
}

#[derive(Debug)]
pub struct Pagination {
    items_per_page: i32,
    page_number: i32,
}

impl Pagination {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn items_per_page(self, items_per_page: i32) -> Self {
        Self {
            items_per_page,
            ..self
        }
    }

    pub fn page_number(self, page_number: i32) -> Self {
        Self {
            page_number,
            ..self
        }
    }

    pub fn to_query_params(&self) -> String {
        format!(
            "includeCount=true&itemsPerPage={}&pageNum={}",
            self.items_per_page, self.page_number
        )
    }
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            items_per_page: 100,
            page_number: 1,
        }
    }
}
