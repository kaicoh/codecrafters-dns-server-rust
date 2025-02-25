mod answer;
mod header;
mod question;

use answer::Answer;
use header::Header;
use question::Question;

#[derive(Debug)]
pub struct Message {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Answer>,
}

impl Message {
    pub fn test() -> Self {
        let msg = Self {
            header: Header::test(),
            questions: vec![],
            answers: vec![],
        };

        msg.set_question(Question::test())
            .set_answer(Answer::test())
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
}

#[derive(Debug)]
struct DomainName(String);

impl DomainName {
    fn as_bytes(&self) -> Vec<u8> {
        self.0
            .split('.')
            .flat_map(label_part)
            .chain([0u8])
            .collect()
    }

    fn test() -> Self {
        Self("codecrafters.io".into())
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
