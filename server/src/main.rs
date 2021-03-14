use actix_files::{NamedFile};
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest, Result};

async fn not_found(req: HttpRequest) -> Result<HttpResponse> {
    NamedFile::open("./public/index.html")?.into_response(&req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/", "./public")
            .index_file("index.html")
        ).default_service(web::to(not_found))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
