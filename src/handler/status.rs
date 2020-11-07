use actix_web::{get, HttpResponse, Responder};

#[get("/status")]
pub async fn handler() -> impl Responder {
    HttpResponse::NoContent().finish()
}

// LCOV_EXCL_START
#[cfg(test)]
mod tests {
    use crate::tests::execute_request;
    use actix_web::http::StatusCode;
    use actix_web::test;

    #[actix_rt::test]
    #[serial]
    async fn status_success() {
        let req = test::TestRequest::get().uri("/status").to_request();
        let res = execute_request(req).await;
        assert_eq!(res.status(), StatusCode::NO_CONTENT);
    }
}
// LCOV_EXCL_END
