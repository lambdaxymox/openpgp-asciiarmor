//use std::str::Chars;
//use std::iter::Iterator;
use std::collections::VecDeque;
use std::iter::Peekable;
use std::fmt;


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TokenType {
    Pad,
    ForwardSlash,
    NonPaddedBase64,
    Base64,
    Colon,
    Comma,
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
    Number,
    Eof
}

impl TokenType {
    fn armor_string(self) -> Option<&'static str> {
        match self {
            TokenType::Pad          => Some("="),
            TokenType::ForwardSlash => Some("/"),
            TokenType::Colon        => Some(":"),
            TokenType::Comma        => Some(","),
            TokenType::WhiteSpace   => Some(" "),
            TokenType::ColonSpace   => Some(": "),
            TokenType::NewLine      => Some("\n"),
            TokenType::FiveDashes   => Some("-----"),
            TokenType::Begin        => Some("BEGIN "),
            TokenType::End          => Some("END "),
            TokenType::Version      => Some("Version"),
            TokenType::Comment      => Some("Comment"),
            TokenType::MessageID    => Some("MessageID"),
            TokenType::Hash         => Some("Hash"),
            TokenType::Charset      => Some("Charset"),
            TokenType::Eof          => Some("EOF"),
            TokenType::PGPSymbol    => Some("PGP "),
            TokenType::PGPMessage   => Some("MESSAGE"),
            TokenType::PGPPublicKeyBlock  => Some("PUBLIC KEY BLOCK"),
            TokenType::PGPPrivateKeyBlock => Some("PRIVATE KEY BLOCK"),
            TokenType::PGPMessagePart     => Some("PGP MESSAGE, PART "),
            TokenType::PGPSignature       => Some("SIGNATURE"),
            _ => None
        }
    }
}

fn string_to_token_type(token_string: &str) -> Option<TokenType> {
    match token_string {
        "="  => Some(TokenType::Pad),
        "/"  => Some(TokenType::ForwardSlash),
        ":"  => Some(TokenType::Colon),
        ","  => Some(TokenType::Comma),
        " "  => Some(TokenType::WhiteSpace),
        ": " => Some(TokenType::ColonSpace),
        "\n" => Some(TokenType::NewLine),
        "-----"     => Some(TokenType::FiveDashes),
        "BEGIN "    => Some(TokenType::Begin),
        "END "      => Some(TokenType::End),
        "Version"   => Some(TokenType::Version),
        "Comment"   => Some(TokenType::Comment),
        "MessageID" => Some(TokenType::MessageID),
        "Hash"      => Some(TokenType::Hash),
        "Charset"   => Some(TokenType::Charset),
        "EOF"       => Some(TokenType::Eof),
        "PGP "      => Some(TokenType::PGPSymbol),
        "MESSAGE"   => Some(TokenType::PGPMessage),
        "PUBLIC KEY BLOCK"   => Some(TokenType::PGPPublicKeyBlock),
        "PRIVATE KEY BLOCK"  => Some(TokenType::PGPPrivateKeyBlock),
        "PGP MESSAGE, PART " => Some(TokenType::PGPMessagePart),
        "SIGNATURE"          => Some(TokenType::PGPSignature),
        _ => None
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Location {
    pub absolute: isize
}

impl Location {
    pub fn eof() -> Location {
        Location { 
            absolute: -1 
        }
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
    location: Location
}

impl Token {
    fn new(token_type: TokenType, text: &str, location: Location) -> Token {
        Token {
            token_type: token_type,
            text: String::from(text),
            location: location
        }
    }

    fn has_token_type(&self, token_type: TokenType) -> bool {
        self.token_type == token_type
    }
}

pub struct Lexer<S> where S: Iterator<Item=char> {
    input: Peekable<S>,
    location: Location,
    tokens: VecDeque<Token>,
    unprocessed_tokens: Vec<Token>,
    offset: usize
}

impl<S> Lexer<S> where S: Iterator<Item=char> {
    pub fn new(input: S) -> Lexer<S> {
        let start = Location { absolute: 0 };

        Lexer {
            input: input.peekable(),
            location: start,
            tokens: VecDeque::with_capacity(20),
            unprocessed_tokens: Vec::new(),
            offset: 0
        }
    }

    /*
    // TODO: Implement next_token.
    pub fn next_token(&mut self) -> Token {
     
    }
    */

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
        let location   = self.location;

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
                },
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

    fn scan_eof(&mut self) -> Option<Token> {
        match self.peek_char() {
            Some(char) => None,
            None => {
                let token_string = TokenType::Eof.armor_string().unwrap();

                Some(Token::new(TokenType::Eof, token_string, self.location))
            }
        }
    }
}


/*
#[cfg(test)]
mod tests {
    use super::ArmorLexer;
    use std::io;
    use std::io::Write;


    fn ascii_armored_data() -> String {
        String::from(
            "-----BEGIN PGP MESSAGE-----\n\
            Version: OpenPrivacy 0.99\n\
            \n\
            yDgBO22WxBHv7O8X7O/jygAEzol56iUKiXmV+XmpCtmpqQUKiQrFqclFqUDBovzS\n\
            vBSFjNSiVHsuAA==\n\
            =njUN\n\
            -----END PGP MESSAGE-----")
    }

    #[test]
    fn test_armor_lexer() {
        let armored_data = ascii_armored_data();
        let armor_lexer = ArmorLexer::new(&armored_data);

        for token in armor_lexer {
            assert!(token.valid_token());
            writeln!(&mut io::stderr(), "{:?}", token).unwrap();
        }
    }
}
*/