use alloc::vec::Vec;
use chronicle_schema::AshWarChronicle;
use molecule::prelude::Entity;

use crate::decoder::{decode_chronicle, decode_environment, decode_player, decode_scene};
use crate::error::Error;
use crate::generated::MOL_CHRONICLE_SCHEMA;
use crate::object::ParsedDNA;

#[repr(u8)]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
enum ObjectType {
    Player,
    Scene,
    Environment,
    Chronicle,
}

impl From<u8> for ObjectType {
    fn from(value: u8) -> Self {
        match value % 4 {
            0 => Self::Player,
            1 => Self::Scene,
            2 => Self::Environment,
            3 => Self::Chronicle,
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
    let awc =
        AshWarChronicle::from_compatible_slice(&MOL_CHRONICLE_SCHEMA).expect("chronicle init");
    let result: Vec<ParsedDNA> = match ObjectType::from(dna.remove(0)) {
        ObjectType::Player => decode_player(awc.player(), dna)?.into(),
        ObjectType::Scene => decode_scene(awc.scene(), dna)?.into(),
        ObjectType::Environment => decode_environment(awc.envionment(), dna)?.into(),
        ObjectType::Chronicle => decode_chronicle(awc.chronicle(), dna)?.into(),
    };
    Ok(serde_json::to_string(&result)
        .expect("encode result")
        .as_bytes()
        .to_vec())
}

pub fn dobs_check_composable(dna_set: [Vec<u8>; 4]) -> Result<bool, Error> {
    let mut player = None;
    let mut scene = None;
    let mut environment = None;
    let mut chronicle = None;

    let awc =
        AshWarChronicle::from_compatible_slice(&MOL_CHRONICLE_SCHEMA).expect("chronicle init");
    dna_set.into_iter().try_for_each(|mut dna| {
        match ObjectType::from(dna.remove(0)) {
            ObjectType::Player => {
                player = Some(decode_player(awc.player(), dna)?);
            }
            ObjectType::Scene => {
                scene = Some(decode_scene(awc.scene(), dna)?);
            }
            ObjectType::Environment => {
                environment = Some(decode_environment(awc.envionment(), dna)?);
            }
            ObjectType::Chronicle => {
                chronicle = Some(decode_chronicle(awc.chronicle(), dna)?);
            }
        };
        Result::<_, Error>::Ok(())
    })?;

    // Check if all objects are present
    let (Some(player), Some(scene), Some(environment), Some(chronicle)) =
        (player, scene, environment, chronicle)
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
        .environment
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
