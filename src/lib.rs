#[macro_use]
extern crate lazy_static;
extern crate rustc_serialize;
extern crate ease;

mod search;
mod master;

pub use search::{Search, SearchType, Urls, Community, SearchResultElement, Pagination, SearchResult};
pub use master::{Video, Artist, Track, MasterRelease};

use ease::Url;
use ease::Error as EaseError;

#[derive(Debug)]
pub enum Error {
    HttpError(EaseError),
    AuthError(String),
}

pub struct Discogs<'a> {
    token: &'a str,
    user_agent: &'a str
}

lazy_static! {
    static ref MASTERS_URL: Url = Url::parse("https://api.discogs.com/masters").ok().expect("Couldn't parse masters url.");
    static ref SEARCH_URL: Url = Url::parse("https://api.discogs.com/database/search").ok().expect("Couldn't parse search url.");
}

impl<'a> Discogs<'a> {
    pub fn new(token: &'a str, user_agent: &'a str) -> Discogs<'a> {
        Discogs {
            token: token,
            user_agent: user_agent
        }
    }
}
