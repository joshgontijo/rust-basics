use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    let server = HttpServer::new(|| {
        App::new()
            .route("/hey", web::get().to(manual_hello))
            .service(hello)
            .service(echo)
    })
        .bind(("127.0.0.1", port))?
        .run();

    println!("Server started on {port}");
    server.await

}