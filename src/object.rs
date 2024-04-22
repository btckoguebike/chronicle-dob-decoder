use alloc::string::String;
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};

use crate::core::decoder::{decode_segment, Segment};
use crate::core::render::Render;
use crate::error::Error;
use crate::generated::{character, date, location, story};

macro_rules! declare_object {
    (
        $name:ident {
            $(($var:ident, $gnvar:ident),)+
        },
        $gns:ident,
        $rndred:ident
    ) => {
        #[derive(Serialize)]
        pub struct $name {
            $(
                pub $var: Segment,
            )+
        }

        impl $name {
            pub fn new_from_generated() -> Result<Self, Error> {
                Ok(Self {
                    $(
                        $var: decode_segment($gns::$gnvar)?,
                    )+
                })
            }

            pub fn render_to_object(&self, dna: Vec<u8>) -> Result<$rndred, Error> {
                let render = self.render(dna)?;
                Ok(serde_json::from_str(&render).map_err(|_| Error::RenderToObjectError)?)
            }
        }

        impl Render for $name {}
    };
}

declare_object!(
    Character {
        (adjective, ADJECTIVE),
        (name, NAME),
        (profession, PROFESSION),
        (hp, HP),
        (power, POWER),
        (attack, ATTACK),
        (defense, DEFENSE),
        (gold, GOLD),
        (card, CARD),
    },
    character,
    RenderedCharacter
);

declare_object!(
    Location {
        (adjective, ADJECTIVE),
        (name, NAME),
        (belonging, BELONGING),
        (coordinate, COORDINATE),
        (area, AREA),
        (color, COLOR),
        (commodity, COMMODITY),
    },
    location,
    RenderedLocation
);

declare_object!(
    Date {
        (era, ERA),
        (year, YEAR),
        (time, TIME),
        (weather, WEATHER),
        (holiday, HOLIDAY),
        (season, SEASON),
        (background, BACKGROUND),
        (effect, EFFECT),
    },
    date,
    RenderedDate
);

declare_object!(
    Story {
        (character, CHARACTER),
        (location, LOCATION),
        (date, DATE),
        (story, STORY),
    },
    story,
    RenderedStory
);

#[derive(Deserialize, Debug)]
pub struct RenderedCharacter {
    pub adjective: String,
    pub name: String,
    pub profession: String,
    #[serde(deserialize_with = "number_adapter")]
    pub hp: u8,
    #[serde(deserialize_with = "number_adapter")]
    pub power: u8,
    #[serde(deserialize_with = "number_adapter")]
    pub attack: u8,
    #[serde(deserialize_with = "number_adapter")]
    pub defense: u8,
    #[serde(deserialize_with = "number_adapter")]
    pub gold: u8,
    pub card: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct RenderedLocation {
    pub adjective: String,
    pub name: String,
    pub belonging: String,
    #[serde(deserialize_with = "number_array_adapter")]
    pub coordinate: [u8; 2],
    #[serde(deserialize_with = "number_array_adapter")]
    pub area: [u8; 2],
    #[serde(deserialize_with = "number_array_adapter")]
    pub color: [u8; 4],
    pub commodity: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct RenderedDate {
    pub era: String,
    #[serde(deserialize_with = "number_adapter")]
    pub year: u8,
    pub time: String,
    pub weather: String,
    pub holiday: String,
    pub season: String,
    #[serde(deserialize_with = "number_array_adapter")]
    pub background: [u8; 4],
    pub effect: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub struct RenderedStory {
    #[serde(deserialize_with = "ingredient_array_adapter")]
    pub character: [Option<String>; 3],
    #[serde(deserialize_with = "ingredient_array_adapter")]
    pub location: [Option<String>; 3],
    #[serde(deserialize_with = "ingredient_array_adapter")]
    pub date: [Option<String>; 3],
    pub story: [String; 4],
}

fn number_adapter<'de, D>(deserializer: D) -> Result<u8, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let number_string: String = serde::Deserialize::deserialize(deserializer)?;
    u8::from_str_radix(&number_string, 10)
        .map_err(|_| serde::de::Error::custom(Error::ParseRenderedNumberError))
}

fn number_array_adapter<'de, D, const N: usize>(deserializer: D) -> Result<[u8; N], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let number_string_array: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
    let number_array: [u8; N] = number_string_array
        .into_iter()
        .map(|number| u8::from_str_radix(&number, 10))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| serde::de::Error::custom(Error::ParseRenderedNumberError))?
        .try_into()
        .map_err(|_| serde::de::Error::custom(Error::ParseRenderedNumbeArrayCountError))?;
    Ok(number_array)
}

fn ingredient_array_adapter<'de, D>(deserializer: D) -> Result<[Option<String>; 3], D::Error>
where
    D: serde::Deserializer<'de>,
{
    let ingredients: [Vec<String>; 3] = serde::Deserialize::deserialize(deserializer)?;
    Ok(ingredients.map(|mut value| {
        if value.is_empty() {
            None
        } else {
            Some(value.remove(0))
        }
    }))
}

#[cfg(test)]
mod test {
    use crate::core::render::Render;
    use crate::object::{
        Character, Date, Location, RenderedCharacter, RenderedDate, RenderedLocation,
        RenderedStory, Story,
    };

    const DNA: &str = "0a257cbbf6e9ef6ef62f1fb958ac5349cc985b404f26a7ea1dff13";

    #[test]
    fn test_render_character() {
        let render = Character::new_from_generated()
            .expect("new character")
            .render(hex::decode(DNA).unwrap())
            .expect("render charactor");
        let character: RenderedCharacter = serde_json::from_str(&render).expect("parse render");
        println!("{character:?}");
    }

    #[test]
    fn test_render_location() {
        let render = Location::new_from_generated()
            .expect("new location")
            .render(hex::decode(DNA).unwrap())
            .expect("render charactor");
        let location: RenderedLocation = serde_json::from_str(&render).expect("parse render");
        println!("{location:?}");
    }

    #[test]
    fn test_render_date() {
        let render = Date::new_from_generated()
            .expect("new date")
            .render(hex::decode(DNA).unwrap())
            .expect("render charactor");
        let date: RenderedDate = serde_json::from_str(&render).expect("parse render");
        println!("{date:?}");
    }

    #[test]
    fn test_render_story() {
        let render = Story::new_from_generated()
            .expect("new story")
            .render(hex::decode(DNA).unwrap())
            .expect("render story");
        let story: RenderedStory = serde_json::from_str(&render).expect("parse render");
        println!("{story:?}");
    }
}
