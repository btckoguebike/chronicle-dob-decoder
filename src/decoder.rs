use alloc::{
    format,
    string::{String, ToString},
    vec,
    vec::{IntoIter, Vec},
};
use chronicle_schema::{
    ContextSchema, EventSchema, Instruction, InstructionUnion, Pattern, PatternUnion, PlayerSchema,
    PoolUnion, SceneSchema, Segment, VariableSegment,
};
use serde_json::Value;

use crate::{
    error::Error,
    object::{Context, Event, Player, Scene},
};

struct PoolSelector<T: Clone> {
    array: Vec<T>,
    bytes: Vec<u8>,
}

macro_rules! check_splice {
    ($vec:ident, $occupied:expr) => {{
        if $vec.len() < $occupied {
            return Err(Error::DecodeInsufficientDNA);
        }
        $vec.splice(0..$occupied, vec![]).collect::<Vec<_>>()
    }};
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
    let occupied_bytes = check_splice!(dna_bytes, occupied);
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
            let value = value.into_iter().map(u8::from).collect();
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
            InstructionUnion::NumberRange(value) => {
                let min: u8 = value.min().into();
                let max: u8 = value.max().into();
                let value =
                    PoolSelector::new_range(min..=max, check_splice!(dna_bytes, 1))?.select()?;
                pending_values.push(value.to_string());
            }
            InstructionUnion::TraitPool(value) => {
                let value = value
                    .into_iter()
                    .map(|v| String::from_utf8_lossy(&v.raw_data()).to_string())
                    .collect();
                let value = PoolSelector::new(value, check_splice!(dna_bytes, 1))?.select()?;
                pending_values.push(value);
            }
            InstructionUnion::UTF8Bytes(template) => {
                let template = String::from_utf8_lossy(&template.raw_data()).to_string();
                match pending_values.len() {
                    0 => results.push(template),
                    1 => {
                        let parts = template.split('x').collect::<Vec<_>>();
                        assert!(parts.len() == 2);
                        results.push(format!(
                            "{}{}{}",
                            parts[0],
                            pending_values.remove(0),
                            parts[1]
                        ))
                    }
                    2 => {
                        let parts = template.split('x').collect::<Vec<_>>();
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
    let occupied_bytes = check_splice!(dna_bytes, occupied);
    let count = match variable.count().pool().to_enum() {
        PoolUnion::NumberPool(value) => {
            let value = value.into_iter().map(u8::from).collect();
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
    variable
        .segments()
        .into_iter()
        .take(count as usize)
        .map(|segment| decode_segment(segment, dna_bytes))
        .collect::<Result<Vec<_>, _>>()
}

fn decode_pattern(pattern: Pattern, dna_bytes: &mut Vec<u8>) -> Result<Value, Error> {
    match pattern.to_enum() {
        PatternUnion::Segment(segment) => {
            let value = decode_segment(segment, dna_bytes)?;
            Ok(value)
        }
        PatternUnion::VariableSegment(variable) => {
            let value = decode_variable_segment(variable, dna_bytes)?
                .into_iter()
                .collect::<Vec<_>>();
            Ok(Value::Array(value))
        }
        PatternUnion::VariableSegmentVec(variables) => {
            let value = variables
                .into_iter()
                .map(|variable| {
                    decode_variable_segment(variable, dna_bytes)
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

pub fn decode_player(player: PlayerSchema, mut dna_bytes: Vec<u8>) -> Result<Player, Error> {
    let adjective = decode_pattern(player.adjective(), &mut dna_bytes)?;
    let name = decode_pattern(player.name(), &mut dna_bytes)?;
    let profession = decode_pattern(player.profession(), &mut dna_bytes)?;
    let power = decode_pattern(player.power(), &mut dna_bytes)?;
    let gold = decode_pattern(player.gold(), &mut dna_bytes)?;
    let card = decode_pattern(player.card(), &mut dna_bytes)?;

    Ok(Player {
        adjective: parse_string!(adjective, ParsePlayerAdjectiveError),
        name: parse_string!(name, ParsePlayerNameError),
        profession: parse_string!(profession, ParsePlayerProfessionError),
        power: parse_number!(power, ParsePlayerPowerError),
        gold: parse_number!(gold, ParsePlayerGoldError),
        card: parse_string_array!(card, ParsePlayerCardError),
    })
}

pub fn decode_scene(scene: SceneSchema, mut dna_bytes: Vec<u8>) -> Result<Scene, Error> {
    let name = decode_pattern(scene.name(), &mut dna_bytes)?;
    let attribute = decode_pattern(scene.attribute(), &mut dna_bytes)?;
    let operation = decode_pattern(scene.operation(), &mut dna_bytes)?;
    let score = decode_pattern(scene.score(), &mut dna_bytes)?;
    let difficulty = decode_pattern(scene.difficulty(), &mut dna_bytes)?;
    let commodity = decode_pattern(scene.commodity(), &mut dna_bytes)?;

    Ok(Scene {
        name: parse_string!(name, ParseSceneNameError),
        attribute: parse_string!(attribute, ParseSceneAttributeError),
        operation: parse_string!(operation, ParseSceneOperationError),
        score: parse_number!(score, ParseSceneScoreError),
        difficulty: parse_number!(difficulty, ParseSceneDifficultyError),
        commodity: parse_string_array!(commodity, ParseSceneCommodityError),
    })
}

pub fn decode_context(context: ContextSchema, mut dna_bytes: Vec<u8>) -> Result<Context, Error> {
    let adjective = decode_pattern(context.adjective(), &mut dna_bytes)?;
    let era = decode_pattern(context.era(), &mut dna_bytes)?;
    let time = decode_pattern(context.time(), &mut dna_bytes)?;
    let mode = decode_pattern(context.mode(), &mut dna_bytes)?;
    let rank = decode_pattern(context.rank(), &mut dna_bytes)?;
    let effect = decode_pattern(context.effect(), &mut dna_bytes)?;

    Ok(Context {
        adjective: parse_string!(adjective, ParseEnvironmentAjectiveError),
        era: parse_string!(era, ParseEnvironmentEraError),
        time: parse_string!(time, ParseEnvironmentTimeError),
        mode: parse_string!(mode, ParseEnvironmentModeError),
        rank: parse_number!(rank, ParseEnvironmentRankError),
        effect: parse_string_array!(effect, ParseEnvironmentEffectError),
    })
}

pub fn decode_event(event: EventSchema, mut dna_bytes: Vec<u8>) -> Result<Event, Error> {
    let player = decode_pattern(event.player(), &mut dna_bytes)?;
    let scene = decode_pattern(event.scene(), &mut dna_bytes)?;
    let context = decode_pattern(event.context(), &mut dna_bytes)?;
    let transition = decode_pattern(event.transition(), &mut dna_bytes)?;
    let climax = decode_pattern(event.climax(), &mut dna_bytes)?;
    let ending = decode_pattern(event.ending(), &mut dna_bytes)?;

    let handle = |value: &Value, error: Error| {
        let value = value.as_array().ok_or(error.clone())?;
        match value.first() {
            Some(Value::String(value)) => Ok(Some(value.clone())),
            Some(Value::Number(value)) => Ok(Some(value.to_string())),
            None => Ok(None),
            _ => Err(error),
        }
    };

    Ok(Event {
        player: parse_string_array!(player, ParseEventPlayerError, handle)
            .try_into()
            .map_err(|_| Error::ParseEventPlayerError)?,
        scene: parse_string_array!(scene, ParseEventSceneError, handle)
            .try_into()
            .map_err(|_| Error::ParseEventSceneError)?,
        context: parse_string_array!(context, ParseEventContextError, handle)
            .try_into()
            .map_err(|_| Error::ParseEventContextError)?,
        transition: parse_string!(transition, ParseEventTransitionError),
        climax: parse_string!(climax, ParseEventClimaxError),
        ending: parse_string!(ending, ParseEventEndingError),
    })
}
