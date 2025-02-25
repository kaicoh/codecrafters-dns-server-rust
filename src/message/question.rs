#[derive(Debug)]
pub struct Question {
    name: DomainName,
    r#type: RecordType,
    class: u16,
}

impl Question {
    pub fn test() -> Self {
        Self {
            name: DomainName("codecrafters.io".into()),
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

#[derive(Debug)]
pub struct DomainName(String);

impl DomainName {
    fn as_bytes(&self) -> Vec<u8> {
        self.0
            .split('.')
            .flat_map(label_part)
            .chain([0u8])
            .collect()
    }
}

fn label_part(substr: &str) -> Vec<u8> {
    let length = substr.len() as u8;
    [length]
        .into_iter()
        .chain(substr.as_bytes().to_vec())
        .collect()
}

#[derive(Debug, Clone, Copy)]
pub enum RecordType {
    A,
    #[allow(unused)]
    Ns,
    #[allow(unused)]
    Md,
    #[allow(unused)]
    Mf,
    #[allow(unused)]
    Cname,
    #[allow(unused)]
    Soa,
    #[allow(unused)]
    Mb,
    #[allow(unused)]
    Mg,
    #[allow(unused)]
    Mr,
    #[allow(unused)]
    Null,
    #[allow(unused)]
    Wks,
    #[allow(unused)]
    Ptr,
    #[allow(unused)]
    Hinfo,
    #[allow(unused)]
    Minfo,
    #[allow(unused)]
    Mx,
    #[allow(unused)]
    Txt,
}

impl RecordType {
    fn as_u16(&self) -> u16 {
        match self {
            Self::A => 1,
            Self::Ns => 2,
            Self::Md => 3,
            Self::Mf => 4,
            Self::Cname => 5,
            Self::Soa => 6,
            Self::Mb => 7,
            Self::Mg => 8,
            Self::Mr => 9,
            Self::Null => 10,
            Self::Wks => 11,
            Self::Ptr => 12,
            Self::Hinfo => 13,
            Self::Minfo => 14,
            Self::Mx => 15,
            Self::Txt => 16,
        }
    }

    fn as_bytes(&self) -> [u8; 2] {
        self.as_u16().to_be_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_serializes_domain_name() {
        let name = DomainName("google.com".into());
        let expected = b"\x06google\x03com\x00".to_vec();
        assert_eq!(name.as_bytes(), expected);
    }
}
