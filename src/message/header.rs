// Ref: https://en.wikipedia.org/wiki/Domain_Name_System#DNS_message_format
#[derive(Debug)]
pub struct Header {
    id: TransactionId,
    qr: Qr,
    opcode: OpCode,
    aa: AuthAnswer,
    tc: Truncation,
    rd: RecursionDesired,
    ra: RecursionAvailable,
    ad: AuthenticData,
    cd: CheckingDisable,
    rcode: Rcode,
    num_of_qs: u16,
    num_of_an: u16,
    num_of_authorities: u16,
    num_of_additionals: u16,
}

impl Header {
    pub fn test() -> Self {
        Self {
            id: TransactionId(1234),
            qr: Qr::Reply,
            opcode: OpCode::Query,
            aa: AuthAnswer(false),
            tc: Truncation(false),
            rd: RecursionDesired(false),
            ra: RecursionAvailable(false),
            ad: AuthenticData(false),
            cd: CheckingDisable(false),
            rcode: Rcode::NoErr,
            num_of_qs: 0,
            num_of_an: 0,
            num_of_authorities: 0,
            num_of_additionals: 0,
        }
    }

    pub fn set_qs(self, qs: u16) -> Self {
        Self {
            num_of_qs: qs,
            ..self
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TransactionId(u16);

impl TransactionId {
    fn as_bytes(&self) -> [u8; 2] {
        self.0.to_be_bytes()
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Qr {
    #[allow(unused)]
    Query,
    Reply,
}

impl Qr {
    fn as_byte(&self) -> u8 {
        match self {
            Self::Query => 0b00000000,
            Self::Reply => 0b10000000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OpCode {
    Query,
    #[allow(unused)]
    Iquery,
    #[allow(unused)]
    Status,
}

impl OpCode {
    fn as_byte(&self) -> u8 {
        match self {
            Self::Query => 0b00000000,
            Self::Iquery => 0b00001000,
            Self::Status => 0b00010000,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Rcode {
    NoErr,
    #[allow(unused)]
    FormatErr,
    #[allow(unused)]
    ServerErr,
    #[allow(unused)]
    NonexistentDomain,
}

impl Rcode {
    fn as_byte(&self) -> u8 {
        match self {
            Self::NoErr => 0b00000000,
            Self::FormatErr => 0b00000001,
            Self::ServerErr => 0b00000010,
            Self::NonexistentDomain => 0b00000011,
        }
    }
}

// Authoritative Answer (AA)
// 1 if the responding server "owns" the domain queried, i.e., it's authoritative.
#[derive(Debug, Clone, Copy)]
pub struct AuthAnswer(bool);

impl AuthAnswer {
    fn as_byte(&self) -> u8 {
        if self.0 {
            0b00000100
        } else {
            0b00000000
        }
    }
}

// Truncation (TC)
// 1 if the message is larger than 512 bytes. Always 0 in UDP responses.
#[derive(Debug, Clone, Copy)]
pub struct Truncation(bool);

impl Truncation {
    fn as_byte(&self) -> u8 {
        if self.0 {
            0b00000010
        } else {
            0b00000000
        }
    }
}

// Recursion Desired (RD)
// Sender sets this to 1 if the server should recursively resolve this query, 0 otherwise.
#[derive(Debug, Clone, Copy)]
pub struct RecursionDesired(bool);

impl RecursionDesired {
    fn as_byte(&self) -> u8 {
        if self.0 {
            0b00000001
        } else {
            0b00000000
        }
    }
}

// Recursion Available (RA)
// Server sets this to 1 to indicate that recursion is available.
#[derive(Debug, Clone, Copy)]
pub struct RecursionAvailable(bool);

impl RecursionAvailable {
    fn as_byte(&self) -> u8 {
        if self.0 {
            0b10000000
        } else {
            0b00000000
        }
    }
}

// Authentic Data (AD)
// in a response, indicates if the replying DNS server verified the data.
#[derive(Debug, Clone, Copy)]
pub struct AuthenticData(bool);

impl AuthenticData {
    fn as_byte(&self) -> u8 {
        if self.0 {
            0b00100000
        } else {
            0b00000000
        }
    }
}

// Checking Disabled (CD)
// in a query, indicates that non-verified data is acceptable in a response.
#[derive(Debug, Clone, Copy)]
pub struct CheckingDisable(bool);

impl CheckingDisable {
    fn as_byte(&self) -> u8 {
        if self.0 {
            0b00010000
        } else {
            0b00000000
        }
    }
}

fn flag_byte_1st_half(
    qr: &Qr,
    opcode: &OpCode,
    aa: &AuthAnswer,
    tc: &Truncation,
    rd: &RecursionDesired,
) -> u8 {
    qr.as_byte() | opcode.as_byte() | aa.as_byte() | tc.as_byte() | rd.as_byte()
}

fn flag_byte_2nd_half(
    ra: &RecursionAvailable,
    ad: &AuthenticData,
    cd: &CheckingDisable,
    rcode: &Rcode,
) -> u8 {
    ra.as_byte() | ad.as_byte() | cd.as_byte() | rcode.as_byte()
}

impl Header {
    pub fn as_bytes(&self) -> [u8; 12] {
        let Self {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            ad,
            cd,
            rcode,
            num_of_qs,
            num_of_an,
            num_of_authorities,
            num_of_additionals,
        } = self;

        let [b0, b1] = id.as_bytes();
        let b2 = flag_byte_1st_half(qr, opcode, aa, tc, rd);
        let b3 = flag_byte_2nd_half(ra, ad, cd, rcode);
        let [b4, b5] = num_of_qs.to_be_bytes();
        let [b6, b7] = num_of_an.to_be_bytes();
        let [b8, b9] = num_of_authorities.to_be_bytes();
        let [b10, b11] = num_of_additionals.to_be_bytes();

        [b0, b1, b2, b3, b4, b5, b6, b7, b8, b9, b10, b11]
    }
}
