use std::str::Chars;
use std::iter::Iterator;


const LOWER_CASE_LETTERS: [char; 26] = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 
        'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'
    ];

const UPPER_CASE_LETTERS: [char; 26] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 
        'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z'
    ];

const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];


#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum TokenType {
    Character,
    NewLine,
    WhiteSpace,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ArmorToken {
    token_type: TokenType,
    token: char,
}

impl ArmorToken {
    pub fn new(token_type: TokenType, token: char) -> ArmorToken {
        ArmorToken {
            token_type: token_type,
            token: token,
        }
    }

    pub fn valid_token(&self) -> bool {
        match self.token {
            ' '  | '\t' => self.token_type == TokenType::WhiteSpace,
            '\n' | '\r' => self.token_type == TokenType::NewLine,
            _           => self.token_type == TokenType::Character, 
        }
    }

    pub fn token_type(&self) -> TokenType {
        self.token_type
    }

    pub fn token(&self) -> char {
        self.token
    }

    #[inline]
    pub fn is_letter(&self) -> bool {
        self.is_upper_case() || self.is_lower_case()
    }

    #[inline]
    pub fn is_upper_case(&self) -> bool {
        UPPER_CASE_LETTERS.contains(&self.token)
    }

    #[inline]
    pub fn is_lower_case(&self) -> bool {
        LOWER_CASE_LETTERS.contains(&self.token)
    }

    #[inline]
    pub fn is_digit(&self) -> bool {
        DIGITS.contains(&self.token)
    }

    #[inline]
    pub fn is_equal_sign(&self) -> bool {
        self.token == '='
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
            Some(' ')  | Some('\t') => Some(ArmorToken::new(TokenType::WhiteSpace, ' ')),
            Some('\n') | Some('\r') => Some(ArmorToken::new(TokenType::NewLine, '\n')),
            Some(character)         => Some(ArmorToken::new(TokenType::Character, character)), 
        } 
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
        }
    }
}