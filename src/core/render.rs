use alloc::string::{String, ToString};
use alloc::{format, vec, vec::Vec};
use core::cmp::max;

use serde::Serialize;
use serde_json::error::Category;

use crate::{
    core::decoder::{
        Pattern, Pool, Schema, Segment, Selector, TemplateInstruction, VariablePatterns,
    },
    error::Error,
};

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

struct PatternRender {
    pattern: Pattern,
}

impl PatternRender {
    fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }

    fn render(self, bytes: &mut Vec<u8>) -> Result<String, Error> {
        let occupied = self.pattern.occupied as usize;
        let pattern_bytes = bytes.splice(0..occupied, vec![]).collect::<Vec<_>>();
        let render_result = match self.pattern.pool {
            Pool::TraitPool(value) => {
                PoolSelector::new(value, pattern_bytes)?.select_by(self.pattern.selector)?
            }
            Pool::NumberPool(value) => {
                let value =
                    PoolSelector::new(value, pattern_bytes)?.select_by(self.pattern.selector)?;
                format!("{value}")
            }
            Pool::NumberRange(value) => {
                let value = PoolSelector::new_range(value.0..value.1, pattern_bytes)?
                    .select_by(self.pattern.selector)?;
                format!("{value}")
            }
            Pool::TemplatePool(value) => {
                let templates =
                    PoolSelector::new(value, pattern_bytes)?.select_by(self.pattern.selector)?;
                Self::render_template_by_single(Vec::new(), bytes)?
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
                    let value =
                        PoolSelector::new_range(v1..=v2, bytes.splice(0..1, vec![]).collect())?
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
}

pub trait Render: Serialize {
    fn render(&self, dna: Vec<u8>) -> Result<String, Error> {
        serde_json::to_string(self).map_err(|error| match error.classify() {
            Category::Data => error.to_string().into(),
            _ => Error::RenderObjectError,
        })
    }
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
            PatternRender::new(pattern).render(segment_bytes)
        })
        .collect::<Result<Vec<_>, _>>()
}

pub enum RenderedSegment {
    Simple(String),
    Fixed(Vec<String>),
    Variable(Vec<String>),
    MultipleVariables(Vec<Vec<String>>),
}

pub fn render_segment(
    segment: &Segment,
    dna_bytes: &mut Vec<u8>,
) -> Result<RenderedSegment, Error> {
    let segment_occupied = segment.bytes as usize;
    let mut segment_bytes = dna_bytes
        .splice(0..segment_occupied, vec![])
        .collect::<Vec<_>>();
    match segment.schema.clone() {
        Schema::Simple(pattern) => {
            let render = PatternRender::new(pattern).render(&mut segment_bytes)?;
            Ok(RenderedSegment::Simple(render))
        }
        Schema::Fixed(patterns) => {
            let render = patterns
                .into_iter()
                .map(|pattern| PatternRender::new(pattern).render(&mut segment_bytes))
                .collect::<Result<Vec<_>, Error>>()?;
            Ok(RenderedSegment::Fixed(render))
        }
        Schema::Variable(variable) => {
            let render = render_variable(variable, &mut segment_bytes)?;
            Ok(RenderedSegment::Variable(render))
        }
        Schema::MultipleVariables(variables) => {
            let render = variables
                .into_iter()
                .map(|variable| render_variable(variable, &mut segment_bytes))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(RenderedSegment::MultipleVariables(render))
        }
    }
}
