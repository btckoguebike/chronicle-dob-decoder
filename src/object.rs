use alloc::string::{String, ToString};
use alloc::{format, vec, vec::Vec};

use serde::ser::SerializeMap as _;
use serde::Serialize;
use serde_json::Value;

pub struct ParsedTrait {
    pub type_: String,
    pub value: Value,
}

impl Serialize for ParsedTrait {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map = serializer.serialize_map(Some(1))?;
        map.serialize_entry(&self.type_, &self.value)?;
        map.end()
    }
}

#[derive(serde::Serialize)]
pub struct ParsedDNA {
    pub name: String,
    pub traits: Vec<ParsedTrait>,
}

macro_rules! parse_single {
    ($object:ident, $member:ident, $ty:ident) => {
        ParsedDNA {
            name: stringify!($member).to_string(),
            traits: vec![ParsedTrait {
                type_: stringify!($ty).to_string(),
                value: Value::$ty($object.$member.into()),
            }],
        }
    };
    ($object:ident, $member:ident, $ty:ident, $convert:expr) => {
        ParsedDNA {
            name: stringify!($member).to_string(),
            traits: vec![ParsedTrait {
                type_: stringify!($ty).to_string(),
                value: Value::$ty($convert($object.$member)),
            }],
        }
    };
}

macro_rules! parse_multiple {
    ($object:ident, $member:ident, $ty:ident) => {
        ParsedDNA {
            name: stringify!($member).to_string(),
            traits: $object
                .$member
                .into_iter()
                .map(|value| ParsedTrait {
                    type_: stringify!($ty).to_string(),
                    value: Value::$ty(value.into()),
                })
                .collect(),
        }
    };
}

#[cfg_attr(test, derive(Debug))]
pub struct Character {
    pub adjective: String,
    pub name: String,
    pub profession: String,
    pub hp: u8,
    pub power: u8,
    pub attack: u8,
    pub defense: u8,
    pub gold: u8,
    pub card: Vec<String>,
}

impl From<Character> for Vec<ParsedDNA> {
    fn from(character: Character) -> Self {
        let mut parsed_dna = Vec::new();
        parsed_dna.push(parse_single!(character, adjective, String));
        parsed_dna.push(parse_single!(character, name, String));
        parsed_dna.push(parse_single!(character, profession, String));
        parsed_dna.push(parse_single!(character, hp, Number));
        parsed_dna.push(parse_single!(character, power, Number));
        parsed_dna.push(parse_single!(character, attack, Number));
        parsed_dna.push(parse_single!(character, defense, Number));
        parsed_dna.push(parse_single!(character, gold, Number));
        parsed_dna.push(parse_multiple!(character, card, String));
        parsed_dna
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Location {
    pub adjective: String,
    pub name: String,
    pub belonging: String,
    pub coordinate: [u8; 2],
    pub area: [u8; 2],
    pub color: [u8; 4],
    pub commodity: Vec<String>,
}

impl From<Location> for Vec<ParsedDNA> {
    fn from(location: Location) -> Self {
        let mut parsed_dna = Vec::new();
        parsed_dna.push(parse_single!(location, adjective, String));
        parsed_dna.push(parse_single!(location, name, String));
        parsed_dna.push(parse_single!(location, belonging, String));
        parsed_dna.push(parse_single!(
            location,
            coordinate,
            String,
            |v: [u8; 2]| format!("({}, {})", v[0], v[1])
        ));
        parsed_dna.push(parse_single!(location, area, String, |v: [u8; 2]| format!(
            "{} x {}",
            v[0], v[1]
        )));
        parsed_dna.push(parse_single!(
            location,
            color,
            String,
            |v: [u8; 4]| format!("#{:02X}{:02X}{:02X}{:02X}", v[0], v[1], v[2], v[3])
        ));
        parsed_dna.push(parse_multiple!(location, commodity, String));
        parsed_dna
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Date {
    pub era: String,
    pub year: u8,
    pub time: String,
    pub weather: String,
    pub holiday: String,
    pub season: String,
    pub background: [u8; 4],
    pub effect: Vec<String>,
}

impl From<Date> for Vec<ParsedDNA> {
    fn from(date: Date) -> Self {
        let mut parsed_dna = Vec::new();
        parsed_dna.push(parse_single!(date, era, String));
        parsed_dna.push(parse_single!(date, year, Number));
        parsed_dna.push(parse_single!(date, time, String));
        parsed_dna.push(parse_single!(date, weather, String));
        parsed_dna.push(parse_single!(date, holiday, String));
        parsed_dna.push(parse_single!(date, season, String));
        parsed_dna.push(parse_single!(
            date,
            background,
            String,
            |v: [u8; 4]| format!("#{:02X}{:02X}{:02X}{:02X}", v[0], v[1], v[2], v[3])
        ));
        parsed_dna.push(parse_multiple!(date, effect, String));
        parsed_dna
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Story {
    pub character: [Option<String>; 3],
    pub location: [Option<String>; 3],
    pub date: [Option<String>; 3],
    pub event: [String; 4],
}

impl From<Story> for Vec<ParsedDNA> {
    fn from(value: Story) -> Self {
        let mut parsed_dna = Vec::new();
        parsed_dna.push(parse_single!(
            value,
            character,
            String,
            |v: [Option<String>; 3]| v.map(|value| value.unwrap_or("_".to_string())).join("|")
        ));
        parsed_dna.push(parse_single!(
            value,
            location,
            String,
            |v: [Option<String>; 3]| v.map(|value| value.unwrap_or("_".to_string())).join("|")
        ));
        parsed_dna.push(parse_single!(
            value,
            date,
            String,
            |v: [Option<String>; 3]| v.map(|value| value.unwrap_or("_".to_string())).join("|")
        ));
        parsed_dna.push(parse_multiple!(value, event, String));
        parsed_dna
    }
}
