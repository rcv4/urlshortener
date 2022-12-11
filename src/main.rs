mod url_shortener;
use actix_files::NamedFile;
use actix_web::{web::{self}, App, HttpServer, HttpResponse, post, get, HttpRequest};
use std::{sync::RwLock, path::PathBuf};
use serde::{Deserialize};
use url_shortener::UrlShortener;


struct AppState {
    url_sh: RwLock<UrlShortener>
}
#[derive(Deserialize)]
struct Info {
    url: String
}

#[get("/")]
async fn index() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./assets/index.html")?)
}

#[get("/assets/{filename:.*}")]
async fn resource(req: HttpRequest) -> actix_web::Result<NamedFile> {
    let path: PathBuf = (String::from("./assets/") + req.match_info().query("filename")).parse().unwrap();
    Ok(NamedFile::open(path)?)
}



#[post("/s")]
async fn shorten(data: web::Data<AppState>, form: web::Form<Info>) -> HttpResponse {
    let mut urlsh = data.url_sh.write().unwrap();

    HttpResponse::Ok().insert_header(("Content-type", "text/plain")).body(String::from("127.0.0.1/r/") + &urlsh.shorten(&form.url))
}
#[get("/r/{id}")]
async fn resolve(data: web::Data<AppState>, info: web::Path<String>) -> HttpResponse {
    let urlsh = data.url_sh.read().unwrap();

    let target = match urlsh.resolve(info.as_str()) {
        Some(url) => url,
        None => String::from("/")
    };

    HttpResponse::TemporaryRedirect().insert_header(("Location", target)).finish()
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let urlsh = web::Data::new(AppState {
        url_sh: RwLock::new(UrlShortener::new())
    });

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(resource)
            .app_data(urlsh.clone())
            .service(shorten)
            .service(resolve)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}