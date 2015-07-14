extern crate discogs;

use discogs::search;
use std::env;

fn main()
{
   if let Some(token) = env::args().nth(1) {
       println!("{:?}", search("traneing in", &*token));
   } else {
       println!("You must provide a token!");
   }
}
