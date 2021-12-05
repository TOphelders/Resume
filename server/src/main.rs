use actix_files as fs;
use actix_web::{HttpRequest, Result};
use std::path::PathBuf;

async fn index(_req: HttpRequest) -> Result<fs::NamedFile> {
    let path: PathBuf = "../resume.html".parse().unwrap();
    Ok(fs::NamedFile::open(path)?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{web, App, HttpServer};

    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/dist", "../dist").show_files_listing())
            .route("*", web::get().to(index))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
