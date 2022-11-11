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
}

impl SwaggerMiddleware {
    pub fn new(controllers: Arc<ControllersMiddleware>, title: String, version: String) -> Self {
        Self {
            controllers,
            title,
            version,
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
        if ctx.request.http_path.is_root() {
            return get_next.next(ctx).await;
        }

        if let Some(value) = ctx.request.http_path.get_segment_value_as_str(0) {
            if value != "swagger" {
                return get_next.next(ctx).await;
            }
        }

        if ctx.request.http_path.segments_amount() == 1 {
            let scheme = ctx.request.get_scheme();
            let host = ctx.request.get_host();
            let new_url = format!("{}://{}/swagger/index.html", scheme, host);

            let output = HttpOutput::Redirect {
                url: new_url,
                permanent: false,
            };
            return Ok(HttpOkResult {
                write_telemetry: false,
                output,
            });
        }

        if ctx
            .request
            .http_path
            .has_value_at_index_case_insensitive(1, "index.html")
        {
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

        if ctx
            .request
            .http_path
            .has_value_at_index_case_insensitive(1, "swagger-ui.css")
        {
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

        if ctx
            .request
            .http_path
            .has_value_at_index_case_insensitive(1, "swagger-ui-bundle.js")
        {
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

        if ctx
            .request
            .http_path
            .has_value_at_index_case_insensitive(1, "swagger-ui-standalone-preset.js")
        {
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

        if ctx
            .request
            .http_path
            .has_value_at_index_case_insensitive(1, "favicon-32x32.png")
        {
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

        if ctx
            .request
            .http_path
            .has_value_at_index_case_insensitive(1, "favicon-16x16.png")
        {
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

        if ctx
            .request
            .http_path
            .has_values_at_index_case_insensitive(1, &["v1", "swagger.yaml"])
        {
            let scheme = ctx.request.get_scheme();
            let host = ctx.request.get_host();

            let output = HttpOutput::Content {
                headers: None,
                content_type: Some(WebContentType::Json),
                content: super::swagger_yaml::builder::build(
                    self.controllers.as_ref(),
                    self.title.as_ref(),
                    self.version.as_ref(),
                    host,
                    scheme.as_ref(),
                ),
            };

            let result = HttpOkResult {
                write_telemetry: false,
                output,
            };

            return Ok(result);
        }

        get_next.next(ctx).await
    }
}
