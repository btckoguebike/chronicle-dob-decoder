use alloc::string::String;
use alloc::vec::Vec;

use serde::Deserialize;

use crate::error::Error;

pub enum Language {
    #[allow(unused)]
    CN,
    #[allow(unused)]
    EN,
}

impl TryFrom<String> for Language {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "cn" => Ok(Self::CN),
            "en" => Ok(Self::EN),
            _ => Err(Error::InvalidLanguageInArgs),
        }
    }
}

pub fn decode_segment(content: &str) -> Result<Segment, Error> {
    Ok(serde_json::from_str(content).map_err(|_| Error::ParseSegmentError)?)
}

#[derive(Deserialize, Clone)]
pub enum Selector {
    #[serde(alias = "single")]
    Single,
    #[serde(alias = "double")]
    Double,
}

#[derive(Deserialize, Clone)]
pub enum TemplateInstruction {
    #[serde(alias = "range")]
    Range(u8, u8),
    #[serde(alias = "pool")]
    Pool(Vec<u8>),
    #[serde(alias = "template")]
    Template(String),
}

#[derive(Deserialize, Clone)]
pub enum Pool {
    #[serde(alias = "trait_pool")]
    TraitPool(Vec<String>),
    #[serde(alias = "number_pool")]
    NumberPool(Vec<u8>),
    #[serde(alias = "number_range")]
    NumberRange((u8, u8)),
    #[serde(alias = "template_pool")]
    TemplatePool(Vec<String>), // after rendering, it will be `Vec<Vec<TemplateInstruction>>`
}

#[derive(Deserialize, Clone)]
pub struct Pattern {
    pub occupied: u8,
    pub selector: Selector,
    pub pool: Pool,
}

#[derive(Deserialize, Clone)]
pub struct VariablePatterns {
    pub number: Pattern,
    pub patterns: Vec<Pattern>,
}

#[derive(Deserialize, Clone)]
pub enum Schema {
    #[serde(alias = "simple")]
    Simple(Pattern),
    #[serde(alias = "fixed")]
    Fixed(Vec<Pattern>),
    #[serde(alias = "variable")]
    Variable(VariablePatterns),
    #[serde(alias = "multiple_variables")]
    MultipleVariables(Vec<VariablePatterns>),
}

#[derive(Deserialize)]
pub struct Segment {
    pub bytes: u8,
    pub schema: Schema,
}
