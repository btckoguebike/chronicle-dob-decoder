#![no_std]

mod chronicle;
pub use chronicle::*;

mod casting {
    extern crate alloc;
    use super::*;
    use alloc::{string::String, vec::Vec};
    use molecule::prelude::{Builder, Entity};

    impl From<String> for UTF8Bytes {
        fn from(value: String) -> Self {
            UTF8Bytes::new_builder()
                .set(value.as_bytes().into_iter().map(|v| (*v).into()).collect())
                .build()
        }
    }

    impl From<Vec<String>> for TraitPool {
        fn from(value: Vec<String>) -> Self {
            TraitPool::new_builder()
                .set(value.into_iter().map(Into::into).collect())
                .build()
        }
    }

    impl From<Vec<u8>> for NumberPool {
        fn from(value: Vec<u8>) -> Self {
            NumberPool::new_builder()
                .set(value.into_iter().map(|v| v.into()).collect())
                .build()
        }
    }

    impl From<(u8, u8)> for NumberRange {
        fn from(value: (u8, u8)) -> Self {
            NumberRange::new_builder()
                .min(value.0.into())
                .max(value.1.into())
                .build()
        }
    }

    impl TryFrom<Vec<serde_json::Value>> for TemplatePool {
        type Error = String;

        fn try_from(value: Vec<serde_json::Value>) -> Result<Self, Self::Error> {
            let instruction_set = value
                .into_iter()
                .map(|v| match v {
                    serde_json::Value::Array(array) => {
                        let instructions = array
                            .into_iter()
                            .map(|v| {
                                let mut instruction = Instruction::new_builder();
                                if let Some(range) = v.get("range") {
                                    let range: (u8, u8) = serde_json::from_value(range.clone())
                                        .map_err(|_| "invalid number range")?;
                                    instruction = instruction
                                        .set::<InstructionUnion>(NumberRange::from(range).into());
                                } else if let Some(pool) = v.get("pool") {
                                    let pool: Vec<String> = serde_json::from_value(pool.clone())
                                        .map_err(|_| "invalid number pool")?;
                                    instruction = instruction
                                        .set::<InstructionUnion>(TraitPool::from(pool).into());
                                } else if let Some(template) = v.get("template") {
                                    let template: String = serde_json::from_value(template.clone())
                                        .map_err(|_| "invalid template")?;
                                    instruction = instruction
                                        .set::<InstructionUnion>(UTF8Bytes::from(template).into());
                                } else {
                                    return Err(String::from("invalid instruction"));
                                }
                                Ok(instruction.build())
                            })
                            .collect::<Result<Vec<_>, _>>()?;
                        Ok(InstructionVec::new_builder().set(instructions).build())
                    }
                    _ => Err(String::from("invalid template pool")),
                })
                .collect::<Result<_, _>>()?;
            Ok(TemplatePool::new_builder().set(instruction_set).build())
        }
    }
}
