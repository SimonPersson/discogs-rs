#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]
#![feature(custom_attribute)]
extern crate serde;
extern crate ease;

use ease::{Url, RestClient, UserAgent};

static BASE_URL: &'static str = "https://api.discogs.com/";

#[derive(Deserialize, Debug)]
pub struct Urls {
    next: String,
    last: String
}
    
#[derive(Deserialize, Debug)]
pub struct Community {
    have: u32,
    want: u32,
}

#[derive(Deserialize, Debug)]
pub struct SearchResultElement {
    style: Option<Vec<String>>,
    thumb: Option<String>,
    format: Option<Vec<String>>,
    country: Option<String>,
    barcode: Option<Vec<String>>,
    uri: Option<String>,
    community: Option<Community>,
    label: Option<Vec<String>>,
    catno: Option<String>,
    year: Option<String>,
    genre: Option<Vec<String>>,
    title: Option<String>,
    resource_url: Option<String>,
    #[serde(rename="type")]
    type_name: Option<String>,
    id: Option<u32>
}

#[derive(Deserialize, Debug)]
pub struct Pagination {
    per_page: u32,
    items: u32,
    page: u32,
    urls: Urls,
    pages: u32,
}

#[derive(Deserialize, Debug)]
pub struct SearchResult {
    pagination: Pagination,
    results: Vec<SearchResultElement>
}

pub fn search(query: &str, token: &str) -> SearchResult {
    let mut url_str = BASE_URL.to_owned();
    url_str.extend("database/search".to_owned().chars());
    let url = Url::parse(&*url_str).unwrap();
    RestClient::new(url)
        .param(("q", query))
        .param(("token", token))
        .header(UserAgent("DiscogsTestRust/0,1".to_owned()))
        .get_json_as::<SearchResult>()
        .unwrap()
}
