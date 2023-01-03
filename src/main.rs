mod url_shortener;
mod model;
use actix_files::{NamedFile, Files};
use actix_web::{web::{self}, App, HttpServer, HttpResponse, post, get};
use model::ShortenedUrl;
use r2d2_sqlite::SqliteConnectionManager;
use sea_query::{SqliteQueryBuilder, Table, ColumnDef};
use std::{sync::RwLock};
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

#[post("/s")]
async fn shorten(data: web::Data<AppState>, form: web::Form<Info>, req: actix_web::HttpRequest) -> HttpResponse {
    let mut urlsh = data.url_sh.write().unwrap();
    match &urlsh.shorten(&form.url){
        Ok(code) => HttpResponse::Ok().insert_header(("Content-type", "text/plain")).body(format!("{}/r/{}", req.app_config().host(), code)),
        Err(error) => HttpResponse::BadRequest().body(error.to_string())
    }
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

    let manager = SqliteConnectionManager::file("file.db");
    let pool = r2d2::Pool::new(manager).unwrap();
    let sql = [
        Table::create()
            .if_not_exists()
            .table(ShortenedUrl::Table)
            .col(
                ColumnDef::new(ShortenedUrl::Code)
                    .auto_increment()
                    .primary_key()
                    .integer()
            )
            .col(
                ColumnDef::new(ShortenedUrl::Url)
                    .string()
                    .not_null()
            )
            .build(SqliteQueryBuilder),
        ].join("; ");
    println!("{}", &sql);

    match pool.get().unwrap().execute_batch(&sql) {
        Ok(_) => println!("Checked Schema"),
        Err(e) => panic!("Failed to check schema {:?}", e)
    }

    

    let urlsh = web::Data::new(AppState {
        url_sh: RwLock::new(UrlShortener::new(pool))
    });

    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(Files::new("/assets", "./assets").show_files_listing())
            .app_data(urlsh.clone())
            .service(shorten)
            .service(resolve)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}