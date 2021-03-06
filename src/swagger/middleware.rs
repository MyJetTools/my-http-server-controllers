use std::sync::Arc;

use async_trait::async_trait;
use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpServerMiddleware, HttpServerRequestFlow,
    WebContentType,
};
use tokio::sync::Mutex;

use super::super::controllers::ControllersMiddleware;

pub struct SwaggerMiddleware {
    controllers: Arc<ControllersMiddleware>,
    title: String,
    version: String,
    swagger_json_ok_result: Mutex<Option<HttpOkResult>>,
}

impl SwaggerMiddleware {
    pub fn new(controllers: Arc<ControllersMiddleware>, title: String, version: String) -> Self {
        Self {
            controllers,
            title,
            version,
            swagger_json_ok_result: Mutex::new(None),
        }
    }
}

#[async_trait]
impl HttpServerMiddleware for SwaggerMiddleware {
    async fn handle_request(
        &self,
        ctx: &mut HttpContext,
        get_next: &mut HttpServerRequestFlow,
    ) -> Result<HttpOkResult, HttpFailResult> {
        let path = ctx.request.get_path_lower_case();

        if !path.starts_with("/swagger") {
            return get_next.next(ctx).await;
        }

        if path == "/swagger/index.html" {
            let result = HttpOkResult::Content {
                headers: None,
                content_type: Some(WebContentType::Html),
                content: super::resources::INDEX_PAGE.to_vec(),
            };
            return Ok(result);
        }

        if path == "/swagger/swagger-ui.css" {
            let result = HttpOkResult::Content {
                headers: None,
                content_type: Some(WebContentType::Css),
                content: super::resources::SWAGGER_UI_CSS.to_vec(),
            };
            return Ok(result);
        }

        if path == "/swagger/swagger-ui-bundle.js" {
            let result = HttpOkResult::Content {
                headers: None,
                content_type: Some(WebContentType::JavaScript),
                content: super::resources::SWAGGER_UI_BUNDLE_JS.to_vec(),
            };
            return Ok(result);
        }

        if path == "/swagger/swagger-ui-standalone-preset.js" {
            let result = HttpOkResult::Content {
                headers: None,
                content_type: Some(WebContentType::JavaScript),
                content: super::resources::SWAGGER_UI_STANDALONE_PRESET_JS.to_vec(),
            };
            return Ok(result);
        }

        if path == "/swagger/favicon-32x32.png" {
            let result = HttpOkResult::Content {
                headers: None,
                content_type: Some(WebContentType::Png),
                content: super::resources::FAVICON_32.to_vec(),
            };
            return Ok(result);
        }

        if path == "/swagger/favicon-16x16.png" {
            let result = HttpOkResult::Content {
                headers: None,
                content_type: Some(WebContentType::Png),
                content: super::resources::FAVICON_16.to_vec(),
            };
            return Ok(result);
        }

        let scheme = ctx.request.get_scheme();

        let host = ctx.request.get_host();

        if path == "/swagger" {
            let new_url = format!("{}://{}/swagger/index.html", scheme, host);
            return Ok(HttpOkResult::Redirect { url: new_url });
        }

        if path == "/swagger/v1/swagger.json" {
            let mut write_access = self.swagger_json_ok_result.lock().await;
            if let Some(result) = &*write_access {
                return Ok(result.clone());
            }

            *write_access = Some(HttpOkResult::Content {
                headers: None,
                content_type: Some(WebContentType::Json),
                content: super::swagger_json::builder::build(
                    self.controllers.as_ref(),
                    self.title.as_ref(),
                    self.version.as_ref(),
                    host,
                    scheme.as_ref(),
                ),
            });

            return Ok(write_access.as_ref().unwrap().clone());
        }

        let result =
            my_http_server::middlewares::files::get(format!("./wwwroot{}", path).as_str()).await;

        match result {
            Ok(content) => {
                let result = HttpOkResult::Content {
                    headers: None,
                    content_type: None,
                    content,
                };
                return Ok(result);
            }
            _ => {
                let new_url = format!("{}://{}/swagger/index.html", scheme, host);
                return Ok(HttpOkResult::Redirect { url: new_url });
            }
        }
    }
}
