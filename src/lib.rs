extern crate rustc_serialize;
extern crate ease;

use ease::Url;

mod search;

pub use search::{Search, SearchType};

pub struct Discogs<'a> {
    search_url: Url,
    token: &'a str,
    user_agent: &'a str
}

impl<'a> Discogs<'a> {
    pub fn new(token: &'a str, user_agent: &'a str) -> Discogs<'a> {
        Discogs {
            search_url: Url::parse("https://api.discogs.com/database/search")
                            .ok().expect("database/search URL did not parse."),
            token: token,
            user_agent: user_agent
        }
    }
}
