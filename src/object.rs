use alloc::string::{String, ToString};
use alloc::{vec, vec::Vec};

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
pub struct Player {
    pub adjective: String,
    pub name: String,
    pub profession: String,
    pub power: u8,
    pub gold: u8,
    pub card: Vec<String>,
}

impl From<Player> for Vec<ParsedDNA> {
    fn from(value: Player) -> Self {
        vec![
            parse_single!(value, adjective, String),
            parse_single!(value, name, String),
            parse_single!(value, profession, String),
            parse_single!(value, power, Number),
            parse_single!(value, gold, Number),
            parse_multiple!(value, card, String),
        ]
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Scene {
    pub name: String,
    pub attribute: String,
    pub operation: String,
    pub score: u8,
    pub difficulty: u8,
    pub commodity: Vec<String>,
}

impl From<Scene> for Vec<ParsedDNA> {
    fn from(value: Scene) -> Self {
        vec![
            parse_single!(value, name, String),
            parse_single!(value, attribute, String),
            parse_single!(value, operation, String),
            parse_single!(value, score, Number),
            parse_single!(value, difficulty, Number),
            parse_multiple!(value, commodity, String),
        ]
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Context {
    pub adjective: String,
    pub era: String,
    pub time: String,
    pub mode: String,
    pub rank: u8,
    pub effect: Vec<String>,
}

impl From<Context> for Vec<ParsedDNA> {
    fn from(value: Context) -> Self {
        vec![
            parse_single!(value, adjective, String),
            parse_single!(value, era, String),
            parse_single!(value, time, String),
            parse_single!(value, mode, String),
            parse_single!(value, rank, Number),
            parse_multiple!(value, effect, String),
        ]
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Event {
    pub player: [Option<String>; 3],
    pub scene: [Option<String>; 3],
    pub context: [Option<String>; 3],
    pub transition: String,
    pub climax: String,
    pub ending: String,
}

impl From<Event> for Vec<ParsedDNA> {
    fn from(value: Event) -> Self {
        vec![
            parse_single!(value, player, String, |v: [Option<String>; 3]| v
                .map(|value| value.unwrap_or("_".to_string()))
                .join("|")),
            parse_single!(value, scene, String, |v: [Option<String>; 3]| v
                .map(|value| value.unwrap_or("_".to_string()))
                .join("|")),
            parse_single!(value, context, String, |v: [Option<String>; 3]| v
                .map(|value| value.unwrap_or("_".to_string()))
                .join("|")),
            parse_single!(value, transition, String),
            parse_single!(value, climax, String),
            parse_single!(value, ending, String),
        ]
    }
}
