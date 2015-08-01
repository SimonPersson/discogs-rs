extern crate rustc_serialize;
extern crate ease;

mod search;
mod master;

pub use search::{Search, SearchType};

pub struct Discogs<'a> {
    token: &'a str,
    user_agent: &'a str
}

const SEARCH_URL: &'static str = "https://api.discogs.com/database/search";
const MASTERS_URL: &'static str = "https://api.discogs.com/masters";

impl<'a> Discogs<'a> {
    pub fn new(token: &'a str, user_agent: &'a str) -> Discogs<'a> {
        Discogs {
            token: token,
            user_agent: user_agent
        }
    }
}
