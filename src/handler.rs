use alloc::string::ToString;
use alloc::vec::Vec;
use molecule::prelude::Entity;

use crate::decoder::{decode_character, decode_date, decode_location, decode_story};
use crate::error::Error;
use crate::generated::MOL_CHRONICLE_SCHEMA;
use crate::object::ParsedDNA;
use crate::schema::AshWarChronicle;

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

pub fn dobs_parse_parameters(args: Vec<&[u8]>) -> Result<Vec<u8>, Error> {
    if args.is_empty() {
        return Err(Error::InvalidArgsLength);
    }
    let dna = {
        let dna = hex::decode(args[0]).map_err(|_| Error::InvalidHexedDNAInArgs)?;
        if dna.is_empty() {
            return Err(Error::InvalidEmptyDNA);
        }
        dna
    };
    Ok(dna)
}

pub fn dobs_decode(mut dna: Vec<u8>) -> Result<Vec<u8>, Error> {
    let chronicle =
        AshWarChronicle::from_compatible_slice(MOL_CHRONICLE_SCHEMA).expect("chronicle init");
    let result: Vec<ParsedDNA> = match ObjectType::from(dna.remove(0)) {
        ObjectType::Character => decode_character(chronicle.character_schema(), dna)?.into(),
        ObjectType::Location => decode_location(chronicle.location_schema(), dna)?.into(),
        ObjectType::Date => decode_date(chronicle.date_schema(), dna)?.into(),
        ObjectType::Story => decode_story(chronicle.story_schema(), dna)?.into(),
    };
    Ok(serde_json::to_string(&result)
        .expect("encode result")
        .as_bytes()
        .to_vec())
}

pub fn dobs_check_composable(dna_set: [Vec<u8>; 4]) -> Result<bool, Error> {
    let mut character = None;
    let mut location = None;
    let mut date = None;
    let mut story = None;

    let chronicle =
        AshWarChronicle::from_compatible_slice(MOL_CHRONICLE_SCHEMA).expect("chronicle init");
    dna_set.into_iter().try_for_each(|mut dna| {
        match ObjectType::from(dna.remove(0)) {
            ObjectType::Character => {
                character = Some(decode_character(chronicle.character_schema(), dna)?);
            }
            ObjectType::Location => {
                location = Some(decode_location(chronicle.location_schema(), dna)?);
            }
            ObjectType::Date => {
                date = Some(decode_date(chronicle.date_schema(), dna)?);
            }
            ObjectType::Story => {
                story = Some(decode_story(chronicle.story_schema(), dna)?);
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
