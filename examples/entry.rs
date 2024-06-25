#![allow(dead_code)]

use std::env;

use chronicle_decoder::decoder::{decode_context, decode_event, decode_player, decode_scene};
use chronicle_decoder::generated::MOL_CHRONICLE_SCHEMA;
use chronicle_decoder::handler::{dobs_check_composable, dobs_decode};
use chronicle_decoder::object::ParsedDNA;
use chronicle_schema::AshWarChronicle;
use molecule::prelude::Entity;

extern crate alloc;

fn main() {
    let case = env::args().nth(1).unwrap_or("debug_decode".to_string());
    match case.as_str() {
        "debug_decode" => {
            let chronicle =
                AshWarChronicle::from_compatible_slice(&MOL_CHRONICLE_SCHEMA).expect("init");
            let dna = {
                let hexed_dna = env::args()
                    .nth(2)
                    .unwrap_or("ac7b88aabbcc687474703a2f2f3132372e302e302e".to_string());
                hex::decode(hexed_dna).expect("encode dna")
            };
            let player: Vec<ParsedDNA> = decode_player(chronicle.player(), dna.clone())
                .expect("player")
                .into();
            let scene: Vec<ParsedDNA> = decode_scene(chronicle.scene(), dna.clone())
                .expect("scene")
                .into();
            let context: Vec<ParsedDNA> = decode_context(chronicle.context(), dna.clone())
                .expect("context")
                .into();
            let event: Vec<ParsedDNA> = decode_event(chronicle.event(), dna).expect("event").into();

            println!("[玩家]\n{}\n", serde_json::to_string(&player).unwrap());
            println!("[场景]\n{}\n", serde_json::to_string(&scene).unwrap());
            println!("[背景]\n{}\n", serde_json::to_string(&context).unwrap());
            println!("[事件]\n{}\n", serde_json::to_string(&event).unwrap());
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
