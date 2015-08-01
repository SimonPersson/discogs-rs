use super::{Discogs, MASTERS_URL};
use ease::{RestClient, UserAgent};

#[derive(RustcDecodable, Debug)]
struct Video {
    duration: u32,
    description: String, 
    embed: bool,
    uri: String,
    title: String
}

#[derive(RustcDecodable, Debug)]
struct Artist {
    join: String,
    name: String,
    tracks: String,
    role: String,
    resource_url: String,
    id: u32
}

#[derive(Debug)]
struct Image {
    height: u32,
    width: u32,
    resource_url: String,
    type_: String,
    uri: String,
    uri150: String
}

#[derive(RustcDecodable, Debug)]
struct Track {
    duration: String,
    position: String,
    type_: String,
    title: String
}

#[derive(RustcDecodable, Debug)]
pub struct MasterRelease {
    styles: Vec<String>,
    genres: Vec<String>,
    videos: Vec<Video>,
    title: String,
    main_release: u32,
    main_release_url: String,
    uri: String,
    artists: Vec<Artist>,
    versions_url: String,
    year: u32,
    images: Vec<Image>,
    resource_url: String,
    tracklist: Vec<Track>,
    id: u32,
    data_quality: String
}

impl<'a> Discogs<'a> {
    pub fn master_release(&self, master_id: u32) -> Result<MasterRelease, String> {
        let mut url = MASTERS_URL.clone();
        url.path_mut().expect("Couldn't get masters url path.").push(format!("{}", master_id));
        RestClient::new(url)
            .param("token", self.token)
            .header(UserAgent(self.user_agent.to_string()))
            .get_json_as::<MasterRelease>()
    }
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

