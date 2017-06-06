extern crate image;
extern crate img_hash;
#[macro_use]
extern crate log;

use std::env;
use std::fs::File;
use std::io::Read;

use img_hash::{ImageHash, HashType};

fn main() {
    let path: String = env::args().nth(1).unwrap();
    match phash(&path) {
        Ok(hash) => {
            println!("{:?}", hash.bitv.to_bytes());
        }
        Err(e) => {
            error!("Error: {}! Filepath: {}", e, path);
        }
    }

}

fn phash(path: &String) -> Result<ImageHash, String> {

    let mut file = try!(File::open(path).map_err(|e| e.to_string()));
    let mut buffer: Vec<u8> = Vec::new();
    try!(file.read_to_end(&mut buffer).map_err(|e| e.to_string()));
    let image = try!(image::load_from_memory(&buffer).map_err(|e| e.to_string()));

    let hash = ImageHash::hash(&image, 64, HashType::Gradient);
    Ok(hash)
}
