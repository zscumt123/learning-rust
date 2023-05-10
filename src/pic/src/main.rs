use anyhow::Result;
use serde::Deserialize;
use std::io::{BufReader, Write};
use std::{fs::File, io::Read};

#[derive(Debug, Deserialize)]
struct RecordItem {
    name: String,
    address: String,
    tel: String,
    image_url: String,
}

fn main() -> Result<()> {
    let file = File::open("./car.json")?;
    let mut buf_reader = BufReader::new(file);
    let mut st = String::new();
    buf_reader.read_to_string(&mut st)?;

    let result: Vec<RecordItem> = serde_json::from_str(&st)?;
    for (id, item) in result.iter().enumerate() {
        match download_img(item, id) {
            Ok(_) => println!("download {}.jpg image success", id),
            Err(e) => eprintln!("err :{:?}", e),
        }
    }

    Ok(())
}

fn download_img(item: &RecordItem, id: usize) -> Result<()> {
    println!("start request {:?} pic", id);
    let resp = reqwest::blocking::get(&item.image_url)?;
    let images = resp.bytes()?;
    let mut file = File::create(format!("./images/{}.jpg", id))?;
    file.write_all(&images)?;
    Ok(())
}
