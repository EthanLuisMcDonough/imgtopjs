extern crate raster;

use std::fs::File;
use std::io::Write;
use std::string::ToString;
use std::env::args;
use raster::transform::resize_fit;

struct RLEItem {
    value: u8,
    count: u64
}

impl ToString for RLEItem {
    fn to_string(&self) -> String {
        format!("{}{:x}", std::char::from_u32(self.value as u32 + 255).unwrap(), self.count)
    }
}

fn in_bounds(base_n: i32, c_n: i32, range: i32) -> bool {
    c_n >= base_n - range && c_n <= base_n + range
}

fn rleencode(bytes: &[u8], quality: i32) -> Vec<RLEItem> {
    let mut encoded: Vec<RLEItem> = vec![];
    for byte in bytes.iter() {
        let mut push_in = false;
        match encoded.last_mut() {
            Some(ref mut prev) 
                if in_bounds(prev.value as i32, *byte as i32, quality) => { prev.count += 1; },
            _ => { push_in = true; }
        }
        if push_in { encoded.push(RLEItem { value: *byte, count: 1u64 }); }
    }
    encoded
}

fn main() {
    let save_name = "out.js";
    let template = include_str!("prog.js");
    println!("{}", match &args().skip(1).take(2).collect::<Vec<String>>()[..] {
        [path, str_quality] => str_quality.parse::<i32>().map(|quality| match raster::open(&*path) {
            Ok(ref mut image) => resize_fit(image, 400, 400).or(Err(()))
                .map(|()| rleencode(&image.bytes, quality))
                .map(|stream| template
                    .replace("$WIDTH", &*image.width.to_string())
                    .replace("$HEIGHT", &*image.height.to_string())
                    .replace("$DATA", &*format!("\"{}\"", stream.iter()
                        .map(|i| i.to_string()).collect::<String>())))
                .and_then(|message| File::create(save_name)
                .and_then(|mut file| file.write_all(message.as_bytes()))
                .or(Err(())))
                .map(|()| format!("saved to {}", save_name))
                .unwrap_or("Error".to_string()),
            Err(_) => format!("Could not open image {}", path)
        }).unwrap_or("<quality> must be a positive integer".to_string()),
        _ => "Invalid args.  imgtopjs <path> <quality>".to_string()
    });
}
