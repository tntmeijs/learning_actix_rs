use actix_files::NamedFile;
use actix_web::{
    get,
    web,
    App,
    HttpRequest,
    HttpResponse,
    HttpServer,
    Result
};
use std::sync::Mutex;
use serde::{Serialize};

#[derive(Serialize, Clone)]
struct DataObject {
    id: String,
    data: String
}

struct InMemoryDatabase {
    data: Mutex<Vec<DataObject>>
}

impl InMemoryDatabase {
    pub fn new() -> Self {
        Self { data: Mutex::new(vec!()) }
    }

    pub fn all(&self) -> Vec<DataObject> {
        (&*self.data.lock().unwrap()).clone()
    }
}

#[get("/all")]
async fn list_all(data: web::Data<InMemoryDatabase>) -> Result<HttpResponse> {
    Ok(HttpResponse::Ok().json(&data.all()))
}

async fn not_found(req: HttpRequest) -> Result<HttpResponse> {
    NamedFile::open("./public/index.html")?.into_response(&req)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let database = web::Data::new(InMemoryDatabase::new());

    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .service(web::scope("/api")
                .service(list_all))
            .service(actix_files::Files::new("/", "./public").index_file("index.html"))
            .default_service(web::to(not_found))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
