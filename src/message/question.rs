use super::{DomainName, RecordType};
use crate::{utils, Result};
use std::io::Cursor;

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

    pub fn new(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let name = DomainName::new(cursor)?;
        let bytes = utils::read_2_bytes(cursor)?;
        let r#type = RecordType::from_bytes(bytes);
        let _ = utils::read_2_bytes(cursor)?;

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

    pub(super) fn name(&self) -> &DomainName {
        &self.name
    }

    pub(super) fn r#type(&self) -> RecordType {
        self.r#type
    }

    pub(super) fn class(&self) -> u16 {
        self.class
    }
}
