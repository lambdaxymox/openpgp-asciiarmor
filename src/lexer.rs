// use std::str::Chars;
// use std::iter::Iterator;
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
    PGPSymbol,
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
            TokenType::PGPSymbol => Some("PGP "),
            TokenType::PGPMessage => Some("MESSAGE"),
            TokenType::PGPPublicKeyBlock => Some("PUBLIC KEY BLOCK"),
            TokenType::PGPPrivateKeyBlock => Some("PRIVATE KEY BLOCK"),
            TokenType::PGPMessagePart => Some("PGP MESSAGE, PART "),
            TokenType::PGPSignature => Some("SIGNATURE"),
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
        "PGP " => Some(TokenType::PGPSymbol),
        "MESSAGE" => Some(TokenType::PGPMessage),
        "PUBLIC KEY BLOCK" => Some(TokenType::PGPPublicKeyBlock),
        "PRIVATE KEY BLOCK" => Some(TokenType::PGPPrivateKeyBlock),
        "PGP MESSAGE, PART " => Some(TokenType::PGPMessagePart),
        "SIGNATURE" => Some(TokenType::PGPSignature),
        _ => None,
    }
}

#[derive(Copy, Clone, Debug)]
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

#[derive(Clone, Debug)]
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
    tokens: VecDeque<Token>,
    unprocessed_tokens: Vec<Token>,
    offset: usize,
}

impl<S> Lexer<S>
    where S: Iterator<Item = char>
{
    pub fn new(input: S) -> Lexer<S> {
        let start = Location { absolute: 0 };

        Lexer {
            input: input.peekable(),
            location: start,
            tokens: VecDeque::with_capacity(20),
            unprocessed_tokens: Vec::new(),
            offset: 0,
        }
    }

    // TODO: Implement next_token.
    // pub fn next_token(&mut self) -> Token {
    //
    // }
    //

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

    fn consumeN(&mut self, amount: usize) {
        self.location.increment(amount);
    }

    fn consume(&mut self) {
        self.location.increment(1);
    }

    fn backtrackN(&mut self, amount: usize) {
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
                        self.backtrackN(result.len());
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

    fn scan_one_pad_symbol(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::Pad)
    }

    fn scan_one_whitespace_symbol(&mut self) -> Option<Token> {
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

    fn scan_pgp_symbol(&mut self) -> Option<Token> {
        self.scan_symbol(TokenType::PGPSymbol)
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

    fn process_char(&mut self, ch: &str) -> Option<Token> {
        let result = Some(Token::new(TokenType::Digit, ch, self.location));
        self.consume();

        result
    }

    fn scan_letter(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some('a') => self.process_char("a"),
            Some('b') => self.process_char("b"),
            Some('c') => self.process_char("c"),
            Some('d') => self.process_char("d"),
            Some('e') => self.process_char("e"),
            Some('f') => self.process_char("f"),
            Some('g') => self.process_char("g"),
            Some('h') => self.process_char("h"),
            Some('i') => self.process_char("i"),
            Some('j') => self.process_char("j"),
            Some('k') => self.process_char("k"),
            Some('l') => self.process_char("l"),
            Some('m') => self.process_char("m"),
            Some('n') => self.process_char("n"),
            Some('o') => self.process_char("o"),
            Some('p') => self.process_char("p"),
            Some('q') => self.process_char("q"),
            Some('r') => self.process_char("r"),
            Some('s') => self.process_char("s"),
            Some('t') => self.process_char("t"),
            Some('u') => self.process_char("u"),
            Some('v') => self.process_char("v"),
            Some('w') => self.process_char("w"),
            Some('x') => self.process_char("x"),
            Some('y') => self.process_char("y"),
            Some('z') => self.process_char("z"),
            Some('A') => self.process_char("A"),
            Some('B') => self.process_char("B"),
            Some('C') => self.process_char("C"),
            Some('D') => self.process_char("D"),
            Some('E') => self.process_char("E"),
            Some('F') => self.process_char("F"),
            Some('G') => self.process_char("G"),
            Some('H') => self.process_char("H"),
            Some('I') => self.process_char("I"),
            Some('J') => self.process_char("J"),
            Some('K') => self.process_char("K"),
            Some('L') => self.process_char("L"),
            Some('M') => self.process_char("M"),
            Some('N') => self.process_char("N"),
            Some('O') => self.process_char("O"),
            Some('P') => self.process_char("P"),
            Some('Q') => self.process_char("Q"),
            Some('R') => self.process_char("R"),
            Some('S') => self.process_char("S"),
            Some('T') => self.process_char("T"),
            Some('U') => self.process_char("U"),
            Some('V') => self.process_char("V"),
            Some('W') => self.process_char("W"),
            Some('X') => self.process_char("X"),
            Some('Y') => self.process_char("Y"),
            Some('Z') => self.process_char("Z"),
            _ => None,
        }
    }

    fn scan_digit(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some('0') => self.process_char("0"),
            Some('1') => self.process_char("1"),
            Some('2') => self.process_char("2"),
            Some('3') => self.process_char("3"),
            Some('4') => self.process_char("4"),
            Some('5') => self.process_char("5"),
            Some('6') => self.process_char("6"),
            Some('7') => self.process_char("7"),
            Some('8') => self.process_char("8"),
            Some('9') => self.process_char("9"),
            _ => None,
        }
    }

    fn scan_eof(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some(_) => None,
            None => {
                let token_string = TokenType::Eof.armor_string().unwrap();

                Some(Token::new(TokenType::Eof, token_string, self.location))
            }
        }
    }
}


// #[cfg(test)]
// mod tests {
// use super::ArmorLexer;
// use std::io;
// use std::io::Write;
//
//
// fn ascii_armored_data() -> String {
// String::from(
// "-----BEGIN PGP MESSAGE-----\n\
// Version: OpenPrivacy 0.99\n\
// \n\
// yDgBO22WxBHv7O8X7O/jygAEzol56iUKiXmV+XmpCtmpqQUKiQrFqclFqUDBovzS\n\
// vBSFjNSiVHsuAA==\n\
// =njUN\n\
// -----END PGP MESSAGE-----")
// }
//
// #[test]
// fn test_armor_lexer() {
// let armored_data = ascii_armored_data();
// let armor_lexer = ArmorLexer::new(&armored_data);
//
// for token in armor_lexer {
// assert!(token.valid_token());
// writeln!(&mut io::stderr(), "{:?}", token).unwrap();
// }
// }
// }
//
