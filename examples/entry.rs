#![allow(dead_code)]

use std::env;

use chronicle_decoder::decoder::{decode_character, decode_date, decode_location, decode_story};
use chronicle_decoder::generated::MOL_CHRONICLE_SCHEMA;
use chronicle_decoder::handler::{dobs_check_composable, dobs_decode};
use chronicle_decoder::object::ParsedDNA;
use chronicle_decoder::schema::AshWarChronicle;
use molecule::prelude::Entity;

extern crate alloc;

fn main() {
    let case = env::args().nth(1).expect("need case");
    match case.as_str() {
        "debug_decode" => {
            let chronicle =
                AshWarChronicle::from_compatible_slice(MOL_CHRONICLE_SCHEMA).expect("init");
            let dna = {
                let hexed_dna = env::args().nth(2).expect("DNA is required");
                hex::decode(hexed_dna).expect("encode dna")
            };
            let character: Vec<ParsedDNA> =
                decode_character(chronicle.character_schema(), dna.clone())
                    .expect("character")
                    .into();
            let location: Vec<ParsedDNA> =
                decode_location(chronicle.location_schema(), dna.clone())
                    .expect("location")
                    .into();
            let date: Vec<ParsedDNA> = decode_date(chronicle.date_schema(), dna.clone())
                .expect("date")
                .into();
            let story: Vec<ParsedDNA> = decode_story(chronicle.story_schema(), dna)
                .expect("story")
                .into();

            println!("[人物]\n{}\n", serde_json::to_string(&character).unwrap());
            println!("[地点]\n{}\n", serde_json::to_string(&location).unwrap());
            println!("[时间]\n{}\n", serde_json::to_string(&date).unwrap());
            println!("[故事]\n{}\n", serde_json::to_string(&story).unwrap());
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
                .map(|i| {
                    let hexed_dna = env::args().nth(i).expect(&format!("dna at {i}"));
                    hex::decode(&hexed_dna).expect("hex")
                })
                .collect::<Vec<_>>();
            let compose = dobs_check_composable(dna_set.try_into().unwrap()).expect("compose");
            println!("composable = {compose}");
        }
        _ => panic!("bad case"),
    }
}
