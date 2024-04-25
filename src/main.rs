use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
//use hyper::Client;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello {}!", &name))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    // println!("in echo: {}", req_body);
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
