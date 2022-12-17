use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use sea_query::{Query, SqliteQueryBuilder, Expr};
use sea_query_rusqlite::RusqliteBinder;


use crate::model::{ShortenedUrl, ShortenedUrlStruct};

pub struct UrlShortener {
    pool: Pool<SqliteConnectionManager>
}
impl UrlShortener {
    pub fn new(db_pool: Pool<SqliteConnectionManager>) -> Self {
        UrlShortener { pool: db_pool }
    }

    pub fn shorten(&mut self, origin: &str) -> String{
        let (query, values) = Query::insert()
            .into_table(ShortenedUrl::Table)
            .columns([
                ShortenedUrl::Url
            ])
            .values_panic([
                origin.into()
            ])
            .build_rusqlite(SqliteQueryBuilder);

        let con = self.pool.get().unwrap();

        match con.execute(&query, &*values.as_params()) {
            Ok(_) => println!("added {} to database", origin),
            Err(_) => println!("error")
        }

        con.last_insert_rowid().to_string()
    }

    pub fn resolve(&self, code: &str) -> Option<String>{
        let con = self.pool.get().unwrap();

        let (sql, values) = Query::select()
            .columns([
                ShortenedUrl::Code,
                ShortenedUrl::Url
            ])
            .from(ShortenedUrl::Table)
            .and_where(Expr::col(ShortenedUrl::Code).eq(code))
            .limit(1)
            .build_rusqlite(SqliteQueryBuilder);
        let mut statement = con.prepare(&sql).unwrap();
        let mut result = statement.query(&*values.as_params()).unwrap();


        match result.next(){
            Ok(r) => ShortenedUrlStruct::get_opt_url(r),
            Err(err) => {
                println!("{:?}", err);
                None
            }
        }
    }
}
