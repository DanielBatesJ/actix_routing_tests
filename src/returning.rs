use super::error::CustomError;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Result};

// Return Examples
/// Our custom Data type we will use to showcase impl Responder with web::Json.
#[derive(Serialize, Debug)]
struct A {
    pub a: i32,
    pub b: String,
}
/// Our custom data type that we will use to demonstrate manual implementation.
#[derive(Serialize, Debug)]
struct B {
    pub a: i32,
    pub b: String,
}

/// A custom implementation of Responder for A.
impl Responder for A {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _req: &HttpRequest) -> HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::json())
            .body(body)
    }
}

/// Because we have Responder implemented for A, we can now just return our type.
#[get("/custom_responder")]
async fn custom_responder() -> A {
    A {
        a: 123,
        b: String::from("Here we are testing our own implementation of Responder."),
    }
}

/// Testing an example that implements Responder for us.
#[get("/json_responder")]
async fn json_responder() -> impl Responder {
    web::Json(B {
        a: 123,
        b: String::from("Here we test web::Json's built-in Responder implementation."),
    })
}

#[get("/manual_response")]
async fn manual_response() -> HttpResponse {
    let maybe = Some(123);
    if let Some(a) = maybe {
        HttpResponse::Ok().json(A {
            a,
            b: String::from("Here we are manually returning an HttpResponse"),
        })
    } else {
        HttpResponse::NotFound().body("Value for A:a not found.")
    }
}

/// Custom error handling example. Will get an error when it goes to get a number.
/// NOTE: We could rewrite this to explicitly return `Result<a>`
/// Alternatively, without changing the current return signature, we could return Ok(web::Json(B)).
#[get("/custom_error")]
async fn custom_error() -> Result<impl Responder> {
    Ok(A {
        a: get_num().await?,
        b: String::from("Hello"),
    })
}

/// Helper function to test errors.
async fn get_num() -> Result<i32, CustomError> {
    Err(CustomError::NotFound)
}
