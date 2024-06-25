use chronicle_schema as gen;
use molecule::prelude::*;
use serde_json::Value;
use std::{collections::HashMap, fs, process::Command};

#[derive(serde::Deserialize)]
struct Segment {
    bytes: u8,
    trait_pool: Option<Vec<String>>,
    number_pool: Option<Vec<u8>>,
    number_range: Option<(u8, u8)>,
    template_pool: Option<Vec<String>>,
}

#[derive(serde::Deserialize)]
struct VariableSegment {
    count: Segment,
    segments: Vec<Segment>,
}

#[derive(serde::Deserialize)]
struct Pattern {
    segment: Option<Segment>,
    variable_segment: Option<VariableSegment>,
    variable_segment_vec: Option<Vec<VariableSegment>>,
    #[serde(flatten)]
    additional_fields: HashMap<String, Value>,
}

fn parse_segment(value: &Segment, additional_fields: &HashMap<String, Value>) -> gen::Segment {
    let mut builder = gen::Segment::new_builder().segment_bytes(value.bytes.into());
    if let Some(trait_pool) = &value.trait_pool {
        builder = builder.pool(gen::TraitPool::from(trait_pool.clone()).into());
    } else if let Some(number_pool) = &value.number_pool {
        builder = builder.pool(gen::NumberPool::from(number_pool.clone()).into());
    } else if let Some(range) = &value.number_range {
        builder = builder.pool(gen::NumberRange::from(*range).into());
    } else if let Some(template_pool) = &value.template_pool {
        let roots = template_pool
            .iter()
            .map(|s| {
                additional_fields
                    .get(s)
                    .cloned()
                    .ok_or(format!("not found {s}"))
            })
            .collect::<Result<Vec<_>, _>>()
            .expect("get template roots");
        builder = builder.pool(
            gen::TemplatePool::try_from(roots)
                .expect("template pool")
                .into(),
        );
    } else {
        panic!("invalid segment");
    }
    builder.build()
}

fn parse_variable_segment(
    value: &VariableSegment,
    additional_fields: &HashMap<String, Value>,
) -> gen::VariableSegment {
    gen::VariableSegment::new_builder()
        .count(parse_segment(&value.count, additional_fields))
        .segments(
            value
                .segments
                .iter()
                .map(|segment| parse_segment(segment, additional_fields))
                .collect(),
        )
        .build()
}

fn parse_variable_segment_vec(
    value: &[VariableSegment],
    additional_fields: &HashMap<String, Value>,
) -> gen::VariableSegmentVec {
    gen::VariableSegmentVec::new_builder()
        .set(
            value
                .iter()
                .map(|variable_segment| parse_variable_segment(variable_segment, additional_fields))
                .collect::<Vec<_>>(),
        )
        .build()
}

fn parse_pattern(value: &Pattern) -> gen::Pattern {
    if let Some(segment) = &value.segment {
        gen::Pattern::new_builder()
            .set(parse_segment(segment, &value.additional_fields))
            .build()
    } else if let Some(variable_segment) = &value.variable_segment {
        gen::Pattern::new_builder()
            .set(parse_variable_segment(
                variable_segment,
                &value.additional_fields,
            ))
            .build()
    } else if let Some(variable_segment_vec) = &value.variable_segment_vec {
        gen::Pattern::new_builder()
            .set(parse_variable_segment_vec(
                variable_segment_vec,
                &value.additional_fields,
            ))
            .build()
    } else {
        panic!("invalid schema");
    }
}

macro_rules! parse_schema_object {
    ($target:ident, $object:ident, ($($field:ident $(,)?)+)) => {{
        let object = [
            $(stringify!($field),)+
        ]
        .into_iter()
        .map(|field| {
            let pattern_json = fs::read_to_string(format!("./assets/{}/{field}.json", stringify!($object)))
                .expect(&format!("read {field}.json"));
            let pattern: Pattern = serde_json::from_str(&pattern_json).expect(&format!("parse {field}.json"));
            (field, pattern)
        })
        .collect::<HashMap<_, _>>();
        gen::$target::new_builder()
            $(
                .$field(parse_pattern(&object[stringify!($field)]))
            )+
            .build()
    }};
}

pub fn main() {
    // generate `character` schema
    let player = parse_schema_object!(
        PlayerSchema,
        player,
        (adjective, name, profession, power, gold, card)
    );

    // gnerate `date` module
    let context = parse_schema_object!(
        ContextSchema,
        context,
        (adjective, era, time, mode, rank, effect)
    );

    // gnerate `location` module
    let scene = parse_schema_object!(
        SceneSchema,
        scene,
        (name, attribute, operation, difficulty, score, commodity)
    );

    // gnerate `story` module
    let event = parse_schema_object!(
        EventSchema,
        event,
        (player, scene, context, transition, climax, ending)
    );

    // generate `generated.rs`
    let mol_chronicle = gen::AshWarChronicle::new_builder()
        .player(player)
        .context(context)
        .scene(scene)
        .event(event)
        .build();
    let mol_bytes = mol_chronicle.as_slice();
    let file_content = format!(
        "#[allow(dead_code)]\npub static MOL_CHRONICLE_SCHEMA: [u8; {}] = {mol_bytes:?};",
        mol_bytes.len(),
    );
    fs::write("./src/generated.rs", file_content).expect("write generated.rs");
    Command::new("rustfmt")
        .arg("./src/generated.rs")
        .status()
        .expect("rustfmt generated.rs");

    println!("cargo:rerun-if-changed=build.rs");
}
