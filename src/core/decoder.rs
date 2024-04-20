use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use serde::{Deserialize, Serialize};
use spin::{lazy::Lazy, Mutex};

use crate::{error::Error, generated, handler::LanguagePackage};

static TEXT_POOL_RESOURCE: Lazy<Mutex<BTreeMap<String, String>>> = Lazy::new(|| Default::default());
static TEMPLATE_POOL_RESOURCE: Lazy<Mutex<BTreeMap<String, Vec<TemplateInstruction>>>> =
    Lazy::new(|| Default::default());

pub enum Language {
    FromPackage(LanguagePackage),
    #[allow(unused)]
    CN,
    #[allow(unused)]
    EN,
}

pub fn set_decoder_language(language: Language) -> Result<(), Error> {
    let (trait_pool, tempalte_pool, paragraph_pool) = match language {
        Language::FromPackage(package) => (
            package.trait_pool,
            package.template_pool,
            package.paragraph_pool,
        ),
        #[cfg(not(feature = "production"))]
        Language::CN => (
            generated::language::cn::TRAIT_POOL.to_string(),
            generated::language::cn::TEMPLATE_POOL.to_string(),
            generated::language::cn::PARAGRAPH_POOL.to_string(),
        ),
        #[cfg(not(feature = "production"))]
        Language::EN => unimplemented!(),
        #[cfg(feature = "production")]
        _ => panic!("language selection disabled in PRODUCTION mode"),
    };
    *TEXT_POOL_RESOURCE.lock() = {
        let mut trait_resource: BTreeMap<String, String> =
            serde_json::from_str(&trait_pool).map_err(|_| Error::ParseLanguageTraitPoolError)?;
        let paragraph_resource: BTreeMap<String, String> = serde_json::from_str(&paragraph_pool)
            .map_err(|_| Error::ParseLanguageParagraphPoolError)?;
        trait_resource.extend(paragraph_resource);
        trait_resource
    };
    *TEMPLATE_POOL_RESOURCE.lock() =
        serde_json::from_str(&tempalte_pool).map_err(|_| Error::ParseLanguageTemplatePoolError)?;
    Ok(())
}

pub fn decode_segment(content: &str) -> Result<Segment, Error> {
    Ok(serde_json::from_str(content).map_err(|_| Error::ParseSegmentError)?)
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
pub enum Selector {
    #[serde(alias = "single")]
    Single,
    #[serde(alias = "double")]
    Double,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum TemplateInstruction {
    #[serde(alias = "range")]
    Range(u8, u8),
    #[serde(alias = "pool")]
    Pool(Vec<u8>),
    #[serde(alias = "template")]
    Template(String),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum Pool {
    #[serde(alias = "trait_pool", deserialize_with = "trait_pool_adapter")]
    TraitPool(Vec<String>),
    #[serde(alias = "number_pool")]
    NumberPool(Vec<u16>),
    #[serde(alias = "number_range")]
    NumberRange((u16, u16)),
    #[serde(alias = "template_pool", deserialize_with = "template_pool_adapter")]
    TemplatePool(Vec<Vec<TemplateInstruction>>),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Pattern {
    pub occupied: u8,
    pub selector: Selector,
    pub pool: Pool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct VariablePatterns {
    pub number: Pattern,
    pub patterns: Vec<Pattern>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct Segment {
    pub bytes: u8,
    pub schema: Schema,
}

fn trait_pool_adapter<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw_trait_pool: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
    let map = TEXT_POOL_RESOURCE.lock();
    if !map.is_empty() {
        let trait_pool = raw_trait_pool
            .into_iter()
            .filter_map(|v| map.get(&v).cloned())
            .collect();
        Ok(trait_pool)
    } else {
        Ok(raw_trait_pool)
    }
}

fn template_pool_adapter<'de, D>(deserializer: D) -> Result<Vec<Vec<TemplateInstruction>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let raw_template_pool: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
    let map = TEMPLATE_POOL_RESOURCE.lock();
    if !map.is_empty() {
        let template_pool = raw_template_pool
            .into_iter()
            .filter_map(|v| map.get(&v).cloned())
            .collect();
        Ok(template_pool)
    } else {
        Ok(Vec::new())
    }
}
