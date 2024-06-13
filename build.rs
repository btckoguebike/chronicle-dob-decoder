use std::{fs, process::Command};

#[derive(serde::Serialize)]
pub struct LanguagePackage {
    pub trait_pool: String,
    pub template_pool: String,
    pub paragraph_pool: String,
}

pub fn main() {
    // // generate `character` module
    // let mut character_mod = String::from("pub mod character {");
    // [
    //     "adjective",
    //     "name",
    //     "profession",
    //     "hp",
    //     "power",
    //     "attack",
    //     "defense",
    //     "gold",
    //     "card",
    // ]
    // .into_iter()
    // .for_each(|file| {
    //     let content = fs::read_to_string(format!("./assets/schema/character/{file}.json"))
    //         .expect("read character_*.json")
    //         .replace("\"", "\\\"")
    //         .replace(" ", "")
    //         .replace("\n", "");
    //     character_mod.push_str(&format!(
    //         "pub const {}: &str = \"{content}\";",
    //         file.to_uppercase()
    //     ));
    // });
    // character_mod.push_str("}");

    // // gnerate `date` module
    // let mut date_mod = String::from("pub mod date {");
    // [
    //     "era",
    //     "year",
    //     "time",
    //     "weather",
    //     "holiday",
    //     "season",
    //     "background",
    //     "effect",
    // ]
    // .into_iter()
    // .for_each(|file| {
    //     let content = fs::read_to_string(format!("./assets/schema/date/{file}.json"))
    //         .expect("read date_*.json")
    //         .replace("\"", "\\\"")
    //         .replace(" ", "")
    //         .replace("\n", "");
    //     date_mod.push_str(&format!(
    //         "pub const {}: &str = \"{content}\";",
    //         file.to_uppercase()
    //     ));
    // });
    // date_mod.push_str("}");

    // // gnerate `location` module
    // let mut location_mod = String::from("pub mod location {");
    // [
    //     "adjective",
    //     "name",
    //     "belonging",
    //     "coordinate",
    //     "area",
    //     "color",
    //     "commodity",
    // ]
    // .into_iter()
    // .for_each(|file| {
    //     let content = fs::read_to_string(format!("./assets/schema/location/{file}.json"))
    //         .expect("read location_*.json")
    //         .replace("\"", "\\\"")
    //         .replace(" ", "")
    //         .replace("\n", "");
    //     location_mod.push_str(&format!(
    //         "pub const {}: &str = \"{content}\";",
    //         file.to_uppercase()
    //     ));
    // });
    // location_mod.push_str("}");

    // // gnerate `story` module
    // let mut story_mod = String::from("pub mod story {");
    // ["character", "location", "date", "story"]
    //     .into_iter()
    //     .for_each(|file| {
    //         let content = fs::read_to_string(format!("./assets/schema/story/{file}.json"))
    //             .expect("read story_*.json")
    //             .replace("\"", "\\\"")
    //             .replace(" ", "")
    //             .replace("\n", "");
    //         story_mod.push_str(&format!(
    //             "pub const {}: &str = \"{content}\";",
    //             file.to_uppercase()
    //         ));
    //     });
    // story_mod.push_str("}");

    // // generate `language` module
    // let mut language_mod = String::from("pub mod language {");
    // ["cn"].into_iter().for_each(|language| {
    //     let mut value_mod = format!("pub mod {language} {{");
    //     ["trait_pool", "template_pool", "paragraph_pool"]
    //         .into_iter()
    //         .for_each(|file| {
    //             let content =
    //                 fs::read_to_string(format!("./assets/language/{language}/{file}.json"))
    //                     .expect("read language_*.json")
    //                     .replace("\"", "\\\"")
    //                     .replace(" ", "")
    //                     .replace("\n", "");
    //             value_mod.push_str(&format!(
    //                 "pub const {}: &str = \"{content}\";",
    //                 file.to_uppercase()
    //             ));
    //         });
    //     value_mod.push_str("}");
    //     language_mod.push_str(&value_mod);
    // });
    // language_mod.push_str("}");

    // // generate `generated.rs`
    // let file_content = String::from("#![allow(dead_code)]\n")
    //     + &character_mod
    //     + &location_mod
    //     + &date_mod
    //     + &story_mod
    //     + &language_mod;
    // fs::write("./src/generated.rs", file_content).expect("write generated.rs");
    // Command::new("rustfmt")
    //     .arg("./src/generated.rs")
    //     .status()
    //     .expect("rustfmt generated.rs");

    // println!("cargo:rerun-if-changed=build.rs");
}
