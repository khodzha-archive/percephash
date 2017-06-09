extern crate image;
extern crate img_hash;
extern crate serde;
#[macro_use]
extern crate serde_json;

use std::env;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

use img_hash::{ImageHash, HashType};

const HASH_SIZE: u32 = 32;

fn main() {
    let path: String = env::args().nth(1).unwrap();
    let mut file = File::open(path).unwrap();
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer);
    let image = image::load_from_memory(&buffer).unwrap();

    let mut hashes = HashMap::new();
    hashes.insert("mean", ImageHash::hash(&image, HASH_SIZE, HashType::Mean).bitv.to_bytes());
    hashes.insert("gradient", ImageHash::hash(&image, HASH_SIZE, HashType::DoubleGradient).bitv.to_bytes());
    hashes.insert("dct", ImageHash::hash(&image, HASH_SIZE, HashType::DCT).bitv.to_bytes());
    println!("{}", json!(hashes));
}
