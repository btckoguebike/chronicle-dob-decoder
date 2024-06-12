use alloc::string::{String, ToString};
use alloc::{format, vec, vec::Vec};
use core::cmp::max;
use serde_json::error::Category;

use serde::ser::{self, SerializeSeq};
use serde::Serialize;
use spin::{lazy::Lazy, Mutex};

use crate::{
    core::decoder::{Pattern, Pool, Schema, Segment, Selector, TemplateInstruction},
    error::Error,
};

use super::decoder::VariablePatterns;

static RENDER_DNA: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| Mutex::new(vec![]));

fn set_render_dna(dna: Vec<u8>) {
    *RENDER_DNA.lock() = dna;
}

struct PoolSelector<T: Clone> {
    array: Vec<T>,
    bytes: Vec<u8>,
}

impl<T: Clone> PoolSelector<T> {
    fn new(array: Vec<T>, bytes: Vec<u8>) -> Result<Self, Error> {
        if array.is_empty() {
            return Err(Error::RenderPoolEmptyError);
        }
        Ok(Self { array, bytes })
    }

    fn new_range(range: impl Iterator<Item = T>, bytes: Vec<u8>) -> Result<Self, Error> {
        let array = range.collect();
        Self::new(array, bytes)
    }

    fn select_by(self, selector: Selector) -> Result<T, Error> {
        match selector {
            Selector::Single => self.select_by_single(),
            Selector::Double => self.select_by_double(),
        }
    }

    fn select_by_single(self) -> Result<T, Error> {
        let indicator = match self.bytes.len() {
            1 => self.bytes[0] as usize,
            2 => u16::from_le_bytes(self.bytes.try_into().unwrap()) as usize,
            4 => u32::from_le_bytes(self.bytes.try_into().unwrap()) as usize,
            8 => u64::from_le_bytes(self.bytes.try_into().unwrap()) as usize,
            _ => return Err(Error::RenderAssembleSingleSelectorError),
        };
        let offset = indicator % self.array.len();
        Ok(self.array[offset].clone())
    }

    fn select_by_double(self) -> Result<T, Error> {
        let (first_indicator, second_indicator) = match self.bytes.len() {
            2 => (self.bytes[0] as usize, self.bytes[1] as usize),
            4 => (
                u16::from_le_bytes(self.bytes[0..2].try_into().unwrap()) as usize,
                u16::from_le_bytes(self.bytes[2..4].try_into().unwrap()) as usize,
            ),
            8 => (
                u32::from_le_bytes(self.bytes[0..4].try_into().unwrap()) as usize,
                u32::from_le_bytes(self.bytes[4..8].try_into().unwrap()) as usize,
            ),
            _ => return Err(Error::RenderAssembleDoubleSelectorError),
        };
        let first_offset = first_indicator % self.array.len();
        let second_offset = second_indicator % max(first_offset, 1);
        Ok(self.array[second_offset].clone())
    }
}

pub trait Render: Serialize {
    fn render(&self, dna: Vec<u8>) -> Result<String, Error> {
        set_render_dna(dna);
        serde_json::to_string(self).map_err(|error| match error.classify() {
            Category::Data => error.to_string().into(),
            _ => Error::RenderObjectError,
        })
    }
}

fn render_pattern(pattern: Pattern, bytes: &mut Vec<u8>) -> Result<String, Error> {
    let occupied = pattern.occupied as usize;
    let pattern_bytes = bytes.splice(0..occupied, vec![]).collect::<Vec<_>>();
    let render_result = match pattern.pool {
        Pool::TraitPool(value) => {
            PoolSelector::new(value, pattern_bytes)?.select_by(pattern.selector)?
        }
        Pool::NumberPool(value) => {
            let value = PoolSelector::new(value, pattern_bytes)?.select_by(pattern.selector)?;
            format!("{value}")
        }
        Pool::NumberRange(value) => {
            let value = PoolSelector::new_range(value.0..value.1, pattern_bytes)?
                .select_by(pattern.selector)?;
            format!("{value}")
        }
        Pool::TemplatePool(value) => {
            if value.is_empty() {
                String::new()
            } else {
                let templates =
                    PoolSelector::new(value, pattern_bytes)?.select_by(pattern.selector)?;
                render_template_by_single(templates, bytes)?
            }
        }
    };
    Ok(render_result)
}

fn render_template_by_single(
    templates: Vec<TemplateInstruction>,
    bytes: &mut Vec<u8>,
) -> Result<String, Error> {
    let mut render_results = vec![];
    let mut pending_values = vec![];
    for template in templates {
        match template {
            TemplateInstruction::Pool(array) => {
                let value = PoolSelector::new(array, bytes.splice(0..1, vec![]).collect())?
                    .select_by_single()?;
                pending_values.push(value);
            }
            TemplateInstruction::Range(v1, v2) => {
                let value = PoolSelector::new_range(v1..=v2, bytes.splice(0..1, vec![]).collect())?
                    .select_by_single()?;
                pending_values.push(value);
            }
            TemplateInstruction::Template(template) => match pending_values.len() {
                0 => render_results.push(template),
                1 => {
                    let parts = template.split("x").collect::<Vec<_>>();
                    assert!(parts.len() == 2);
                    render_results.push(format!(
                        "{}{}{}",
                        parts[0],
                        pending_values.remove(0),
                        parts[1]
                    ))
                }
                2 => {
                    let parts = template.split("x").collect::<Vec<_>>();
                    assert!(parts.len() == 3);
                    render_results.push(format!(
                        "{}{}{}{}{}",
                        parts[0],
                        pending_values.remove(0),
                        parts[1],
                        pending_values.remove(0),
                        parts[2]
                    ));
                }
                _ => return Err(Error::RenderTemplateElementsCountError),
            },
        }
    }
    Ok(render_results.join(" => "))
}

fn render_variable(
    mut variable: VariablePatterns,
    segment_bytes: &mut Vec<u8>,
) -> Result<Vec<String>, Error> {
    let variable_occupied = variable.number.occupied as usize;
    let number_bytes = segment_bytes
        .splice(0..variable_occupied, vec![])
        .collect::<Vec<_>>();
    let number = match variable.number.pool {
        Pool::NumberPool(pool) => {
            PoolSelector::new(pool, number_bytes)?.select_by(variable.number.selector)?
        }
        Pool::NumberRange((v1, v2)) => {
            PoolSelector::new_range(v1..=v2, number_bytes)?.select_by(variable.number.selector)?
        }
        _ => return Err(Error::RenderVariableNumberPoolError),
    };
    if number as usize > variable.patterns.len() {
        return Err(Error::RenderPatternCountError);
    }
    (0..number)
        .map(|_| {
            let pattern = variable.patterns.remove(0);
            render_pattern(pattern, segment_bytes)
        })
        .collect::<Result<Vec<_>, _>>()
}

pub fn segment_render<S>(segment: &Segment, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut dna_bytes = RENDER_DNA.lock();
    let segment_occupied = segment.bytes as usize;
    let mut segment_bytes = dna_bytes
        .splice(0..segment_occupied, vec![])
        .collect::<Vec<_>>();
    match segment.schema.clone() {
        Schema::Simple(pattern) => {
            let render = render_pattern(pattern, &mut segment_bytes).map_err(ser::Error::custom)?;
            serializer.serialize_str(&render)
        }
        Schema::Fixed(patterns) => {
            let mut seq = serializer.serialize_seq(Some(patterns.len()))?;
            patterns.into_iter().try_for_each(|pattern| {
                let render =
                    render_pattern(pattern, &mut segment_bytes).map_err(ser::Error::custom)?;
                seq.serialize_element(&render)
            })?;
            seq.end()
        }
        Schema::Variable(variable) => {
            let render_results =
                render_variable(variable, &mut segment_bytes).map_err(ser::Error::custom)?;
            let mut seq = serializer.serialize_seq(Some(render_results.len()))?;
            for value in render_results {
                seq.serialize_element(&value)?;
            }
            seq.end()
        }
        Schema::MultipleVariables(variables) => {
            let mut seq = serializer.serialize_seq(Some(variables.len()))?;
            for variable in variables.into_iter() {
                let render_results =
                    render_variable(variable, &mut segment_bytes).map_err(ser::Error::custom)?;
                seq.serialize_element(&render_results)?;
            }
            seq.end()
        }
    }
}
