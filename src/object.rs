use alloc::string::String;
use alloc::vec::Vec;

use serde::Serialize;

use crate::core::decoder::{decode_segment, Segment};
use crate::core::render::{RenderedSegment, SegmentRender};
use crate::error::Error;
use crate::generated::{character, date, location, story};

macro_rules! declare_object {
    (
        {
            $(($var:ident, $gnvar:ident),)+
        },
        $name:ident,
        $gns:ident,
        $rndred:ident,
        $rname:ident
    ) => {
        pub struct $name {
            $(
                pub $var: Segment,
            )+
        }

        pub struct $rndred {
            $(
                pub $var: RenderedSegment,
            )+
        }

        impl $name {
            pub fn from_generated() -> Result<Self, Error> {
                Ok(Self {
                    $(
                        $var: decode_segment($gns::$gnvar)?,
                    )+
                })
            }

            pub fn render_to_object(&self, segment_render: &SegmentRender, mut dna: Vec<u8>) -> Result<$rname, Error> {
                $rndred {
                    $(
                        $var: segment_render.render(&self.$var, &mut dna)?,
                    )+
                }.try_into()
            }

            pub fn render(&self, segment_render: &SegmentRender, dna: Vec<u8>) -> Result<String, Error> {
                let object = self.render_to_object(segment_render, dna)?;
                serde_json::to_string(&object).map_err(|_| Error::RenderRecoverToObjectError)
            }
        }
    };
}

declare_object!(
    {
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
    RawCharacter,
    character,
    RenderedCharacter,
    Character
);

declare_object!(
    {
        (adjective, ADJECTIVE),
        (name, NAME),
        (belonging, BELONGING),
        (coordinate, COORDINATE),
        (area, AREA),
        (color, COLOR),
        (commodity, COMMODITY),
    },
    RawLocation,
    location,
    RenderedLocation,
    Location
);

declare_object!(
    {
        (era, ERA),
        (year, YEAR),
        (time, TIME),
        (weather, WEATHER),
        (holiday, HOLIDAY),
        (season, SEASON),
        (background, BACKGROUND),
        (effect, EFFECT),
    },
    RawDate,
    date,
    RenderedDate,
    Date
);

declare_object!(
    {
        (character, CHARACTER),
        (location, LOCATION),
        (date, DATE),
        (story, STORY),
    },
    RawStory,
    story,
    RenderedStory,
    Story
);

#[derive(Serialize)]
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

impl TryFrom<RenderedCharacter> for Character {
    type Error = Error;

    fn try_from(value: RenderedCharacter) -> Result<Self, Self::Error> {
        Ok(Self {
            adjective: value.adjective.text()?,
            name: value.name.text()?,
            profession: value.profession.text()?,
            hp: value.hp.number()?,
            power: value.power.number()?,
            attack: value.attack.number()?,
            defense: value.defense.number()?,
            gold: value.gold.number()?,
            card: value.card.text_array()?,
        })
    }
}

#[derive(Serialize)]
pub struct Location {
    pub adjective: String,
    pub name: String,
    pub belonging: String,
    pub coordinate: [u8; 2],
    pub area: [u8; 2],
    pub color: [u8; 4],
    pub commodity: Vec<String>,
}

impl TryFrom<RenderedLocation> for Location {
    type Error = Error;

    fn try_from(value: RenderedLocation) -> Result<Self, Self::Error> {
        Ok(Self {
            adjective: value.adjective.text()?,
            name: value.name.text()?,
            belonging: value.belonging.text()?,
            coordinate: fixed_number_array(value.coordinate)?,
            area: fixed_number_array(value.area)?,
            color: fixed_number_array(value.color)?,
            commodity: value.commodity.text_array()?,
        })
    }
}

#[derive(Serialize)]
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

impl TryFrom<RenderedDate> for Date {
    type Error = Error;

    fn try_from(value: RenderedDate) -> Result<Self, Self::Error> {
        Ok(Self {
            era: value.era.text()?,
            year: value.year.number()?,
            time: value.time.text()?,
            weather: value.weather.text()?,
            holiday: value.holiday.text()?,
            season: value.season.text()?,
            background: fixed_number_array(value.background)?,
            effect: value.effect.text_array()?,
        })
    }
}

#[derive(Serialize)]
pub struct Story {
    pub character: [Option<String>; 3],
    pub location: [Option<String>; 3],
    pub date: [Option<String>; 3],
    pub story: [String; 4],
}

impl TryFrom<RenderedStory> for Story {
    type Error = Error;

    fn try_from(value: RenderedStory) -> Result<Self, Self::Error> {
        Ok(Self {
            character: fixed_story_multiple_array(value.character)?,
            location: fixed_story_multiple_array(value.location)?,
            date: fixed_story_multiple_array(value.date)?,
            story: fixed_string_array(value.story)?,
        })
    }
}

fn fixed_number_array<const N: usize>(value: RenderedSegment) -> Result<[u8; N], Error> {
    Ok(value
        .number_array()?
        .try_into()
        .map_err(|_| Error::MatchFixedNumberArrayError)?)
}

fn fixed_string_array<const N: usize>(value: RenderedSegment) -> Result<[String; N], Error> {
    Ok(value
        .text_array()?
        .try_into()
        .map_err(|_| Error::MatchFixedStringArrayError)?)
}

fn fixed_story_multiple_array<const N: usize>(
    value: RenderedSegment,
) -> Result<[Option<String>; N], Error> {
    value
        .multiple_array()?
        .into_iter()
        .map(|mut v| {
            if v.is_empty() {
                Ok(None)
            } else {
                Ok(Some(v.remove(0).text()?))
            }
        })
        .collect::<Result<Vec<_>, Error>>()?
        .try_into()
        .map_err(|_| Error::MatchFixedMultipleArrayError)
}

#[cfg(test)]
mod test {
    use lazy_static::lazy_static;

    use crate::core::decoder::Language;
    use crate::core::render::SegmentRender;
    use crate::object::{RawCharacter, RawDate, RawLocation, RawStory};

    const DNA: &str = "0a257cbbf6e9ef6ef62f1fb958ac5349cc985b404f26a7ea1dff13";

    lazy_static! {
        static ref SEGMENT_RENDER: SegmentRender =
            SegmentRender::new(Language::CN).expect("segment render");
    }

    #[test]
    fn test_render_character() {
        let render = RawCharacter::from_generated()
            .expect("new character")
            .render(&SEGMENT_RENDER, hex::decode(DNA).unwrap())
            .expect("render charactor");
        println!("{render:?}");
    }

    #[test]
    fn test_render_location() {
        let render = RawLocation::from_generated()
            .expect("new location")
            .render(&&SEGMENT_RENDER, hex::decode(DNA).unwrap())
            .expect("render charactor");
        println!("{render:?}");
    }

    #[test]
    fn test_render_date() {
        let render = RawDate::from_generated()
            .expect("new date")
            .render(&&SEGMENT_RENDER, hex::decode(DNA).unwrap())
            .expect("render charactor");
        println!("{render:?}");
    }

    #[test]
    fn test_render_story() {
        let render = RawStory::from_generated()
            .expect("new story")
            .render(&&SEGMENT_RENDER, hex::decode(DNA).unwrap())
            .expect("render story");
        println!("{render:?}");
    }
}
