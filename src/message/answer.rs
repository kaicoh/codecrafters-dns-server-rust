use super::{DomainName, RecordType};

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
