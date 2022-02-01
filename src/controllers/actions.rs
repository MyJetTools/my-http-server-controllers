use async_trait::async_trait;
use my_http_server::{HttpContext, HttpFailResult, HttpOkResult};

use super::documentation::HttpActionDescription;

#[async_trait]
pub trait GetAction {
    async fn handle_request(&self, ctx: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult>;
    fn get_description(&self) -> Option<HttpActionDescription>;
    fn get_route(&self) -> &str;
}

#[async_trait]
pub trait PostAction {
    async fn handle_request(&self, ctx: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult>;
    fn get_description(&self) -> Option<HttpActionDescription>;
    fn get_route(&self) -> &str;
}

#[async_trait]
pub trait PutAction {
    async fn handle_request(&self, ctx: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult>;
    fn get_description(&self) -> Option<HttpActionDescription>;
    fn get_route(&self) -> &str;
}

#[async_trait]
pub trait DeleteAction {
    async fn handle_request(&self, ctx: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult>;
    fn get_description(&self) -> Option<HttpActionDescription>;
    fn get_route(&self) -> &str;
}
