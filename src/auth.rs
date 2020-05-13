//! Contains the auth methods you can use for the client.

use crate::{client::route::Route, error::Error, model::misc::Params};
use async_trait::async_trait;
use reqwest::Response;

pub mod basic;

///The trait handling authentication and GET requests for different authentication methods.
#[async_trait]
pub trait Auth {
    /// Perfors the authentication via the reddit API.
    async fn login(&self) -> Result<String, Error> {
        unimplemented!()
    }

    /// Performs a GET request using the auth method.
    async fn get(
        &self,
        _route: Route,
        _key: &str,
        _user_agent: &str,
        _params: &Params,
    ) -> Result<Response, Error> {
        unimplemented!()
    }

    /// Performs a POST request using the auth method.
    async fn post(
        &self,
        _route: Route,
        _key: &str,
        _user_agent: &str,
        _params: &Params,
    ) -> Result<Response, Error> {
        unimplemented!()
    }
}
