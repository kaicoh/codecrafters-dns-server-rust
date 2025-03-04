use crate::{utils, Error, Result};
use std::io::{Cursor, Read, Seek, SeekFrom};

mod answer;
mod header;
mod question;

pub use answer::Answer;
pub use header::Header;
pub use question::Question;

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub questions: Vec<Question>,
    pub answers: Vec<Answer>,
}

impl Message {
    fn new(header: Header) -> Self {
        Self {
            header,
            questions: vec![],
            answers: vec![],
        }
    }

    pub fn reply(msg: Self) -> Self {
        let Self {
            header, questions, ..
        } = msg;

        let mut msg = Self::new(Header::new_reply(header));

        for q in questions {
            msg = msg.set_question(q);
        }

        msg
    }

    pub fn query(id: u16, q: &Question) -> Self {
        let mut msg = Self::new(Header::new_query(id));
        msg = msg.set_question(q.clone());
        msg
    }

    pub fn error() -> Self {
        Self {
            header: Header::error(),
            questions: vec![],
            answers: vec![],
        }
    }

    pub fn set_question(self, q: Question) -> Self {
        let mut questions = self.questions;
        questions.push(q);
        let header = self.header.set_qs(questions.len() as u16);

        Self {
            header,
            questions,
            ..self
        }
    }

    pub fn set_answer(self, a: Answer) -> Self {
        let mut answers = self.answers;
        answers.push(a);
        let header = self.header.set_an(answers.len() as u16);

        Self {
            header,
            answers,
            ..self
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.header
            .as_bytes()
            .into_iter()
            .chain(self.questions.iter().flat_map(Question::as_bytes))
            .chain(self.answers.iter().flat_map(Answer::as_bytes))
            .collect()
    }

    pub fn id(&self) -> u16 {
        self.header.id().as_u16()
    }
}

impl TryFrom<&[u8]> for Message {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let mut cursor = Cursor::new(bytes);
        let header = Header::new(&mut cursor)?;

        let mut questions: Vec<Question> = vec![];
        for _ in 0..header.num_of_qs() {
            questions.push(Question::new(&mut cursor)?);
        }

        let mut answers: Vec<Answer> = vec![];
        for _ in 0..header.num_of_an() {
            answers.push(Answer::new(&mut cursor)?);
        }

        Ok(Self {
            header,
            questions,
            answers,
        })
    }
}

#[derive(Debug, Clone)]
struct DomainName(String);

impl DomainName {
    fn as_bytes(&self) -> Vec<u8> {
        self.0
            .split('.')
            .flat_map(label_part)
            .chain([0u8])
            .collect()
    }

    fn new(cursor: &mut Cursor<&[u8]>) -> Result<Self> {
        let mut byte = utils::read_1_byte(cursor)?;
        let mut tokens: Vec<Vec<u8>> = vec![];

        while byte != 0 {
            if byte & 0b11000000 == 0b11000000 {
                let b0 = byte & 0b00111111;
                let b1 = utils::read_1_byte(cursor)?;

                let p: u64 = u16::from_be_bytes([b0, b1]) as u64;
                let mut c = Cursor::new(*(cursor.get_ref()));
                c.seek(SeekFrom::Start(p))?;

                let mut pointed_tokens = Self::uncompressed_tokens(&mut c)?;
                tokens.append(&mut pointed_tokens);
                byte = 0;
            } else {
                let bytes = utils::read_n_bytes(cursor, byte as usize)?;
                tokens.push(bytes);
                byte = utils::read_1_byte(cursor)?;
            }
        }

        let val = tokens
            .into_iter()
            .filter_map(|bytes| String::from_utf8(bytes).ok())
            .collect::<Vec<String>>()
            .join(".");

        Ok(Self(val))
    }

    fn uncompressed_tokens<R: Read>(r: &mut R) -> Result<Vec<Vec<u8>>> {
        let mut len: usize = utils::read_1_byte(r)? as usize;
        let mut tokens: Vec<Vec<u8>> = vec![];

        while len > 0 {
            let bytes = utils::read_n_bytes(r, len)?;
            tokens.push(bytes);

            len = utils::read_1_byte(r)? as usize;
        }

        Ok(tokens)
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
    Ns,
    Md,
    Mf,
    Cname,
    Soa,
    Mb,
    Mg,
    Mr,
    Null,
    Wks,
    Ptr,
    Hinfo,
    Minfo,
    Mx,
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

    fn from_bytes(bytes: [u8; 2]) -> Self {
        match u16::from_be_bytes(bytes) {
            1 => Self::A,
            2 => Self::Ns,
            3 => Self::Md,
            4 => Self::Mf,
            5 => Self::Cname,
            6 => Self::Soa,
            7 => Self::Mb,
            8 => Self::Mg,
            9 => Self::Mr,
            10 => Self::Null,
            11 => Self::Wks,
            12 => Self::Ptr,
            13 => Self::Hinfo,
            14 => Self::Minfo,
            15 => Self::Mx,
            16 => Self::Txt,
            _ => panic!("Unexpected bytes: {bytes:?}"),
        }
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
