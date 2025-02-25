use super::{DomainName, RecordType};

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

    pub fn as_bytes(&self) -> Vec<u8> {
        self.name
            .as_bytes()
            .into_iter()
            .chain(self.r#type.as_bytes())
            .chain(self.class.to_be_bytes())
            .collect()
    }
}
