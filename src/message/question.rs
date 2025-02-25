use super::{DomainName, RecordType};
use crate::{utils, Result};
use std::io::Read;

#[derive(Debug)]
pub struct Question {
    name: DomainName,
    r#type: RecordType,
    class: u16,
}

impl Question {
    pub fn test() -> Self {
        Self {
            name: DomainName::test(),
            r#type: RecordType::A,
            class: 1,
        }
    }

    pub fn new<R: Read>(r: &mut R) -> Result<Self> {
        let name = DomainName::new(r)?;
        let bytes = utils::read_2_bytes(r)?;
        let r#type = RecordType::from_bytes(bytes);
        let _ = utils::read_2_bytes(r)?;

        Ok(Self {
            name,
            r#type,
            class: 1,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.name
            .as_bytes()
            .into_iter()
            .chain(self.r#type.as_bytes())
            .chain(self.class.to_be_bytes())
            .collect()
    }
}
