extern crate ease;

use ease::{Url, RestClient, UserAgent};

static BASE_URL: &'static str = "https://api.discogs.com/";

pub fn search(query: &str, token: &str) -> String {
    let mut url_str = BASE_URL.to_owned();
    url_str.extend("database/search".to_owned().chars());
    let url = Url::parse(&*url_str).unwrap();
    RestClient::new(url)
        .param(("q", query))
        .param(("token", token))
        .header(UserAgent("DiscogsTestRust/0,1".to_owned()))
        .get()
        .unwrap()
}
