use super::{DomainName, RecordType};
use crate::{utils, Result};
use std::io::Read;

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

    pub fn new<R: Read>(r: &mut R) -> Result<Self> {
        let name = DomainName::new(r)?;
        let bytes = utils::read_2_bytes(r)?;
        let r#type = RecordType::from_bytes(bytes);
        let _ = utils::read_2_bytes(r)?;

        let bytes = utils::read_4_bytes(r)?;
        let ttl = u32::from_be_bytes(bytes);

        let _length = utils::read_2_bytes(r)?;

        let bytes = utils::read_4_bytes(r)?;
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
