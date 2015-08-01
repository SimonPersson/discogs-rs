extern crate discogs;

use discogs::Discogs;
use std::env;

fn main()
{
    if let Some(token) = env::args().nth(1) {
        let mut discogs = Discogs::new(&token, "TestApp");
        println!("{:#?}", discogs.master_release(62462));
    } else {
        println!("You must provide a token!");
    }
}
