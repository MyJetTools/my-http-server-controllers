use std::sync::Arc;

use async_trait::async_trait;
use my_http_server::{
    HttpContext, HttpFailResult, HttpOkResult, HttpOutput, HttpServerMiddleware,
    HttpServerRequestFlow, WebContentType,
};

use super::super::controllers::ControllersMiddleware;

pub struct SwaggerMiddleware {
    controllers: Arc<ControllersMiddleware>,
    title: String,
    version: String,
    authorization_enabled: bool,
}

impl SwaggerMiddleware {
    pub fn new(controllers: Arc<ControllersMiddleware>, title: String, version: String) -> Self {
        Self {
            controllers,
            title,
            version,
            authorization_enabled: false,
        }
    }

    pub fn enable_athorization(&mut self) {
        self.authorization_enabled = true;
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
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Html),
                content: super::resources::INDEX_PAGE.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/swagger-ui.css" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Css),
                content: super::resources::SWAGGER_UI_CSS.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/swagger-ui-bundle.js" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::JavaScript),
                content: super::resources::SWAGGER_UI_BUNDLE_JS.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/swagger-ui-standalone-preset.js" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::JavaScript),
                content: super::resources::SWAGGER_UI_STANDALONE_PRESET_JS.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/favicon-32x32.png" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Png),
                content: super::resources::FAVICON_32.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/favicon-16x16.png" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Png),
                content: super::resources::FAVICON_16.to_vec(),
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        let scheme = ctx.request.get_scheme();

        let host = ctx.request.get_host();

        if path == "/swagger" {
            let new_url = format!("{}://{}/swagger/index.html", scheme, host);

            let output = HttpOutput::Redirect { url: new_url };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if path == "/swagger/v1/swagger.json" {
            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Json),
                content: super::swagger_json::builder::build(
                    self.controllers.as_ref(),
                    self.title.as_ref(),
                    self.version.as_ref(),
                    host,
                    scheme.as_ref(),
                    self.authorization_enabled,
                ),
            };

            let result = HttpOkResult {
                write_telemetry: false,
                output,
            };

            return Ok(result);
        }

        let result =
            my_http_server::middlewares::files::get(format!("./wwwroot{}", path).as_str()).await;

        match result {
            Ok(content) => {
                let output = HttpOutput::Content {
                    headers: None,
                    content_type: None,
                    content,
                };
                return Ok(HttpOkResult {
                    write_telemetry: false,
                    output,
                });
            }
            _ => {
                let new_url = format!("{}://{}/swagger/index.html", scheme, host);
                let output = HttpOutput::Redirect { url: new_url };
                return Ok(HttpOkResult {
                    write_telemetry: false,
                    output,
                });
            }
        }
    }
}
