use super::{Discogs, MASTERS_URL, Error};
use ease::{Request, UserAgent, StatusCode};
use ease::Error as EaseError;

impl<'a> Discogs<'a> {
    pub fn master_release(&self, master_id: u32) -> Result<MasterRelease, Error> {
        let mut url = MASTERS_URL.clone();
        url.path_mut().expect("Couldn't get masters url path.").push(format!("{}", master_id));
        Request::new(url)
            .param("token", self.token)
            .header(UserAgent(self.user_agent.to_string()))
            .get()
            .and_then(|res| res.json_as::<MasterRelease>())
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

#[derive(RustcDecodable, Debug)]
pub struct Video {
    pub duration: u32,
    pub description: String, 
    pub embed: bool,
    pub uri: String,
    pub title: String
}

#[derive(RustcDecodable, Debug)]
pub struct Artist {
    pub join: String,
    pub name: String,
    pub tracks: String,
    pub role: String,
    pub resource_url: String,
    pub id: u32
}

#[derive(Debug)]
pub struct Image {
    pub height: u32,
    pub width: u32,
    pub resource_url: String,
    pub type_: String,
    pub uri: String,
    pub uri150: String
}

#[derive(RustcDecodable, Debug)]
pub struct Track {
    pub duration: String,
    pub position: String,
    pub type_: String,
    pub title: String
}

#[derive(RustcDecodable, Debug)]
pub struct MasterRelease {
    pub styles: Option<Vec<String>>,
    pub genres: Option<Vec<String>>,
    pub videos: Option<Vec<Video>>,
    pub title: String,
    pub main_release: u32,
    pub main_release_url: String,
    pub uri: String,
    pub artists: Vec<Artist>,
    pub versions_url: String,
    pub year: u32,
    pub images: Vec<Image>,
    pub resource_url: String,
    pub tracklist: Vec<Track>,
    pub id: u32,
    pub data_quality: String
}

impl ::rustc_serialize::Decodable for Image {
    fn decode<__D: ::rustc_serialize::Decoder>(__arg_0: &mut __D)
     -> ::std::result::Result<Image, __D::Error> {
        __arg_0.read_struct("Image", 6usize, |_d| -> _ {
                            ::std::result::Result::Ok(Image{height:
                                                                match _d.read_struct_field("height",
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
                                                            width:
                                                                match _d.read_struct_field("width",
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
                                                            resource_url:
                                                                match _d.read_struct_field("resource_url",
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
                                                            type_:
                                                                match _d.read_struct_field("type",
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
                                                            uri:
                                                                match _d.read_struct_field("uri",
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
                                                            uri150:
                                                                match _d.read_struct_field("uri150",
                                                                                           5usize,
                                                                                           ::rustc_serialize::Decodable::decode)
                                                                    {
                                                                    ::std::result::Result::Ok(__try_var)
                                                                    =>
                                                                    __try_var,
                                                                    ::std::result::Result::Err(__try_var)
                                                                    =>
                                                                    return ::std::result::Result::Err(__try_var),
                                                                },}) })
    }
}

