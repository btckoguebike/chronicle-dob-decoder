#![allow(dead_code)]

use std::env;

use core::decoder::Language;
use core::render::SegmentRender;
use object::{RawCharacter, RawDate, RawLocation, RawStory};

extern crate alloc;
mod core;
mod error;
mod generated;
mod handler;
mod object;

fn main() {
    let dna = {
        let hexed_dna = env::args().nth(1).expect("DNA is required");
        hex::decode(hexed_dna).expect("encode dna")
    };
    let segment_render = SegmentRender::new(Language::CN)
        .map_err(|_| "segment render")
        .unwrap();
    let character_render = RawCharacter::from_generated()
        .map_err(|_| "new character")
        .unwrap()
        .render(&segment_render, dna.clone())
        .map_err(|_| "render charactor")
        .unwrap();
    let location_render = RawLocation::from_generated()
        .map_err(|_| "new location")
        .unwrap()
        .render(&segment_render, dna.clone())
        .map_err(|_| "render location")
        .unwrap();
    let date_render = RawDate::from_generated()
        .map_err(|_| "new date")
        .unwrap()
        .render(&segment_render, dna.clone())
        .map_err(|_| "render character")
        .unwrap();
    let story_render = RawStory::from_generated()
        .map_err(|_| "new story")
        .unwrap()
        .render(&segment_render, dna)
        .map_err(|_| "render story")
        .unwrap();

    println!("[人物]\n{character_render}\n");
    println!("[地点]\n{location_render}\n");
    println!("[时间]\n{date_render}\n");
    println!("[故事]\n{story_render}\n");
}
