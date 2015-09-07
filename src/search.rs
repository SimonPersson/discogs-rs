use super::{Discogs, SEARCH_URL, Error};
use ease::{Request, UserAgent, Url, StatusCode};
use ease::Error as EaseError;

pub enum SearchType {
    Release,
    Master,
    Artist,
    Label
}

pub struct Search<'a> {
    url: &'a Url,
    token: &'a str,
    user_agent: &'a str,
    params: Vec<(&'a str, &'a str)>
}

impl<'a> Search<'a> {
    pub fn query(mut self, query: &'a str) -> Search<'a> {
        self.params.push(("q", query));
        self
    }

    pub fn release_title(mut self, release_title: &'a str) -> Search<'a> {
        self.params.push(("release_title", release_title));
        self
    }

    pub fn title(mut self, title: &'a str) -> Search<'a> {
        self.params.push(("title", title));
        self
    }
    
    pub fn credit(mut self, credit: &'a str) -> Search<'a> {
        self.params.push(("credit", credit));
        self
    }

    pub fn artist(mut self, artist: &'a str) -> Search<'a> {
        self.params.push(("artist", artist));
        self
    }
    
    pub fn anv(mut self, anv: &'a str) -> Search<'a> {
        self.params.push(("anv", anv));
        self
    }
    
    pub fn label(mut self, label: &'a str) -> Search<'a> {
        self.params.push(("label", label));
        self
    }
    
    pub fn genre(mut self, genre: &'a str) -> Search<'a> {
        self.params.push(("genre", genre));
        self
    }
    
    pub fn style(mut self, style: &'a str) -> Search<'a> {
        self.params.push(("style", style));
        self
    }
    
    pub fn country(mut self, country: &'a str) -> Search<'a> {
        self.params.push(("country", country));
        self
    }
    
    pub fn year(mut self, year: &'a str) -> Search<'a> {
        self.params.push(("year", year));
        self
    }
    
    pub fn format(mut self, format: &'a str) -> Search<'a> {
        self.params.push(("format", format));
        self
    }
    
    pub fn catno(mut self, catno: &'a str) -> Search<'a> {
        self.params.push(("catno", catno));
        self
    }
    
    pub fn barcode(mut self, barcode: &'a str) -> Search<'a> {
        self.params.push(("barcode", barcode));
        self
    }
    
    pub fn track(mut self, track: &'a str) -> Search<'a> {
        self.params.push(("track", track));
        self
    }
    
    pub fn submitter(mut self, submitter: &'a str) -> Search<'a> {
        self.params.push(("submitter", submitter));
        self
    }
    
    pub fn contributor(mut self, contributor: &'a str) -> Search<'a> {
        self.params.push(("contributor", contributor));
        self
    }
    
    pub fn search_type(mut self, search_type: SearchType) -> Search<'a> {
        self.params.push(("type",
                      match search_type {
                          SearchType::Release => "release",
                          SearchType::Master => "master",
                          SearchType::Artist => "artist",
                          SearchType::Label => "label"
                      }));
        self
    }
            
    pub fn send(self) -> Result<SearchResult, Error> {
        Request::new(self.url.clone())
            .params(self.params)
            .param("token", self.token)
            .header(UserAgent(self.user_agent.to_string()))
            .get()
            .and_then(|res| res.json_as::<SearchResult>())
            .map_err(|ease_err| {
                match ease_err {
                    EaseError::UnsuccessfulResponse(ref e)
                        if e.hyper_response.status == StatusCode::Unauthorized =>
                            Error::AuthError(e.body.clone()),
                    _ => Error::HttpError(ease_err),
                }
            })
    }
}

impl<'a, 'b> Discogs<'a> where 'a: 'b {
    pub fn search(&'a mut self) -> Search<'b> {
        Search {
            url: &SEARCH_URL, 
            token: self.token,
            user_agent: self.user_agent,
            params: Vec::new()
        } 
    }
}

#[derive(RustcDecodable, Debug)]
pub struct Urls {
    pub next: Option<String>,
    pub last: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct Community {
    pub have: u32,
    pub want: u32,
}

#[derive(Debug)]
pub struct SearchResultElement {
    pub style: Option<Vec<String>>,
    pub format: Option<Vec<String>>,
    pub country: Option<String>,
    pub barcode: Option<Vec<String>>,
    pub community: Option<Community>,
    pub label: Option<Vec<String>>,
    pub catno: Option<String>,
    pub year: Option<String>,
    pub genre: Option<Vec<String>>,
    pub thumb: String,
    pub uri: String,
    pub title: String,
    pub resource_url: String,
    pub type_: String,
    pub id: u32,
}

#[derive(RustcDecodable, Debug)]
pub struct Pagination {
    pub per_page: Option<u32>,
    pub items: u32,
    pub page: u32,
    pub urls: Urls,
    pub pages: u32,
}

#[derive(RustcDecodable, Debug)]
pub struct SearchResult {
    pub pagination: Pagination,
    pub results: Option<Vec<SearchResultElement>>,
}


/* Discogs api has "type" as a parameter. We cannot have a field named
 * type in SearchResultElement, as type is a keyword. Hence, we
 * have to use a type_ field and then manually modify the Decodable
 * implementation to make it work.
 */
impl ::rustc_serialize::Decodable for SearchResultElement {
    fn decode<__D: ::rustc_serialize::Decoder>
                                               (__arg_0: &mut __D)
                                                -> ::std::result::Result<SearchResultElement, __D::Error> {
        __arg_0.read_struct("SearchResultElement", 15usize, |_d| -> _ {
                ::std::result::Result::Ok(SearchResultElement{style:
                    match _d.read_struct_field("style",
                                               0usize,
                                               ::rustc_serialize::Decodable::decode)
                    {
                        ::std::result::Result::Ok(__try_var)
                            =>
                            __try_var,
                            ::std::result::Result::Err(__try_var)
                                =>
                                return ::std::result::Result::Err(__try_var),
                    },
                    format:
                        match _d.read_struct_field("format",
                                                   1usize,
                                                   ::rustc_serialize::Decodable::decode)
                        {
                            ::std::result::Result::Ok(__try_var)
                                =>
                                __try_var,
                                ::std::result::Result::Err(__try_var)
                                    =>
                                    return ::std::result::Result::Err(__try_var),
                        },
                        country:
                            match _d.read_struct_field("country",
                                                       2usize,
                                                       ::rustc_serialize::Decodable::decode)
                            {
                                ::std::result::Result::Ok(__try_var)
                                    =>
                                    __try_var,
                                    ::std::result::Result::Err(__try_var)
                                        =>
                                        return ::std::result::Result::Err(__try_var),
                            },
                            barcode:
                                match _d.read_struct_field("barcode",
                                                           3usize,
                                                           ::rustc_serialize::Decodable::decode)
                                {
                                    ::std::result::Result::Ok(__try_var)
                                        =>
                                        __try_var,
                                        ::std::result::Result::Err(__try_var)
                                            =>
                                            return ::std::result::Result::Err(__try_var),
                                },
                                community:
                                    match _d.read_struct_field("community",
                                                               4usize,
                                                               ::rustc_serialize::Decodable::decode)
                                    {
                                        ::std::result::Result::Ok(__try_var)
                                            =>
                                            __try_var,
                                            ::std::result::Result::Err(__try_var)
                                                =>
                                                return ::std::result::Result::Err(__try_var),
                                    },
                                    label:
                                        match _d.read_struct_field("label",
                                                                   5usize,
                                                                   ::rustc_serialize::Decodable::decode)
                                        {
                                            ::std::result::Result::Ok(__try_var)
                                                =>
                                                __try_var,
                                                ::std::result::Result::Err(__try_var)
                                                    =>
                                                    return ::std::result::Result::Err(__try_var),
                                        },
                                        catno:
                                            match _d.read_struct_field("catno",
                                                                       6usize,
                                                                       ::rustc_serialize::Decodable::decode)
                                            {
                                                ::std::result::Result::Ok(__try_var)
                                                    =>
                                                    __try_var,
                                                    ::std::result::Result::Err(__try_var)
                                                        =>
                                                        return ::std::result::Result::Err(__try_var),
                                            },
                                            year:
                                                match _d.read_struct_field("year",
                                                                           7usize,
                                                                           ::rustc_serialize::Decodable::decode)
                                                {
                                                    ::std::result::Result::Ok(__try_var)
                                                        =>
                                                        __try_var,
                                                        ::std::result::Result::Err(__try_var)
                                                            =>
                                                            return ::std::result::Result::Err(__try_var),
                                                },
                                                genre:
                                                    match _d.read_struct_field("genre",
                                                                               8usize,
                                                                               ::rustc_serialize::Decodable::decode)
                                                    {
                                                        ::std::result::Result::Ok(__try_var)
                                                            =>
                                                            __try_var,
                                                            ::std::result::Result::Err(__try_var)
                                                                =>
                                                                return ::std::result::Result::Err(__try_var),
                                                    },
                                                    thumb:
                                                        match _d.read_struct_field("thumb",
                                                                                   9usize,
                                                                                   ::rustc_serialize::Decodable::decode)
                                                        {
                                                            ::std::result::Result::Ok(__try_var)
                                                                =>
                                                                __try_var,
                                                                ::std::result::Result::Err(__try_var)
                                                                    =>
                                                                    return ::std::result::Result::Err(__try_var),
                                                        },
                                                        uri:
                                                            match _d.read_struct_field("uri",
                                                                                       10usize,
                                                                                       ::rustc_serialize::Decodable::decode)
                                                            {
                                                                ::std::result::Result::Ok(__try_var)
                                                                    =>
                                                                    __try_var,
                                                                    ::std::result::Result::Err(__try_var)
                                                                        =>
                                                                        return ::std::result::Result::Err(__try_var),
                                                            },
                                                            title:
                                                                match _d.read_struct_field("title",
                                                                                           11usize,
                                                                                           ::rustc_serialize::Decodable::decode)
                                                                {
                                                                    ::std::result::Result::Ok(__try_var)
                                                                        =>
                                                                        __try_var,
                                                                        ::std::result::Result::Err(__try_var)
                                                                            =>
                                                                            return ::std::result::Result::Err(__try_var),
                                                                },
                                                                resource_url:
                                                                    match _d.read_struct_field("resource_url",
                                                                                               12usize,
                                                                                               ::rustc_serialize::Decodable::decode)
                                                                    {
                                                                        ::std::result::Result::Ok(__try_var)
                                                                            =>
                                                                            __try_var,
                                                                            ::std::result::Result::Err(__try_var)
                                                                                =>
                                                                                return ::std::result::Result::Err(__try_var),
                                                                    },
                                                                    type_:
                                                                        match _d.read_struct_field("type",
                                                                                                   13usize,
                                                                                                   ::rustc_serialize::Decodable::decode)
                                                                        {
                                                                            ::std::result::Result::Ok(__try_var)
                                                                                =>
                                                                                __try_var,
                                                                                ::std::result::Result::Err(__try_var)
                                                                                    =>
                                                                                    return ::std::result::Result::Err(__try_var),
                                                                        },
                                                                        id:
                                                                            match _d.read_struct_field("id",
                                                                                                       14usize,
                                                                                                       ::rustc_serialize::Decodable::decode)
                                                                            {
                                                                                ::std::result::Result::Ok(__try_var)
                                                                                    =>
                                                                                    __try_var,
                                                                                    ::std::result::Result::Err(__try_var)
                                                                                        =>
                                                                                        return ::std::result::Result::Err(__try_var),
                                                                            },})
            })
    }
}
