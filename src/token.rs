#![allow(dead_code)]
use std::fmt;


const LETTERS: [&'static str; 52] = [
        "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
        "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
        "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M",
        "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z"
    ];

const DIGITS: [&'static str; 10] = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Pad,
    ForwardSlash,
    Colon,
    Comma,
    Digit,
    Letter,
    PlusSign,
    WhiteSpace,
    OtherUtf8,
    ColonSpace,
    NewLine,
    FiveDashes,
    Begin,
    End,
    Version,
    Comment,
    MessageID,
    Hash,
    Charset,
    BlankLine,
    PGPMessage,
    PGPPublicKeyBlock,
    PGPPrivateKeyBlock,
    PGPMessagePart,
    PGPSignature,
    Eof,
}

impl TokenType {
    pub fn armor_string(self) -> Option<&'static str> {
        match self {
            TokenType::Pad => Some("="),
            TokenType::ForwardSlash => Some("/"),
            TokenType::Colon => Some(":"),
            TokenType::Comma => Some(","),
            TokenType::PlusSign => Some("+"),
            TokenType::WhiteSpace => Some(" "),
            TokenType::ColonSpace => Some(": "),
            TokenType::NewLine => Some("\n"),
            TokenType::FiveDashes => Some("-----"),
            TokenType::Begin => Some("BEGIN "),
            TokenType::End => Some("END "),
            TokenType::Version => Some("Version"),
            TokenType::Comment => Some("Comment"),
            TokenType::MessageID => Some("MessageID"),
            TokenType::Hash => Some("Hash"),
            TokenType::Charset => Some("Charset"),
            TokenType::Eof => Some("EOF"),
            TokenType::PGPMessage => Some("PGP MESSAGE"),
            TokenType::PGPPublicKeyBlock => Some("PGP PUBLIC KEY BLOCK"),
            TokenType::PGPPrivateKeyBlock => Some("PGP PRIVATE KEY BLOCK"),
            TokenType::PGPMessagePart => Some("PGP MESSAGE, PART "),
            TokenType::PGPSignature => Some("PGP SIGNATURE"),
            _ => None,
        }
    }
}

pub fn string_to_token_type(token_string: &str) -> Option<TokenType> {
    match token_string {
        "=" => Some(TokenType::Pad),
        "/" => Some(TokenType::ForwardSlash),
        ":" => Some(TokenType::Colon),
        "," => Some(TokenType::Comma),
        "+" => Some(TokenType::PlusSign),
        " " => Some(TokenType::WhiteSpace),
        ": " => Some(TokenType::ColonSpace),
        "\n" => Some(TokenType::NewLine),
        "\r" => Some(TokenType::NewLine),
        "-----" => Some(TokenType::FiveDashes),
        "BEGIN " => Some(TokenType::Begin),
        "END " => Some(TokenType::End),
        "Version" => Some(TokenType::Version),
        "Comment" => Some(TokenType::Comment),
        "MessageID" => Some(TokenType::MessageID),
        "Hash" => Some(TokenType::Hash),
        "Charset" => Some(TokenType::Charset),
        "EOF" => Some(TokenType::Eof),
        "PGP MESSAGE" => Some(TokenType::PGPMessage),
        "PGP PUBLIC KEY BLOCK" => Some(TokenType::PGPPublicKeyBlock),
        "PGP PRIVATE KEY BLOCK" => Some(TokenType::PGPPrivateKeyBlock),
        "PGP MESSAGE, PART " => Some(TokenType::PGPMessagePart),
        "PGP SIGNATURE" => Some(TokenType::PGPSignature),
        _ => None,
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Location {
    pub absolute: isize,
}

impl Location {
    pub fn eof() -> Location {
        Location { absolute: -1 }
    }

    #[inline]
    pub fn increment(&mut self, amount: usize) {
        self.absolute += amount as isize;
    }

    #[allow(dead_code)]
    #[inline]
    pub fn decrement(&mut self, amount: usize) {
        self.absolute -= amount as isize;
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Token {
    token_type: TokenType,
    text: String,
    location: Location,
}

impl Token {
    pub fn new(token_type: TokenType, text: &str, location: Location) -> Token {
        Token {
            token_type: token_type,
            text: String::from(text),
            location: location,
        }
    }

    pub fn from_char(token_type: TokenType, ch: char, location: Location) -> Token {
        let mut text = String::new();
        text.push(ch);

        Token {
            token_type: token_type,
            text: text,
            location: location
        }
    }

    pub fn has_token_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn as_str(&self) -> &str {
        self.text.as_str()
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.text.as_bytes()
    }

    fn is_pad(&self) -> bool {
        (self.token_type == TokenType::Pad) && (self.text == "=")
    }

    fn is_forwardslash(&self) -> bool {
        (self.token_type == TokenType::ForwardSlash) && (self.text == "/")
    }

    fn is_colon(&self) -> bool {
        (self.token_type == TokenType::Colon) && (self.text == ":")
    }

    fn is_comma(&self) -> bool {
        (self.token_type == TokenType::Comma) && (self.text == ",")
    }

    fn is_digit(&self) -> bool {
        (self.token_type == TokenType::Digit) && DIGITS.contains(&self.text.as_str())
    }

    fn is_letter(&self) -> bool {
        (self.token_type == TokenType::Letter) && LETTERS.contains(&self.text.as_str())
    }

    fn is_plussign(&self) -> bool {
        (self.token_type == TokenType::PlusSign) && (self.text == "+")
    }

    fn is_whitespace(&self) -> bool {
        (self.token_type == TokenType::WhiteSpace) && (self.text == " ")
    }

    fn is_other_utf8(&self) -> bool {
        (self.token_type == TokenType::OtherUtf8) && (self.text.len() == 1)
    }

    fn is_colonspace(&self) -> bool {
        (self.token_type == TokenType::ColonSpace) && (self.text == ": ")
    }

    fn is_newline(&self) -> bool {
        (self.token_type == TokenType::NewLine) && (self.text == "\n" || self.text == "\r")
    }

    fn is_fivedashes(&self) -> bool {
        (self.token_type == TokenType::FiveDashes) && (self.text == "-----")
    }

    fn is_begin(&self) -> bool {
        (self.token_type == TokenType::Begin) && (self.text == "BEGIN ")
    }

    fn is_end(&self) -> bool {
        (self.token_type == TokenType::End) && (self.text == "END ")
    }

    fn is_version(&self) -> bool {
        (self.token_type == TokenType::Version) && (self.text == "Version")
    }

    fn is_comment(&self) -> bool {
        (self.token_type == TokenType::Comment) && (self.text == "Comment")
    }

    fn is_messageid(&self) -> bool {
        (self.token_type == TokenType::MessageID) && (self.text == "MessageID")
    }

    fn is_hash(&self) -> bool {
        (self.token_type == TokenType::Hash) && (self.text == "Hash")
    }

    fn is_charset(&self) -> bool {
        (self.token_type == TokenType::Charset) && (self.text == "Charset")
    }

    fn is_blankline(&self) -> bool {
        let mut chars = self.text.chars();
        match chars.next() {
            Some('\n') | Some('\r') => {}
            _ => return false
        }

        loop {
            match chars.next() {
                Some(' ') => continue,
                Some('\n') | Some('\r') => break,
                _ => return false
            }
        }

        match chars.next() {
            Some(_) => return false,
            None    => {}
        }

        self.token_type == TokenType::BlankLine
    }

    fn is_pgp_message(&self) -> bool {
        (self.token_type == TokenType::PGPMessage) && (self.text == "PGP MESSAGE")
    }

    fn is_pgp_publickey_block(&self) -> bool {
        (self.token_type == TokenType::PGPPublicKeyBlock) && (self.text == "PGP PUBLIC KEY BLOCK")
    }

    fn is_pgp_privatekey_block(&self) -> bool {
        (self.token_type == TokenType::PGPPublicKeyBlock) && (self.text == "PGP PRIVATE KEY BLOCK")
    }

    fn is_pgp_message_part(&self) -> bool {
        (self.token_type == TokenType::PGPMessagePart) && (self.text == "PGP MESSAGE, PART ")
    }

    fn is_pgp_signature(&self) -> bool {
        (self.token_type == TokenType::PGPSignature) && (self.text == "PGP SIGNATURE")
    }

    fn is_eof(&self) -> bool {
        (self.token_type == TokenType::Eof) && (self.text == "EOF")
    }

    pub fn is_valid_token(&self) -> bool {
        match self.token_type {
            TokenType::Pad => self.is_pad(),
            TokenType::ForwardSlash => self.is_forwardslash(),
            TokenType::Colon => self.is_colon(),
            TokenType::Comma => self.is_comma(),
            TokenType::Digit => self.is_digit(),
            TokenType::Letter => self.is_letter(),
            TokenType::PlusSign => self.is_plussign(),
            TokenType::WhiteSpace => self.is_whitespace(),
            TokenType::OtherUtf8 => self.is_other_utf8(),
            TokenType::ColonSpace => self.is_colonspace(),
            TokenType::NewLine => self.is_newline(),
            TokenType::FiveDashes => self.is_fivedashes(),
            TokenType::Begin => self.is_begin(),
            TokenType::End => self.is_end(),
            TokenType::Version => self.is_version(),
            TokenType::Comment => self.is_comment(),
            TokenType::MessageID => self.is_messageid(),
            TokenType::Hash => self.is_hash(),
            TokenType::Charset => self.is_charset(),
            TokenType::BlankLine => self.is_blankline(),
            TokenType::PGPMessage => self.is_pgp_message(),
            TokenType::PGPPublicKeyBlock => self.is_pgp_publickey_block(),
            TokenType::PGPPrivateKeyBlock => self.is_pgp_privatekey_block(),
            TokenType::PGPMessagePart => self.is_pgp_message_part(),
            TokenType::PGPSignature => self.is_pgp_signature(),
            TokenType::Eof => self.is_eof()
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.token_type {
            TokenType::Pad => write!(f, "Pad(\"{}\")", self.text),
            TokenType::ForwardSlash => write!(f, "ForwardSlash(\"{}\")", self.text),
            TokenType::Colon => write!(f, "Colon(\"{}\")", self.text),
            TokenType::Comma => write!(f, "Comma(\"{}\")", self.text),
            TokenType::Digit => write!(f, "Digit(\"{}\")", self.text),
            TokenType::Letter => write!(f, "Letter(\"{}\")", self.text),
            TokenType::PlusSign => write!(f, "PlusSign(\"{}\")", self.text),
            TokenType::WhiteSpace => write!(f, "WhiteSpace(\"{}\")", self.text),
            TokenType::OtherUtf8 => write!(f, "OtherUtf8(\"{}\")", self.text),
            TokenType::ColonSpace => write!(f, "ColonSpace(\"{}\")", self.text),
            TokenType::NewLine => write!(f, "NewLine(\"{}\")", self.text),
            TokenType::FiveDashes => write!(f, "FiveDashes(\"{}\")", self.text),
            TokenType::Begin => write!(f, "Begin(\"{}\")", self.text),
            TokenType::End => write!(f, "End(\"{}\")", self.text),
            TokenType::Version => write!(f, "Version(\"{}\")", self.text),
            TokenType::Comment => write!(f, "Comment(\"{}\")", self.text),
            TokenType::MessageID => write!(f, "MessageID(\"{}\")", self.text),
            TokenType::Hash => write!(f, "Hash(\"{}\")", self.text),
            TokenType::Charset => write!(f, "Charset(\"{}\")", self.text),
            TokenType::BlankLine => write!(f, "BlankLine(\"{}\")", self.text),
            TokenType::PGPMessage => write!(f, "PGPMessage(\"{}\")", self.text),
            TokenType::PGPPublicKeyBlock => write!(f, "PGPPublicKeyBlock(\"{}\")", self.text),
            TokenType::PGPPrivateKeyBlock => write!(f, "PGPPrivateKeyBlock(\"{}\")", self.text),
            TokenType::PGPMessagePart => write!(f, "PGPMessagePart(\"{}\")", self.text),
            TokenType::PGPSignature => write!(f, "PGPSignature(\"{}\")", self.text),
            TokenType::Eof => write!(f, "EOF(\"{}\")", self.text)
        }
    }
}
