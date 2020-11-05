#[cfg(test)]
#[macro_use]
extern crate serial_test;

#[macro_use]
extern crate log;

use actix_web::middleware::{Compress, DefaultHeaders, Logger};
use actix_web::{web, App, HttpServer};

mod error;
mod handler;

fn get_port() -> u32 {
    std::env::var("PORT")
        .ok()
        .and_then(|value| value.parse::<u32>().ok())
        .unwrap_or(3200)
}

fn get_address() -> String {
    std::env::var("ADDRESS").unwrap_or("localhost".into())
}

macro_rules! create_app {
    () => {
        App::new().app_data(web::JsonConfig::default().error_handler(error::json_error_handler))
    };
}

macro_rules! bind_services {
    ($app: expr) => {
        $app.service(handler::status::handler)
    };
}

#[cfg(not(tarpaulin_include))]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let binding = format!("{}:{}", get_address(), get_port());
    HttpServer::new(|| {
        bind_services!(create_app!())
            .wrap(DefaultHeaders::new().header("X-Version", env!("CARGO_PKG_VERSION")))
            .wrap(Compress::default())
            .wrap(Logger::default())
    })
    .bind(binding.as_str())?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::Request;
    use actix_web::dev::ServiceResponse;
    use actix_web::{test, App};

    pub async fn execute_request(req: Request) -> ServiceResponse {
        let mut app = test::init_service(bind_services!(create_app!())).await;
        test::call_service(&mut app, req).await
    }

    #[test]
    #[serial]
    fn test_get_address() {
        let _address = env_test_util::TempEnvVar::new("ADDRESS");
        assert_eq!(get_address(), "localhost");
        let _address = _address.with("something");
        assert_eq!(get_address(), "something");
    }

    #[test]
    #[serial]
    fn test_get_port() {
        let _port = env_test_util::TempEnvVar::new("PORT");
        assert_eq!(get_port(), 3200);
        let _port = _port.with("1234");
        assert_eq!(get_port(), 1234);
    }
}
