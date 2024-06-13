use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::{IntoIter, Vec},
};
use serde_json::Value;

use crate::{error::Error, schema::LocationSchema};
use crate::{
    object::Story,
    schema::{
        CharacterSchema, Instruction, InstructionUnion, PatternUnion, PoolUnion, Schema, Segment,
        StorySchema, VariableSegment,
    },
};
use crate::{
    object::{Character, Date, Location},
    schema::DateSchema,
};

struct PoolSelector<T: Clone> {
    array: Vec<T>,
    bytes: Vec<u8>,
}

impl<T: Clone> PoolSelector<T> {
    fn new(array: Vec<T>, bytes: Vec<u8>) -> Result<Self, Error> {
        if array.is_empty() {
            return Err(Error::DecodePoolEmptyError);
        }
        Ok(Self { array, bytes })
    }

    fn new_range(range: impl Iterator<Item = T>, bytes: Vec<u8>) -> Result<Self, Error> {
        let array = range.collect();
        Self::new(array, bytes)
    }

    fn select(self) -> Result<T, Error> {
        let indicator = match self.bytes.len() {
            1 => self.bytes[0] as usize,
            2 => u16::from_le_bytes(self.bytes.try_into().unwrap()) as usize,
            4 => u32::from_le_bytes(self.bytes.try_into().unwrap()) as usize,
            8 => u64::from_le_bytes(self.bytes.try_into().unwrap()) as usize,
            _ => return Err(Error::DecodeAssembleSingleSelectorError),
        };
        let offset = indicator % self.array.len();
        Ok(self.array[offset].clone())
    }
}

fn decode_segment(segment: Segment, dna_bytes: &mut Vec<u8>) -> Result<Value, Error> {
    let occupied = u8::from(segment.segment_bytes()) as usize;
    let occupied_bytes = dna_bytes.splice(0..occupied, vec![]).collect::<Vec<_>>();
    let result = match segment.pool().to_enum() {
        PoolUnion::TraitPool(value) => {
            let value = value
                .into_iter()
                .map(|v| String::from_utf8_lossy(&v.raw_data()).to_string())
                .collect();
            let value = PoolSelector::new(value, occupied_bytes)?.select()?;
            Value::String(value)
        }
        PoolUnion::NumberPool(value) => {
            let value = value.into_iter().map(|v| u8::from(v)).collect();
            let value = PoolSelector::new(value, occupied_bytes)?.select()?;
            Value::Number(value.into())
        }
        PoolUnion::NumberRange(value) => {
            let min: u8 = value.min().into();
            let max: u8 = value.max().into();
            let value = PoolSelector::new_range(min..=max, occupied_bytes)?.select()?;
            Value::Number(value.into())
        }
        PoolUnion::TemplatePool(value) => {
            if value.is_empty() {
                Value::String(Default::default())
            } else {
                let value = value
                    .into_iter()
                    .map(|v| v.into_iter().collect::<Vec<_>>().into_iter())
                    .collect();
                let instructions = PoolSelector::new(value, occupied_bytes)?.select()?;
                let value = decode_template_instructions(instructions, dna_bytes)?;
                Value::String(value)
            }
        }
    };
    Ok(result)
}

fn decode_template_instructions(
    instructions: IntoIter<Instruction>,
    dna_bytes: &mut Vec<u8>,
) -> Result<String, Error> {
    let mut results = vec![];
    let mut pending_values = vec![];
    for instruction in instructions {
        match instruction.to_enum() {
            InstructionUnion::NumberPool(value) => {
                let value = value.into_iter().map(|v| u8::from(v)).collect();
                let value =
                    PoolSelector::new(value, dna_bytes.splice(0..1, vec![]).collect())?.select()?;
                pending_values.push(value);
            }
            InstructionUnion::NumberRange(value) => {
                let min: u8 = value.min().into();
                let max: u8 = value.max().into();
                let value =
                    PoolSelector::new_range(min..=max, dna_bytes.splice(0..1, vec![]).collect())?
                        .select()?;
                pending_values.push(value);
            }
            InstructionUnion::UTF8Bytes(template) => {
                let template = String::from_utf8_lossy(&template.raw_data()).to_string();
                match pending_values.len() {
                    0 => results.push(template),
                    1 => {
                        let parts = template.split("x").collect::<Vec<_>>();
                        assert!(parts.len() == 2);
                        results.push(format!(
                            "{}{}{}",
                            parts[0],
                            pending_values.remove(0),
                            parts[1]
                        ))
                    }
                    2 => {
                        let parts = template.split("x").collect::<Vec<_>>();
                        assert!(parts.len() == 3);
                        results.push(format!(
                            "{}{}{}{}{}",
                            parts[0],
                            pending_values.remove(0),
                            parts[1],
                            pending_values.remove(0),
                            parts[2]
                        ));
                    }
                    _ => return Err(Error::DecodeTemplateElementsCountError),
                }
            }
        }
    }
    Ok(results.join(" => "))
}

fn decode_variable_segment(
    variable: VariableSegment,
    dna_bytes: &mut Vec<u8>,
) -> Result<Vec<Value>, Error> {
    let occupied = u8::from(variable.count().segment_bytes()) as usize;
    let occupied_bytes = dna_bytes.splice(0..occupied, vec![]).collect::<Vec<_>>();
    let count = match variable.count().pool().to_enum() {
        PoolUnion::NumberPool(value) => {
            let value = value.into_iter().map(|v| u8::from(v)).collect();
            PoolSelector::new(value, occupied_bytes)?.select()?
        }
        PoolUnion::NumberRange(value) => {
            let min: u8 = value.min().into();
            let max: u8 = value.max().into();
            PoolSelector::new_range(min..=max, occupied_bytes)?.select()?
        }
        _ => return Err(Error::DecodeVariableNumberPoolError),
    };
    if count as usize > variable.segments().len() {
        return Err(Error::DecodePatternCountError);
    }
    (0..count)
        .map(|_| {
            let segment = variable.segments().into_iter().next().unwrap();
            decode_segment(segment, dna_bytes)
        })
        .collect::<Result<Vec<_>, _>>()
}

fn decode_schema(schema: Schema, dna_bytes: &mut Vec<u8>) -> Result<Value, Error> {
    let occupied = u8::from(schema.occupied_bytes()) as usize;
    let mut occupied_bytes = dna_bytes.splice(0..occupied, vec![]).collect::<Vec<_>>();
    match schema.pattern().to_enum() {
        PatternUnion::Segment(segment) => {
            let value = decode_segment(segment, &mut occupied_bytes)?;
            Ok(value)
        }
        PatternUnion::SegmentVec(segments) => {
            let value = segments
                .into_iter()
                .map(|pattern| decode_segment(pattern, &mut occupied_bytes))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(value))
        }
        PatternUnion::VariableSegment(variable) => {
            let value = decode_variable_segment(variable, &mut occupied_bytes)?
                .into_iter()
                .collect::<Vec<_>>();
            Ok(Value::Array(value))
        }
        PatternUnion::VariableSegmentVec(variables) => {
            let value = variables
                .into_iter()
                .map(|variable| {
                    decode_variable_segment(variable, &mut occupied_bytes)
                        .map(|values| Value::Array(values.into_iter().collect::<Vec<_>>()))
                })
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(value))
        }
    }
}

macro_rules! parse_string {
    ($field:ident, $error:ident) => {
        $field.as_str().ok_or(Error::$error)?.to_string()
    };
}

macro_rules! parse_number {
    ($field:ident, $error:ident) => {
        $field.as_u64().ok_or(Error::$error)? as u8
    };
}

macro_rules! parse_number_array {
    ($field:ident, $error:ident) => {
        $field
            .as_array()
            .ok_or(Error::$error)?
            .into_iter()
            .map(|value| Ok(parse_number!(value, $error)))
            .collect::<Result<Vec<_>, Error>>()?
            .try_into()
            .map_err(|_| Error::$error)?
    };
}

macro_rules! parse_string_array {
    ($field:ident, $error:ident) => {
        $field
            .as_array()
            .ok_or(Error::$error)?
            .into_iter()
            .map(|value| Ok(parse_string!(value, $error)))
            .collect::<Result<Vec<_>, Error>>()?
    };
    ($field:ident, $error:ident, $handle:expr) => {
        $field
            .as_array()
            .ok_or(Error::$error)?
            .into_iter()
            .map(|value| $handle(value, Error::$error))
            .collect::<Result<Vec<_>, Error>>()?
    };
}

pub fn decode_character(
    character: CharacterSchema,
    mut dna_bytes: Vec<u8>,
) -> Result<Character, Error> {
    let adjective = decode_schema(character.adjective(), &mut dna_bytes)?;
    let name = decode_schema(character.name(), &mut dna_bytes)?;
    let profession = decode_schema(character.profession(), &mut dna_bytes)?;
    let hp = decode_schema(character.hp(), &mut dna_bytes)?;
    let power = decode_schema(character.power(), &mut dna_bytes)?;
    let attack = decode_schema(character.attack(), &mut dna_bytes)?;
    let defense = decode_schema(character.defense(), &mut dna_bytes)?;
    let gold = decode_schema(character.gold(), &mut dna_bytes)?;
    let card = decode_schema(character.card(), &mut dna_bytes)?;

    Ok(Character {
        adjective: parse_string!(adjective, ParseCharacterAdjectiveError),
        name: parse_string!(name, ParseCharacterNameError),
        profession: parse_string!(profession, ParseCharacterProfessionError),
        hp: parse_number!(hp, ParseCharacterHpError),
        power: parse_number!(power, ParseCharacterPowerError),
        attack: parse_number!(attack, ParseCharacterAttackError),
        defense: parse_number!(defense, ParseCharacterDefenseError),
        gold: parse_number!(gold, ParseCharacterGoldError),
        card: parse_string_array!(card, ParseCharacterCardError),
    })
}

pub fn decode_location(
    location: LocationSchema,
    mut dna_bytes: Vec<u8>,
) -> Result<Location, Error> {
    let adjective = decode_schema(location.adjective(), &mut dna_bytes)?;
    let name = decode_schema(location.name(), &mut dna_bytes)?;
    let belonging = decode_schema(location.belonging(), &mut dna_bytes)?;
    let coordinate = decode_schema(location.coordinate(), &mut dna_bytes)?;
    let area = decode_schema(location.area(), &mut dna_bytes)?;
    let color = decode_schema(location.color(), &mut dna_bytes)?;
    let commodity = decode_schema(location.commodity(), &mut dna_bytes)?;

    Ok(Location {
        adjective: parse_string!(adjective, ParseLocationAdjectiveError),
        name: parse_string!(name, ParseLocationNameError),
        belonging: parse_string!(belonging, ParseLocationBelongingError),
        coordinate: parse_number_array!(coordinate, ParseLocationCoordinateError),
        area: parse_number_array!(area, ParseLocationAreaError),
        color: parse_number_array!(color, ParseLocationColorError),
        commodity: parse_string_array!(commodity, ParseLocationCommodityError),
    })
}

pub fn decode_date(date: DateSchema, mut dna_bytes: Vec<u8>) -> Result<Date, Error> {
    let era = decode_schema(date.era(), &mut dna_bytes)?;
    let year = decode_schema(date.year(), &mut dna_bytes)?;
    let time = decode_schema(date.time(), &mut dna_bytes)?;
    let weather = decode_schema(date.weather(), &mut dna_bytes)?;
    let holiday = decode_schema(date.holiday(), &mut dna_bytes)?;
    let season = decode_schema(date.season(), &mut dna_bytes)?;
    let background = decode_schema(date.background(), &mut dna_bytes)?;
    let effect = decode_schema(date.effect(), &mut dna_bytes)?;

    Ok(Date {
        era: parse_string!(era, ParseDateEraError),
        year: parse_number!(year, ParseDateYearError),
        time: parse_string!(time, ParseDateTimeError),
        weather: parse_string!(weather, ParseDateWeatherError),
        holiday: parse_string!(holiday, ParseDateHolidayError),
        season: parse_string!(season, ParseDateSeasonError),
        background: parse_number_array!(background, ParseDateBackgroundError),
        effect: parse_string_array!(effect, ParseDateEffectError),
    })
}

pub fn decode_story(story: StorySchema, mut dna_bytes: Vec<u8>) -> Result<Story, Error> {
    let character = decode_schema(story.character(), &mut dna_bytes)?;
    let location = decode_schema(story.location(), &mut dna_bytes)?;
    let date = decode_schema(story.date(), &mut dna_bytes)?;
    let event = decode_schema(story.event(), &mut dna_bytes)?;

    let handle = |value: &Value, error: Error| {
        let value = value.as_array().ok_or(error.clone())?;
        if let Some(value) = value.first() {
            Ok(Some(value.as_str().ok_or(error)?.to_string()))
        } else {
            Ok(None)
        }
    };

    Ok(Story {
        character: parse_string_array!(character, ParseStoryCharacterError, handle)
            .try_into()
            .map_err(|_| Error::ParseStoryCharacterError)?,
        location: parse_string_array!(location, ParseStoryLocationError, handle)
            .try_into()
            .map_err(|_| Error::ParseStoryLocationError)?,
        date: parse_string_array!(date, ParseStoryDateError, handle)
            .try_into()
            .map_err(|_| Error::ParseStoryDateError)?,
        event: parse_string_array!(event, ParseStoryEventError)
            .try_into()
            .map_err(|_| Error::ParseStoryEventError)?,
    })
}
