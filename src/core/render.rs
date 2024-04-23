use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::{format, vec, vec::Vec};
use core::cmp::max;

use crate::generated::language::cn;
use crate::{
    core::decoder::{
        Language, Pattern, Pool, Schema, Segment, Selector, TemplateInstruction, VariablePatterns,
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

pub enum RenderedPattern {
    Text(String),
    Number(u8),
}

impl RenderedPattern {
    pub fn text(self) -> Result<String, Error> {
        match self {
            Self::Text(text) => Ok(text),
            _ => Err(Error::ExtractPatternTextError),
        }
    }

    pub fn number(self) -> Result<u8, Error> {
        match self {
            Self::Number(value) => Ok(value),
            _ => Err(Error::ExtractPatternNumberError),
        }
    }
}

pub enum RenderedSegment {
    Simple(RenderedPattern),
    Array(Vec<RenderedPattern>),
    MultipleArray(Vec<Vec<RenderedPattern>>),
}

impl RenderedSegment {
    pub fn number(self) -> Result<u8, Error> {
        match self {
            Self::Simple(render) => render.number(),
            _ => Err(Error::ExtractSegmentNumberError),
        }
    }

    pub fn text(self) -> Result<String, Error> {
        match self {
            Self::Simple(render) => render.text(),
            _ => Err(Error::ExtractSegmentTextError),
        }
    }

    pub fn text_array(self) -> Result<Vec<String>, Error> {
        match self {
            Self::Array(render) => render
                .into_iter()
                .map(|v| v.text())
                .collect::<Result<Vec<_>, _>>(),
            _ => Err(Error::ExtractSegmentTextArrayError),
        }
    }

    pub fn number_array(self) -> Result<Vec<u8>, Error> {
        match self {
            Self::Array(render) => render
                .into_iter()
                .map(|v| v.number())
                .collect::<Result<Vec<_>, _>>(),
            _ => Err(Error::ExtractSegmentNumberArrayError),
        }
    }

    pub fn multiple_array(self) -> Result<Vec<Vec<RenderedPattern>>, Error> {
        match self {
            Self::MultipleArray(render) => Ok(render),
            _ => Err(Error::ExtractSegmentMultipleArrayError),
        }
    }
}

pub struct SegmentRender {
    text_resource_pool: BTreeMap<String, String>,
    template_resource_pool: BTreeMap<String, Vec<TemplateInstruction>>,
}

impl SegmentRender {
    pub fn new(language: Language) -> Result<Self, Error> {
        let (trait_pool, tempalte_pool, paragraph_pool) = match language {
            Language::CN => (
                cn::TRAIT_POOL.to_string(),
                cn::TEMPLATE_POOL.to_string(),
                cn::PARAGRAPH_POOL.to_string(),
            ),
            Language::EN => unimplemented!(),
        };
        let text_resource_pool = {
            let mut trait_resource: BTreeMap<String, String> = serde_json::from_str(&trait_pool)
                .map_err(|_| Error::ParseLanguageTraitPoolError)?;
            let paragraph_resource: BTreeMap<String, String> =
                serde_json::from_str(&paragraph_pool)
                    .map_err(|_| Error::ParseLanguageParagraphPoolError)?;
            trait_resource.extend(paragraph_resource);
            trait_resource
        };
        let template_resource_pool = serde_json::from_str(&tempalte_pool)
            .map_err(|_| Error::ParseLanguageTemplatePoolError)?;
        Ok(Self {
            text_resource_pool,
            template_resource_pool,
        })
    }

    fn translate_text(&self, key: &str) -> Result<String, Error> {
        self.text_resource_pool
            .get(key)
            .cloned()
            .ok_or(Error::RenderTextTranslationError)
    }

    fn translate_template(&self, key: &str) -> Result<Vec<TemplateInstruction>, Error> {
        self.template_resource_pool
            .get(key)
            .cloned()
            .ok_or(Error::RenderTemplateTranslationError)
    }

    fn render_pattern(
        &self,
        pattern: Pattern,
        bytes: &mut Vec<u8>,
    ) -> Result<RenderedPattern, Error> {
        let occupied = pattern.occupied as usize;
        let pattern_bytes = bytes.splice(0..occupied, vec![]).collect::<Vec<_>>();
        let render_result = match pattern.pool {
            Pool::TraitPool(value) => {
                let raw_trait =
                    PoolSelector::new(value, pattern_bytes)?.select_by(pattern.selector)?;
                RenderedPattern::Text(self.translate_text(&raw_trait)?)
            }
            Pool::NumberPool(value) => {
                let value = PoolSelector::new(value, pattern_bytes)?.select_by(pattern.selector)?;
                RenderedPattern::Number(value)
            }
            Pool::NumberRange(value) => {
                let value = PoolSelector::new_range(value.0..value.1, pattern_bytes)?
                    .select_by(pattern.selector)?;
                RenderedPattern::Number(value)
            }
            Pool::TemplatePool(value) => {
                let raw_templates =
                    PoolSelector::new(value, pattern_bytes)?.select_by(pattern.selector)?;
                let templates = self.translate_template(&raw_templates)?;
                RenderedPattern::Text(render_template_by_single(templates, bytes)?)
            }
        };
        Ok(render_result)
    }

    fn render_variable_patterns(
        &self,
        mut variable: VariablePatterns,
        segment_bytes: &mut Vec<u8>,
    ) -> Result<Vec<RenderedPattern>, Error> {
        let variable_occupied = variable.number.occupied as usize;
        let number_bytes = segment_bytes
            .splice(0..variable_occupied, vec![])
            .collect::<Vec<_>>();
        let number = match variable.number.pool {
            Pool::NumberPool(pool) => {
                PoolSelector::new(pool, number_bytes)?.select_by(variable.number.selector)?
            }
            Pool::NumberRange((v1, v2)) => PoolSelector::new_range(v1..=v2, number_bytes)?
                .select_by(variable.number.selector)?,
            _ => return Err(Error::RenderVariableNumberPoolError),
        };
        if number as usize > variable.patterns.len() {
            return Err(Error::RenderPatternCountError);
        }
        (0..number)
            .map(|_| {
                let pattern = variable.patterns.remove(0);
                self.render_pattern(pattern, segment_bytes)
            })
            .collect::<Result<Vec<_>, _>>()
    }

    pub fn render(
        &self,
        segment: &Segment,
        dna_bytes: &mut Vec<u8>,
    ) -> Result<RenderedSegment, Error> {
        let segment_occupied = segment.bytes as usize;
        let mut segment_bytes = dna_bytes
            .splice(0..segment_occupied, vec![])
            .collect::<Vec<_>>();
        match segment.schema.clone() {
            Schema::Simple(pattern) => {
                let render = self.render_pattern(pattern, &mut segment_bytes)?;
                Ok(RenderedSegment::Simple(render))
            }
            Schema::Fixed(patterns) => {
                let render = patterns
                    .into_iter()
                    .map(|pattern| self.render_pattern(pattern, &mut segment_bytes))
                    .collect::<Result<Vec<_>, Error>>()?;
                Ok(RenderedSegment::Array(render))
            }
            Schema::Variable(variable) => {
                let render = self.render_variable_patterns(variable, &mut segment_bytes)?;
                Ok(RenderedSegment::Array(render))
            }
            Schema::MultipleVariables(variables) => {
                let render = variables
                    .into_iter()
                    .map(|variable| self.render_variable_patterns(variable, &mut segment_bytes))
                    .collect::<Result<Vec<_>, _>>()?;
                Ok(RenderedSegment::MultipleArray(render))
            }
        }
    }
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
