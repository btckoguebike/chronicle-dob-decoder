use std::{fs, process::Command};

pub fn main() {
    // generate `character` module
    let mut character_mod = String::from("pub mod character {");
    [
        "adjective",
        "name",
        "profession",
        "hp",
        "power",
        "attack",
        "defense",
        "gold",
        "card",
    ]
    .into_iter()
    .for_each(|file| {
        let content = fs::read_to_string(format!("./assets/schema/character/{file}.json"))
            .expect("read character_*.json")
            .replace("\"", "\\\"")
            .replace(" ", "")
            .replace("\n", "");
        character_mod.push_str(&format!(
            "pub const {}: &str = \"{content}\";",
            file.to_uppercase()
        ));
    });
    character_mod.push_str("}");

    // generate `language` module
    let mut language_mod = String::from("pub mod language {");
    ["cn"].into_iter().for_each(|value| {
        let mut value_mod = format!("pub mod {value} {{");
        ["trait_pool", "template_pool"]
            .into_iter()
            .for_each(|file| {
                let content = fs::read_to_string(format!("./assets/language/{value}/{file}.json"))
                    .expect("read language_*.json")
                    .replace("\"", "\\\"")
                    .replace(" ", "")
                    .replace("\n", "");
                value_mod.push_str(&format!(
                    "pub const {}: &str = \"{content}\";",
                    file.to_uppercase()
                ));
            });
        value_mod.push_str("}");
        language_mod.push_str(&value_mod);
    });
    language_mod.push_str("}");

    let file_content = String::from("#![allow(dead_code)]\n") + &character_mod + &language_mod;
    fs::write("./src/generated.rs", file_content).expect("write generated.rs");
    Command::new("rustfmt")
        .arg("./src/generated.rs")
        .status()
        .expect("rustfmt generated.rs");

    println!("cargo:rerun-if-changed=build.rs");
}
