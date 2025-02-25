mod header;
mod question;

use header::Header;
use question::Question;

#[derive(Debug)]
pub struct Message {
    header: Header,
    questions: Vec<Question>,
}

impl Message {
    pub fn test() -> Self {
        let msg = Self {
            header: Header::test(),
            questions: vec![],
        };
        msg.set_question(Question::test())
    }

    pub fn set_question(self, q: Question) -> Self {
        let mut questions = self.questions;
        questions.push(q);
        let header = self.header.set_qs(questions.len() as u16);

        Self { header, questions }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.header
            .as_bytes()
            .into_iter()
            .chain(self.questions.iter().flat_map(Question::as_bytes))
            .collect()
    }
}
