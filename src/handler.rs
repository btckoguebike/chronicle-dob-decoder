use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::core::decoder::Language;
use crate::core::render::Render;
use crate::error::Error;
use crate::object::{Character, Date, Location, Story};

pub struct Parameters {
    pub dna: Vec<u8>,
    pub language: Language,
}

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum ObjectType {
    Character,
    Location,
    Date,
    Story,
}

impl From<u8> for ObjectType {
    fn from(value: u8) -> Self {
        match value % 4 {
            0 => Self::Character,
            1 => Self::Location,
            2 => Self::Date,
            3 => Self::Story,
            _ => unreachable!(),
        }
    }
}

pub fn dobs_parse_parameters(args: Vec<&[u8]>) -> Result<Parameters, Error> {
    if args.len() != 2 {
        return Err(Error::InvalidArgsLength);
    }
    let dna = {
        let dna = hex::decode(args[0]).map_err(|_| Error::InvalidHexedDNAInArgs)?;
        if dna.is_empty() {
            return Err(Error::InvalidEmptyDNA);
        }
        dna
    };
    let language = {
        let language =
            String::from_utf8(args[1].to_vec()).map_err(|_| Error::InvalidLanguageInArgs)?;
        language.try_into()?
    };
    Ok(Parameters { dna, language })
}

pub fn dobs_decode(parameters: Parameters) -> Result<Vec<u8>, Error> {
    let mut dna = parameters.dna;
    match ObjectType::from(dna.remove(0)) {
        ObjectType::Character => Character::new_from_generated()?.render(dna),
        ObjectType::Location => Location::new_from_generated()?.render(dna),
        ObjectType::Date => Date::new_from_generated()?.render(dna),
        ObjectType::Story => Story::new_from_generated()?.render(dna),
    }
    .map(|value| value.as_bytes().to_vec())
}

pub fn dobs_check_composable(dna_set: [String; 4]) -> Result<bool, Error> {
    let mut character = None;
    let mut location = None;
    let mut date = None;
    let mut story = None;

    dna_set.into_iter().try_for_each(|hexed_dna| {
        let mut dna = hex::decode(hexed_dna).map_err(|_| Error::InvalidHexedDNAInArgs)?;
        match ObjectType::from(dna.remove(0)) {
            ObjectType::Character => {
                character = Some(Character::new_from_generated()?.render_to_object(dna)?);
            }
            ObjectType::Location => {
                location = Some(Location::new_from_generated()?.render_to_object(dna)?);
            }
            ObjectType::Date => {
                date = Some(Date::new_from_generated()?.render_to_object(dna)?);
            }
            ObjectType::Story => {
                story = Some(Story::new_from_generated()?.render_to_object(dna)?);
            }
        };
        Result::<_, Error>::Ok(())
    })?;

    // Check if all objects are present
    let (Some(character), Some(location), Some(date), Some(story)) =
        (character, location, date, story)
    else {
        return Ok(false);
    };

    // Check if Character is combinable
    let mismatch = story
        .character
        .into_iter()
        .enumerate()
        .any(|(i, ingredient)| {
            if let Some(ingredient) = ingredient {
                match i {
                    0 => character.adjective != ingredient,
                    1 => character.name != ingredient,
                    2 => character.profession != ingredient,
                    _ => unreachable!(),
                }
            } else {
                false
            }
        });
    if mismatch {
        return Ok(false);
    }

    // Check if Location is combinable
    let mismatch = story
        .location
        .into_iter()
        .enumerate()
        .any(|(i, ingredient)| {
            if let Some(ingredient) = ingredient {
                match i {
                    0 => location.adjective != ingredient,
                    1 => location.name != ingredient,
                    2 => location.belonging != ingredient,
                    _ => unreachable!(),
                }
            } else {
                false
            }
        });
    if mismatch {
        return Ok(false);
    }

    // Check if Date is combinable
    let mismatch = story.date.into_iter().enumerate().any(|(i, ingredient)| {
        if let Some(ingredient) = ingredient {
            match i {
                0 => date.era != ingredient,
                1 => date.year.to_string() != ingredient,
                2 => date.time != ingredient,
                _ => unreachable!(),
            }
        } else {
            false
        }
    });
    if mismatch {
        return Ok(false);
    }

    Ok(true)
}
