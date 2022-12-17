use rusqlite::Row;
use sea_query::Iden;


#[derive(Iden)]
pub enum ShortenedUrl {
    Table,
    Code,
    Url
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct ShortenedUrlStruct {
    code: isize,
    pub url: String
}
impl ShortenedUrlStruct{
    pub fn get_opt_url(opt_row: Option<&Row<'_>>) -> Option<String> {
        opt_row.map(|row| Self::from(row).url)
    }
}
impl From<&Row<'_>> for ShortenedUrlStruct {
    fn from(row: &Row<'_>) -> Self {
        Self {
            code: row.get_unwrap("code"),
            url: row.get_unwrap("url"),
        }
    }
}
