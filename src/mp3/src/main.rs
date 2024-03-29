use std::time::Duration;

use rmp3::{Decoder, Frame};
fn main() {
    let mp3 = std::fs::read("./data_podcast_clip.mp3").unwrap();
    let res = create_bounds(mp3, 60);
}

fn create_bounds(source: Vec<u8>, interval: u32) -> Vec<u32> {
    let mut decoder = Decoder::new(&source);
    let mut start: u32 = 0;
    let mut end: u32 = 0;
    let mut duration: f32 = 0.0;
    let mut result: Vec<u32> = vec![];
    while let Some(frame) = decoder.peek() {
        if let Frame::Audio(audio) = frame {
            if start == 0 {
                start = decoder.position() as u32;
            }
            end = decoder.position() as u32 + audio.source().len() as u32;
            duration += audio.sample_count() as f32 / audio.sample_rate() as f32;
            if duration >= interval as f32 {
                println!("duration:{:?}", duration);
                result.push(start);
                duration = 0.0;
                start = 0;
            }
            decoder.skip();
        }
    }
    result.push(start);
    result.push(end);
    return result;
}
