use std::str::Chars;
use std::iter::Iterator;


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
    OtherUtf8,
    UpperCaseLetter,
    LowerCaseLetter,
    Digit,
    EqualSign,
    Colon,
    WhiteSpace,
    NewLine,
    Comma,
    ForwardSlash,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ArmorToken {
    OtherUtf8(char),
    UpperCaseLetter(char),
    LowerCaseLetter(char),
    Digit(char),
    EqualSign(char),
    Colon(char),
    WhiteSpace(char),
    NewLine(char),
    Comma(char),
    ForwardSlash(char),
}

impl ArmorToken {

    pub fn valid_token(self) -> bool {
        match self {
            ArmorToken::UpperCaseLetter(_)  => self.is_upper_case(),
            ArmorToken::LowerCaseLetter(_)  => self.is_lower_case(),
            ArmorToken::Digit(token)        => self.is_digit(),
            ArmorToken::EqualSign(token)    => token == '=',
            ArmorToken::Colon(token)        => token == ':',
            ArmorToken::WhiteSpace(token)   => token == ' ',
            ArmorToken::NewLine(token)      => (token == '\n') || (token == '\r'),
            ArmorToken::Comma(token)        => token == ',',
            ArmorToken::ForwardSlash(token) => token == '/',
            ArmorToken::OtherUtf8(_)        => self.is_other_utf8(),
        }
    }

    pub fn token(self) -> char {
        match self {
            ArmorToken::UpperCaseLetter(token) | 
            ArmorToken::LowerCaseLetter(token) |
            ArmorToken::Digit(token)           |
            ArmorToken::EqualSign(token)       |
            ArmorToken::Colon(token)           |
            ArmorToken::WhiteSpace(token)      |
            ArmorToken::NewLine(token)         |
            ArmorToken::Comma(token)           |
            ArmorToken::ForwardSlash(token)    |
            ArmorToken::OtherUtf8(token)       => token,
        }
    }

    pub fn token_type(self) -> TokenType {
        match self {
            ArmorToken::UpperCaseLetter(_) => TokenType::UpperCaseLetter,
            ArmorToken::LowerCaseLetter(_) => TokenType::LowerCaseLetter,
            ArmorToken::Digit(_)           => TokenType::Digit,
            ArmorToken::EqualSign(_)       => TokenType::EqualSign,
            ArmorToken::Colon(_)           => TokenType::Colon,
            ArmorToken::WhiteSpace(_)      => TokenType::WhiteSpace,
            ArmorToken::NewLine(_)         => TokenType::NewLine,
            ArmorToken::Comma(_)           => TokenType::Comma,
            ArmorToken::ForwardSlash(_)    => TokenType::ForwardSlash,
            ArmorToken::OtherUtf8(_)       => TokenType::OtherUtf8,
        }        
    }

    #[inline]
    pub fn is_letter(self) -> bool {
        self.is_upper_case() || self.is_lower_case()
    }

    pub fn is_upper_case(self) -> bool {
        match self {
            ArmorToken::UpperCaseLetter(token) => token.is_uppercase(),
            _ => false,
        }
    }

    pub fn is_lower_case(self) -> bool {
        match self {
            ArmorToken::LowerCaseLetter(token) => token.is_lowercase(),
            _ => false,
        }
    }

    pub fn is_digit(self) -> bool {
        match self {
            ArmorToken::Digit(token) => token.is_digit(10),
            _ => false,
        }
    }

    pub fn is_equal_sign(self) -> bool {
        match self {
            ArmorToken::EqualSign(token) => token == '=',
            _ => false,
        }
    }

    pub fn is_colon(self) -> bool {
        match self {
            ArmorToken::Colon(token) => token == ':',
            _ => false,
        }
    }

    pub fn is_whitespace(self) -> bool {
        match self {
            ArmorToken::WhiteSpace(token) => token == ' ',
            _ => false,
        }
    }

    pub fn is_newline(self) -> bool {
        match self {
            ArmorToken::NewLine(token) => (token == '\n') || (token == '\r'),
            _ => false,
        }
    }
    
    pub fn is_comma(self) -> bool {
        match self {
            ArmorToken::Comma(token) => token == ',',
            _ => false,
        }
    }

    pub fn is_forward_slash(self) -> bool {
        match self {
            ArmorToken::ForwardSlash(token) => token == '/',
            _ => false,
        }
    }

    #[inline]
    pub fn is_other_utf8(self) -> bool {
        match self {
            ArmorToken::OtherUtf8(token) => {
                is_utf8(token) && !(self.is_upper_case() || self.is_lower_case() 
                                                         || self.is_digit() 
                                                         || self.is_equal_sign()
                                                         || self.is_colon() 
                                                         || self.is_whitespace() 
                                                         || self.is_newline() 
                                                         || self.is_comma() 
                                                         || self.is_forward_slash()
                                    )
            },
            _ => false,
        }
    }
}

#[inline]
fn is_utf8(token: char) -> bool {
    (token >= 0x00 as char) || (token <= 0xFF as char)
}

fn make_other_token(character: char) -> Option<ArmorToken> {
    if character.is_uppercase() {
        Some(ArmorToken::UpperCaseLetter(character))
    } else if character.is_lowercase() {
        Some(ArmorToken::LowerCaseLetter(character))
    } else if character.is_digit(10) {
        Some(ArmorToken::Digit(character))
    } else if is_utf8(character) {
        Some(ArmorToken::OtherUtf8(character))
    } else {
        None
    }
}

pub struct ArmorLexer<'a> {
    stream: String,
    chars:  Chars<'a>,
}

impl<'a> ArmorLexer<'a> {
    pub fn new(stream: &str) -> ArmorLexer {
        let new_stream = String::from(stream);

        ArmorLexer {
            stream: new_stream,
            chars:  stream.chars(),
        }
    }

    pub fn next_token(&mut self) -> Option<ArmorToken> {
        let next_char = self.chars.next();

        match next_char {
            None                    => None,
            Some(' ')  | Some('\t') => Some(ArmorToken::WhiteSpace(' ')),
            Some('\n') | Some('\r') => Some(ArmorToken::NewLine('\n')),
            Some('=')               => Some(ArmorToken::EqualSign('=')),
            Some('/')               => Some(ArmorToken::ForwardSlash('/')),
            Some(':')               => Some(ArmorToken::Colon(':')),
            Some(',')               => Some(ArmorToken::Comma(',')), 
            Some(character)         => make_other_token(character),
        } 
    }

    pub fn reset(&'a mut self) {
        self.chars = self.stream.chars();
    }
}

impl<'a> Iterator for ArmorLexer<'a> {
    type Item = ArmorToken;

    fn next(&mut self) -> Option<ArmorToken> {
        self.next_token()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.chars.size_hint()
    }
}


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
