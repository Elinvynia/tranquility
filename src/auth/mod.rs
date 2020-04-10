use crate::error::Error;
use async_trait::async_trait;
use reqwest::Response;

pub mod basic;

///The trait handling authentication and get requests for different authentication methods.
#[async_trait]
pub trait Auth {
    async fn login(&self) -> Result<String, Error> {
        Err(Error::Custom("Not implemented.".to_string()))
    }

    async fn get(&self, _route: &str, _key: &str, _user_agent: &str) -> Result<Response, Error> {
        Err(Error::Custom("Not implemented.".to_string()))
    }
}
