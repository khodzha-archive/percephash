extern crate hyper;
extern crate multipart;

extern crate image;
extern crate img_hash;
extern crate serde;
#[macro_use]
extern crate serde_json;

use std::io::Read;
use std::collections::HashMap;
use img_hash::{ImageHash, HashType};

use hyper::server::{Handler, Server, Request, Response};
use hyper::status::StatusCode;
use hyper::server::response::Response as HyperResponse;
use multipart::server::hyper::{Switch, MultipartHandler, HyperRequest};
use multipart::server::{Multipart};
use multipart::server::ReadEntryResult::Entry;

const HASH_SIZE: u32 = 32;


fn get_hashes(data: &[u8]) -> String {
    let image = image::load_from_memory(&data).unwrap();

    let mut hashes = HashMap::new();
    hashes.insert("mean", ImageHash::hash(&image, HASH_SIZE, HashType::Mean).bitv.to_bytes());
    hashes.insert("gradient", ImageHash::hash(&image, HASH_SIZE, HashType::DoubleGradient).bitv.to_bytes());
    hashes.insert("dct", ImageHash::hash(&image, HASH_SIZE, HashType::DCT).bitv.to_bytes());

    let json_response = json!(hashes).to_string();
    json_response
}

struct NonMultipart;
impl Handler for NonMultipart {
    fn handle(&self, _: Request, mut res: Response) {
        *res.status_mut() = StatusCode::ImATeapot;
        res.send(b"Please send a multipart req :(\n").unwrap();
    }
}

struct CalcHashes;
impl MultipartHandler for CalcHashes {
    fn handle_multipart(&self, multipart: Multipart<HyperRequest>, mut res: HyperResponse) {
        let entry = multipart.into_entry();
        match entry {
            Entry(mut mp_field) => {
                match mp_field.data.as_file() {
                    Some(f) => {
                        let mut s = Vec::new();
                        f.read_to_end(&mut s).unwrap();
                        res.send(get_hashes(&s).as_bytes()).unwrap()

                    }
                    _ => {
                        *res.status_mut() = StatusCode::BadRequest;
                        res.send(b"An error occurred :(\n").unwrap();
                    }
                }
            }
            _ => {
                *res.status_mut() = StatusCode::BadRequest;
                res.send(b"An error occurred :(\n").unwrap();
            }
        }

    }
}

fn main() {
    Server::http("0.0.0.0:3333").unwrap().handle(
        Switch::new(
            NonMultipart,
            CalcHashes
        )).unwrap();
}
