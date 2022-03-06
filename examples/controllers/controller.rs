
#[controller]
mod HelloController {
    use my_http_server::{HttpOkResult, HttpOutput, WebContentType, HttpContext, HttpFailResult};

    
    #[get(route = "/get", controller_name = "HelloController")]
    pub fn get_action(_ctx: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let output = HttpOutput::Content { 
            headers: None,
            content_type: Some(WebContentType::Text),
            content: "GET!".as_bytes().to_vec(),
        }; 
        
        return Ok(HttpOkResult {
            write_telemetry: false,
            output,
        });
    }

    #[post(route = "/post", controller_name = "HelloController")]
    pub fn post_action(_ctx: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let output = HttpOutput::Content { 
            headers: None,
            content_type: Some(WebContentType::Text),
            content: "POST!".as_bytes().to_vec(),
        }; 
        
        return Ok(HttpOkResult {
            write_telemetry: false,
            output,
        });
    }

    #[put(route = "/put", controller_name = "HelloController")]
    pub fn post_action(_ctx: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let output = HttpOutput::Content { 
            headers: None,
            content_type: Some(WebContentType::Text),
            content: "PUT!".as_bytes().to_vec(),
        }; 
        
        return Ok(HttpOkResult {
            write_telemetry: false,
            output,
        });
    }

    #[delete(route = "/delete", controller_name = "HelloController")]
    pub fn post_action(_ctx: &mut HttpContext) -> Result<HttpOkResult, HttpFailResult> {
        let output = HttpOutput::Content { 
            headers: None,
            content_type: Some(WebContentType::Text),
            content: "DELETE!".as_bytes().to_vec(),
        }; 
        
        return Ok(HttpOkResult {
            write_telemetry: false,
            output,
        });
    }
}