use tokio::fs::File;
use std::path::PathBuf;
use actix_web::{App, HttpResponse, HttpServer, Responder, get};
use actix_web::{dev::Server, web};
use tokio::io::AsyncReadExt;
use crate::Cache;
use crate::net::StdTcpListener;
use crate::util::TestReport;
use crate::util::{Status, TestKey};

#[derive(Clone)]
struct AppState {
    cache: Cache,
    status: Status,
}

impl AppState {
    pub fn status(&self) -> &Status {
        &self.status
    }
}

pub fn run_http(listener: StdTcpListener, cache: Cache) -> Result<Server, std::io::Error> {
    let state = AppState {
        cache,
        status: Status::new(),
    };

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(status)
            .service(new)
            .service(test)
            .service(malicious_domains)
    })
    .listen(listener.0)?
    .run();

    Ok(server)
}

#[get("/status")]
async fn status(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.status())
}

#[get("/new")]
async fn new(data: web::Data<AppState>) -> impl Responder {
    let report = TestReport::new();
    let key = TestKey::new();

    data.cache.insert(&key, report).await;

    HttpResponse::Ok().json(key)
}

#[get("/test/{key}")]
async fn test(data: web::Data<AppState>, path: web::Path<TestKey>) -> impl Responder {
    let key = path.into_inner();

    let value = data.cache.get(&key).await;
    HttpResponse::Ok().json(value)
}

#[get("/malicious-domains")]
async fn malicious_domains() -> impl Responder {
    let file_path = PathBuf::from("malicious-domains.json");

    match File::open(file_path).await {
        Ok(mut file) => {
            let mut contents = Vec::new();
            if let Err(e) = file.read_to_end(&mut contents).await {
                return HttpResponse::InternalServerError().body(format!("Failed to read file: {}", e));
            }

            // Send the file content as response body
            HttpResponse::Ok()
                .content_type("application/json") // Adjust content type as needed
                .body(contents)
        }
        Err(e) => {
            // File not found or any other error
            println!("Error opening file: {}", e);
            HttpResponse::NotFound().body(format!("File not found: {}", e))
        }
    }
}