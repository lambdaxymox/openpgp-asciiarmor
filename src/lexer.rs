use std::iter::Iterator;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::fmt;


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
    OtherUTF8,
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
    fn armor_string(self) -> Option<&'static str> {
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

fn string_to_token_type(token_string: &str) -> Option<TokenType> {
    match token_string {
        "=" => Some(TokenType::Pad),
        "/" => Some(TokenType::ForwardSlash),
        ":" => Some(TokenType::Colon),
        "," => Some(TokenType::Comma),
        "+" => Some(TokenType::PlusSign),
        " " => Some(TokenType::WhiteSpace),
        ": " => Some(TokenType::ColonSpace),
        "\n" => Some(TokenType::NewLine),
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
    fn new(token_type: TokenType, text: &str, location: Location) -> Token {
        Token {
            token_type: token_type,
            text: String::from(text),
            location: location,
        }
    }

    fn has_token_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

pub struct Lexer<S>
    where S: Iterator<Item = char>
{
    input: Peekable<S>,
    location: Location,
}

impl<S> Lexer<S>
    where S: Iterator<Item = char>
{
    pub fn new(input: S) -> Lexer<S> {
        let start = Location { absolute: 0 };

        Lexer {
            input: input.peekable(),
            location: start,
        }
    }

    pub fn next_token(&mut self) -> Token {
        match self.peek_char() {
            Some('-') => {
                let result = self.scan_five_dashes();
                if result.is_some() {
                    result.unwrap()
                } else {
                    self.scan_other_utf8().unwrap()
                }
            }
            Some('=') => self.scan_pad_symbol().unwrap(),
            Some('/') => self.scan_forwardslash().unwrap(),
            Some(':') => {
                let result = self.scan_colon_space();
                if result.is_some() {
                    result.unwrap()
                } else {
                    self.scan_colon().unwrap()
                }
            }
            Some('+') => self.scan_plus_sign().unwrap(),
            Some(',') => self.scan_comma().unwrap(),
            Some(' ') => self.scan_whitespace_symbol().unwrap(),
            Some('B') => {
                let result = self.scan_begin();
                if result.is_some() {
                    result.unwrap()
                } else {
                    self.scan_letter().unwrap()
                }
            }
            Some('E') => {
                let result = self.scan_end();
                if result.is_some() {
                    result.unwrap()
                } else {
                    self.scan_letter().unwrap()
                }
            }
            Some('V') => {
                let result = self.scan_version();
                if result.is_some() {
                    result.unwrap()
                } else {
                    self.scan_letter().unwrap()
                }
            }
            Some('C') => {
                let result = self.scan_comment();
                if result.is_some() {
                    return result.unwrap();
                }
                let result = self.scan_charset();
                if result.is_some() {
                    return result.unwrap();
                } else {
                    self.scan_letter().unwrap()
                }
            }
            Some('H') => {
                let result = self.scan_hash();
                if result.is_some() {
                    result.unwrap()
                } else {
                    self.scan_letter().unwrap()
                }
            }
            Some('P') => {
                let result = self.scan_pgp_message();
                if result.is_some() {
                    return result.unwrap();
                }
                let result = self.scan_pgp_public_key_block();
                if result.is_some() {
                    return result.unwrap();
                }
                let result = self.scan_pgp_private_key_block();
                if result.is_some() {
                    return result.unwrap();
                }
                let result = self.scan_pgp_message_part();
                if result.is_some() {
                    return result.unwrap();
                }
                let result = self.scan_pgp_message_part();
                if result.is_some() {
                    return result.unwrap();
                }
                let result = self.scan_pgp_signature();
                if result.is_some() {
                    return result.unwrap();
                }
                let result = self.scan_letter();
                if result.is_some() {
                    return result.unwrap();
                } else {
                    unreachable!();
                }
            }
            Some('M') => {
                let result = self.scan_messageid();
                if result.is_some() {
                    result.unwrap()
                } else {
                    self.scan_letter().unwrap()
                }
            }
            Some('\n') => {
                let result = self.scan_blankline();
                if result.is_some() {
                    result.unwrap()
                } else {
                    self.scan_newline().unwrap()
                }
            }
            Some('0'...'9') => self.scan_digit().unwrap(),
            Some('a'...'z') => self.scan_letter().unwrap(),
            Some('A'...'Z') => self.scan_letter().unwrap(),
            Some(_) => self.scan_other_utf8().unwrap(),
            None    => self.scan_eof().unwrap(),
        }
    }


    fn peek_char(&mut self) -> Option<char> {
        self.input.peek().map(|c| *c)
    }

    fn read_char(&mut self) -> Option<char> {
        match self.input.next() {
            Some(c) => {
                self.consume();
                Some(c)
            }
            None => None,
        }
    }

    fn consume_ntimes(&mut self, amount: usize) {
        self.location.increment(amount);
    }

    fn consume(&mut self) {
        self.location.increment(1);
    }

    fn backtrack_ntimes(&mut self, amount: usize) {
        self.location.decrement(amount);
    }

    fn backtrack(&mut self) {
        self.location.decrement(1);
    }

    fn match_terminal_symbol(&mut self, token_string: &str) -> Option<Token> {
        let mut result = String::new();
        let location = self.location;

        for ch in token_string.chars() {
            match self.peek_char() {
                Some(other_ch) => {
                    if other_ch == ch {
                        self.read_char();
                        result.push(other_ch);
                    } else {
                        self.backtrack_ntimes(result.len());
                        return None;
                    }
                }
                None => return None,
            }
        }

        let token_type = string_to_token_type(token_string).unwrap();

        Some(Token::new(token_type, token_string, location))
    }

    #[inline]
    fn scan_symbol(&mut self, token_type: TokenType) -> Option<Token> {
        self.match_terminal_symbol(token_type.armor_string().unwrap())
    }

    fn scan_five_dashes(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::FiveDashes)
    }

    fn scan_colon_space(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::ColonSpace)
    }

    fn scan_forwardslash(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::ForwardSlash)
    }

    fn scan_colon(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Colon)
    }

    fn scan_plus_sign(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PlusSign)
    }

    fn scan_pad_symbol(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Pad)
    }

    fn scan_whitespace_symbol(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::WhiteSpace)
    }

    fn scan_comma(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Comma)
    }

    fn scan_newline(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::NewLine)
    }

    fn scan_begin(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Begin)
    }

    fn scan_end(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::End)
    }

    fn scan_version(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Version)
    }

    fn scan_comment(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Comment)
    }

    fn scan_messageid(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::MessageID)
    }

    fn scan_hash(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Hash)
    }

    fn scan_charset(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Charset)
    }

    fn scan_pgp_message(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPMessage)
    }

    fn scan_pgp_public_key_block(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPPublicKeyBlock)
    }

    fn scan_pgp_private_key_block(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPPrivateKeyBlock)
    }

    fn scan_pgp_message_part(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPMessagePart)
    }

    fn scan_pgp_signature(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPSignature)
    }

    fn scan_other_utf8(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::OtherUTF8)
    }

    fn process_char(&mut self, ch: &str, token_type: TokenType) -> Option<Token> {
        let result = Some(Token::new(token_type, ch, self.location));
        self.read_char();

        result
    }

    fn scan_letter(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some('a') => self.process_char("a", TokenType::Letter),
            Some('b') => self.process_char("b", TokenType::Letter),
            Some('c') => self.process_char("c", TokenType::Letter),
            Some('d') => self.process_char("d", TokenType::Letter),
            Some('e') => self.process_char("e", TokenType::Letter),
            Some('f') => self.process_char("f", TokenType::Letter),
            Some('g') => self.process_char("g", TokenType::Letter),
            Some('h') => self.process_char("h", TokenType::Letter),
            Some('i') => self.process_char("i", TokenType::Letter),
            Some('j') => self.process_char("j", TokenType::Letter),
            Some('k') => self.process_char("k", TokenType::Letter),
            Some('l') => self.process_char("l", TokenType::Letter),
            Some('m') => self.process_char("m", TokenType::Letter),
            Some('n') => self.process_char("n", TokenType::Letter),
            Some('o') => self.process_char("o", TokenType::Letter),
            Some('p') => self.process_char("p", TokenType::Letter),
            Some('q') => self.process_char("q", TokenType::Letter),
            Some('r') => self.process_char("r", TokenType::Letter),
            Some('s') => self.process_char("s", TokenType::Letter),
            Some('t') => self.process_char("t", TokenType::Letter),
            Some('u') => self.process_char("u", TokenType::Letter),
            Some('v') => self.process_char("v", TokenType::Letter),
            Some('w') => self.process_char("w", TokenType::Letter),
            Some('x') => self.process_char("x", TokenType::Letter),
            Some('y') => self.process_char("y", TokenType::Letter),
            Some('z') => self.process_char("z", TokenType::Letter),
            Some('A') => self.process_char("A", TokenType::Letter),
            Some('B') => self.process_char("B", TokenType::Letter),
            Some('C') => self.process_char("C", TokenType::Letter),
            Some('D') => self.process_char("D", TokenType::Letter),
            Some('E') => self.process_char("E", TokenType::Letter),
            Some('F') => self.process_char("F", TokenType::Letter),
            Some('G') => self.process_char("G", TokenType::Letter),
            Some('H') => self.process_char("H", TokenType::Letter),
            Some('I') => self.process_char("I", TokenType::Letter),
            Some('J') => self.process_char("J", TokenType::Letter),
            Some('K') => self.process_char("K", TokenType::Letter),
            Some('L') => self.process_char("L", TokenType::Letter),
            Some('M') => self.process_char("M", TokenType::Letter),
            Some('N') => self.process_char("N", TokenType::Letter),
            Some('O') => self.process_char("O", TokenType::Letter),
            Some('P') => self.process_char("P", TokenType::Letter),
            Some('Q') => self.process_char("Q", TokenType::Letter),
            Some('R') => self.process_char("R", TokenType::Letter),
            Some('S') => self.process_char("S", TokenType::Letter),
            Some('T') => self.process_char("T", TokenType::Letter),
            Some('U') => self.process_char("U", TokenType::Letter),
            Some('V') => self.process_char("V", TokenType::Letter),
            Some('W') => self.process_char("W", TokenType::Letter),
            Some('X') => self.process_char("X", TokenType::Letter),
            Some('Y') => self.process_char("Y", TokenType::Letter),
            Some('Z') => self.process_char("Z", TokenType::Letter),
            _ => None,
        }
    }

    fn scan_digit(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some('0') => self.process_char("0", TokenType::Digit),
            Some('1') => self.process_char("1", TokenType::Digit),
            Some('2') => self.process_char("2", TokenType::Digit),
            Some('3') => self.process_char("3", TokenType::Digit),
            Some('4') => self.process_char("4", TokenType::Digit),
            Some('5') => self.process_char("5", TokenType::Digit),
            Some('6') => self.process_char("6", TokenType::Digit),
            Some('7') => self.process_char("7", TokenType::Digit),
            Some('8') => self.process_char("8", TokenType::Digit),
            Some('9') => self.process_char("9", TokenType::Digit),
            _ => None,
        }
    }

    fn scan_blankline(&mut self) -> Option<Token> {
        let mut result = String::new();
        let location = self.location;

        match self.peek_char() {
            Some('\n') => {
                result.push('\n');
                self.read_char();
            }
            _ => {
                return None;
            }
        }

        loop {
            match self.peek_char() {
                Some(' ') => {
                    result.push(' ');
                    self.read_char();
                }
                Some('\n') => {
                    result.push('\n');
                    self.read_char();
                    break;
                }
                _ => {
                    self.backtrack_ntimes(result.len());
                    return None;
                }
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(Token::new(TokenType::BlankLine, result.as_str(), location))
        }
    }

    fn scan_eof(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some(_) => None,
            None => {
                let token_string = TokenType::Eof.armor_string().unwrap();

                Some(Token::new(TokenType::Eof, token_string, Location::eof()))
            }
        }
    }
}

impl<S> Iterator for Lexer<S> where S: Iterator<Item = char> {
    type Item = Token;

    fn next(&mut self) -> Option<Token> {
        let next_token = self.next_token();
        if next_token.has_token_type(TokenType::Eof) {
            None
        } else {
            Some(next_token)
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Lexer;
    use std::io;
    use std::io::Write;


    fn ascii_armored_data() -> String {
        String::from("-----BEGIN PGP MESSAGE-----\n
                      Version: OpenPrivacy 0.99\n      \n
                      yDgBO22WxBHv7O8X7O/jygAEzol56iUKiXmV+XmpCtmpqQUKiQrFqclFqUDBovzS\n
                      vBSFjNSiVHsuAA==\n
                      =njUN\n
                      -----END PGP MESSAGE-----")
    }

    #[test]
    fn test_armor_lexer() {
        let armored_data = ascii_armored_data();
        let lexer = Lexer::new(armored_data.chars());

        for token in lexer {
            writeln!(&mut io::stderr(), "{:?}", token).unwrap();
        }
    }
}
