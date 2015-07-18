extern crate discogs;

use discogs::{Discogs, SearchType};
use std::env;

fn main()
{
    if let Some(token) = env::args().nth(1) {
        let mut discogs = Discogs::new(&token, "TestApp");
        println!("{:#?}", discogs.search()
                                 .query("Have Trumpet, Will Excite")
                                 .search_type(SearchType::Master)
                                 .send()
                                 );
    } else {
        println!("You must provide a token!");
    }
}
