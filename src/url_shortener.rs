use std::collections::HashMap;

pub struct UrlShortener {
    short_urls: HashMap<u128, String>
}
impl UrlShortener {
    pub fn new() -> Self {
        UrlShortener { short_urls: HashMap::new() }
    }

    pub fn shorten(&mut self, origin: &str) -> String{
        let code = match self.short_urls.keys().max() {
            Some(code) => code+1,
            None => 1
        };
        self.short_urls.insert(code.clone(), origin.to_string());

        format!("{}", code)
    }

    pub fn resolve(&self, code: &str) -> Option<String>{
        let key = code.parse::<u128>().unwrap();

        self.short_urls.get(&key).cloned()
    }
}
