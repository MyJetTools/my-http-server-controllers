#[macro_use] extern crate macros;

use std::{sync::Arc, net::SocketAddr, time::Duration};

use my_http_server::MyHttpServer;
use my_http_server_controllers::controllers::ControllersMiddleware;

mod app;
mod controllers;

#[tokio::main]
async fn main() {
    let app = crate::app::AppContext::new();
    let app = Arc::new(app);

    let mut http_server: MyHttpServer = MyHttpServer::new(SocketAddr::from(([0, 0, 0, 0], 5000)));

    // controllers
    let mut controllers = ControllersMiddleware::new();

    let hello_controller = Arc::new(controllers::HelloController::new());
    
    controllers.register_get_action(hello_controller.clone());
    controllers.register_post_action(hello_controller.clone());
    controllers.register_put_action(hello_controller.clone());
    controllers.register_delete_action(hello_controller.clone());

    // middlewares
    http_server.add_middleware(Arc::new(controllers));

    http_server.start(app);

    loop {
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}
