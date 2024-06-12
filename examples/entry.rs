#![allow(dead_code)]

use std::env;

use chronicle_decoder::core::decoder::{set_decoder_language, Language};
use chronicle_decoder::core::render::Render;
use chronicle_decoder::handler::{dobs_check_composable, dobs_decode};
use chronicle_decoder::object::{Character, Date, Location, Story};

extern crate alloc;

fn main() {
    let case = env::args().nth(1).expect("need case");
    match case.as_str() {
        "debug_decode" => {
            set_decoder_language(Language::CN).expect("set language");
            let dna = {
                let hexed_dna = env::args().nth(2).expect("DNA is required");
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
        "decode" => {
            let dna = {
                let hexed_dna = env::args().nth(2).expect("DNA is required");
                hex::decode(hexed_dna).expect("encode dna")
            };
            let render = dobs_decode(dna).expect("decode");
            println!("render = {}", String::from_utf8_lossy(&render));
        }
        "compose" => {
            let dna_set = (2..6)
                .map(|i| env::args().nth(i).expect(&format!("dna at {i}")))
                .collect::<Vec<_>>();
            let compose = dobs_check_composable(dna_set.try_into().unwrap()).expect("compose");
            println!("composable = {compose}");
        }
        _ => panic!("bad case"),
    }
}
