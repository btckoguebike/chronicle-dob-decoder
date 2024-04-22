#![allow(dead_code)]

use std::env;

use core::decoder::{set_decoder_language, Language};
use core::render::Render;
use object::{Character, Date, Location, Story};

extern crate alloc;
mod core;
mod error;
mod generated;
mod handler;
mod object;

fn main() {
    set_decoder_language(Language::CN).expect("set language");

    let dna = {
        let hexed_dna = env::args().nth(1).expect("DNA is required");
        hex::decode(hexed_dna).expect("encode dna")
    };
    let character_render = Character::new_from_generated()
        .expect("new character")
        .render(dna.clone())
        .expect("render charactor");
    let location_render = Location::new_from_generated()
        .expect("new location")
        .render(dna.clone())
        .expect("render location");
    let date_render = Date::new_from_generated()
        .expect("new date")
        .render(dna.clone())
        .expect("render character");
    let story_render = Story::new_from_generated()
        .expect("new story")
        .render(dna)
        .expect("render story");

    println!("[人物]\n{character_render}\n");
    println!("[地点]\n{location_render}\n");
    println!("[时间]\n{date_render}\n");
    println!("[故事]\n{story_render}\n");
}
