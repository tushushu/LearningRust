use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use std::ops::{AddAssign, Mul};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/predict")]
async fn predict(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[derive(Deserialize)]
struct LinearRegression<T> {
    coef: Vec<T>,
    intercept: T,
}

impl<T> LinearRegression<T>
where
    T: Mul<Output = T> + AddAssign + Copy,
{
    fn predict(&self, x: Vec<T>) -> T {
        let mut result = self.intercept;
        for (i, &coef) in self.coef.iter().enumerate() {
            result += coef * x[i];
        }
        result
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(predict))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
