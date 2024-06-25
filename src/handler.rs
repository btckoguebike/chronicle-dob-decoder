use alloc::vec::Vec;
use chronicle_schema::AshWarChronicle;
use molecule::prelude::Entity;

use crate::decoder::{decode_context, decode_event, decode_player, decode_scene};
use crate::error::Error;
use crate::generated::MOL_CHRONICLE_SCHEMA;
use crate::object::ParsedDNA;

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum ObjectType {
    Player,
    Scene,
    Context,
    Event,
}

impl From<u8> for ObjectType {
    fn from(value: u8) -> Self {
        match value % 4 {
            0 => Self::Player,
            1 => Self::Scene,
            2 => Self::Context,
            3 => Self::Event,
            _ => unreachable!(),
        }
    }
}

pub enum Parameters {
    Single(Vec<u8>),
    Multiple([Vec<u8>; 4]),
}

pub fn dobs_parse_parameters(args: Vec<&[u8]>) -> Result<Parameters, Error> {
    if args.is_empty() {
        return Err(Error::InvalidArgsLength);
    }
    if args.len() < 4 {
        let dna = {
            let dna = hex::decode(args[0]).map_err(|_| Error::InvalidHexedDNAInArgs)?;
            if dna.is_empty() {
                return Err(Error::InvalidEmptyDNA);
            }
            dna
        };
        Ok(Parameters::Single(dna))
    } else {
        let dna_set = args[..4]
            .iter()
            .map(|arg| {
                let dna = hex::decode(arg).map_err(|_| Error::InvalidHexedDNAInArgs)?;
                if dna.is_empty() {
                    return Err(Error::InvalidEmptyDNA);
                }
                Ok(dna)
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Parameters::Multiple(dna_set.try_into().unwrap()))
    }
}

pub fn dobs_decode(mut dna: Vec<u8>) -> Result<Vec<u8>, Error> {
    let chronicle =
        AshWarChronicle::from_compatible_slice(&MOL_CHRONICLE_SCHEMA).expect("chronicle init");
    let result: Vec<ParsedDNA> = match ObjectType::from(dna.remove(0)) {
        ObjectType::Player => decode_player(chronicle.player(), dna)?.into(),
        ObjectType::Scene => decode_scene(chronicle.scene(), dna)?.into(),
        ObjectType::Context => decode_context(chronicle.context(), dna)?.into(),
        ObjectType::Event => decode_event(chronicle.event(), dna)?.into(),
    };
    Ok(serde_json::to_string(&result)
        .expect("encode result")
        .as_bytes()
        .to_vec())
}

pub fn dobs_check_composable(dna_set: [Vec<u8>; 4]) -> Result<bool, Error> {
    let mut player = None;
    let mut scene = None;
    let mut context = None;
    let mut event = None;

    let chronicle =
        AshWarChronicle::from_compatible_slice(&MOL_CHRONICLE_SCHEMA).expect("chronicle init");
    dna_set.into_iter().try_for_each(|mut dna| {
        match ObjectType::from(dna.remove(0)) {
            ObjectType::Player => {
                player = Some(decode_player(chronicle.player(), dna)?);
            }
            ObjectType::Scene => {
                scene = Some(decode_scene(chronicle.scene(), dna)?);
            }
            ObjectType::Context => {
                context = Some(decode_context(chronicle.context(), dna)?);
            }
            ObjectType::Event => {
                event = Some(decode_event(chronicle.event(), dna)?);
            }
        };
        Result::<_, Error>::Ok(())
    })?;

    // Check if all objects are present
    let (Some(player), Some(scene), Some(environment), Some(chronicle)) =
        (player, scene, context, event)
    else {
        return Ok(false);
    };

    // Check if Character is combinable
    let mismatch = chronicle
        .player
        .into_iter()
        .enumerate()
        .any(|(i, ingredient)| {
            if let Some(ingredient) = ingredient {
                match i {
                    0 => player.adjective != ingredient,
                    1 => player.name != ingredient,
                    2 => player.profession != ingredient,
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
    let mismatch = chronicle
        .scene
        .into_iter()
        .enumerate()
        .any(|(i, ingredient)| {
            if let Some(ingredient) = ingredient {
                match i {
                    0 => scene.name != ingredient,
                    1 => scene.attribute != ingredient,
                    2 => scene.operation != ingredient,
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
    let mismatch = chronicle
        .context
        .into_iter()
        .enumerate()
        .any(|(i, ingredient)| {
            if let Some(ingredient) = ingredient {
                match i {
                    0 => environment.adjective != ingredient,
                    1 => environment.era != ingredient,
                    2 => environment.time != ingredient,
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
