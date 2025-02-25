use super::{question::Question, DomainName, RecordType};
use crate::{utils, Result};
use std::io::Cursor;

#[derive(Debug)]
pub struct Answer {
    name: DomainName,
    r#type: RecordType,
    class: u16,
    ttl: u32,
    data: Rdata,
}

impl Answer {
    pub fn test() -> Self {
        Self {
            name: DomainName::test(),
            r#type: RecordType::A,
            class: 1,
            ttl: 60,
            data: Rdata::A([8, 8, 8, 8]),
        }
    }

    pub fn new(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let name = DomainName::new(cursor)?;
        let bytes = utils::read_2_bytes(cursor)?;
        let r#type = RecordType::from_bytes(bytes);
        let _ = utils::read_2_bytes(cursor)?;

        let bytes = utils::read_4_bytes(cursor)?;
        let ttl = u32::from_be_bytes(bytes);

        let _length = utils::read_2_bytes(cursor)?;

        let bytes = utils::read_4_bytes(cursor)?;
        let data = Rdata::A(bytes);

        Ok(Self {
            name,
            r#type,
            class: 1,
            ttl,
            data,
        })
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.name
            .as_bytes()
            .into_iter()
            .chain(self.r#type.as_bytes())
            .chain(self.class.to_be_bytes())
            .chain(self.ttl.to_be_bytes())
            .chain(self.data.len().to_be_bytes())
            .chain(self.data.as_bytes())
            .collect()
    }
}

#[derive(Debug)]
enum Rdata {
    A([u8; 4]),
}

impl Rdata {
    fn len(&self) -> u16 {
        match self {
            Self::A(_) => 4,
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Self::A(bytes) => bytes.to_vec(),
        }
    }
}

impl From<&Question> for Answer {
    fn from(q: &Question) -> Self {
        Self {
            name: q.name().clone(),
            r#type: q.r#type(),
            class: q.class(),
            ttl: 60,
            data: Rdata::A([8, 8, 8, 8]),
        }
    }
}
