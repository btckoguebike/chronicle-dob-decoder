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
    fn from(player: Player) -> Self {
        vec![
            parse_single!(player, adjective, String),
            parse_single!(player, name, String),
            parse_single!(player, profession, String),
            parse_single!(player, power, Number),
            parse_single!(player, gold, Number),
            parse_multiple!(player, card, String),
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
    fn from(scene: Scene) -> Self {
        vec![
            parse_single!(scene, name, String),
            parse_single!(scene, attribute, String),
            parse_single!(scene, operation, String),
            parse_single!(scene, score, Number),
            parse_single!(scene, difficulty, Number),
            parse_multiple!(scene, commodity, String),
        ]
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Environment {
    pub adjective: String,
    pub era: String,
    pub time: String,
    pub mode: String,
    pub rank: u8,
    pub effect: Vec<String>,
}

impl From<Environment> for Vec<ParsedDNA> {
    fn from(environment: Environment) -> Self {
        vec![
            parse_single!(environment, adjective, String),
            parse_single!(environment, era, String),
            parse_single!(environment, time, String),
            parse_single!(environment, mode, String),
            parse_single!(environment, rank, Number),
            parse_multiple!(environment, effect, String),
        ]
    }
}

#[cfg_attr(test, derive(Debug))]
pub struct Chronicle {
    pub player: [Option<String>; 3],
    pub scene: [Option<String>; 3],
    pub environment: [Option<String>; 3],
    pub transition: String,
    pub climax: String,
    pub ending: String,
}

impl From<Chronicle> for Vec<ParsedDNA> {
    fn from(value: Chronicle) -> Self {
        vec![
            parse_single!(value, player, String, |v: [Option<String>; 3]| v
                .map(|value| value.unwrap_or("_".to_string()))
                .join("|")),
            parse_single!(value, scene, String, |v: [Option<String>; 3]| v
                .map(|value| value.unwrap_or("_".to_string()))
                .join("|")),
            parse_single!(value, environment, String, |v: [Option<String>; 3]| v
                .map(|value| value.unwrap_or("_".to_string()))
                .join("|")),
            parse_single!(value, transition, String),
            parse_single!(value, climax, String),
            parse_single!(value, ending, String),
        ]
    }
}
