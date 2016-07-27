use std::marker::PhantomData;
use combine::combinator::{Or, Many, Choice, Expected, Satisfy};
use combine::combinator;
use combine::primitives::Stream;
use combine::primitives;
use combine::{ParserExt, Parser, ParseResult, ParseError};
use combine::char;


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

macro_rules! lexer_combinator_impl {
    ( $ name : ident, $ inner_parser_type : ty, $ armor_token : ident ) => {
        #[derive(Clone)]
        pub struct $name<I> where I: Stream<Item=char> {
            inner: $inner_parser_type,
            _marker: PhantomData<I>,
        }

        impl<I> Parser for $name<I> where I: Stream<Item=char> {
            type Input = I;
            type Output = ArmorToken;

            fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
                let result = self.inner.parse_lazy(input);
                match result {
                    Ok((parsed_char, consumed)) => Ok((ArmorToken::$armor_token(parsed_char), consumed)),
                    Err(e) => Err(e),
                }
            }

            fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
                self.inner.add_error(_error);
            }
        }
    }
}

lexer_combinator_impl!(UpperCaseLetter, char::Upper<I>, UpperCaseLetter);

pub fn uppercase_letter<I>() -> UpperCaseLetter<I> 
    where I: Stream<Item=char> {

    UpperCaseLetter {
        inner: char::upper(),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(LowerCaseLetter, char::Lower<I>, LowerCaseLetter);

pub fn lowercase_letter<I>() -> LowerCaseLetter<I> 
    where I: Stream<Item=char> {

    LowerCaseLetter {
        inner: char::lower(),
        _marker: PhantomData,
    }

}

lexer_combinator_impl!(EqualSign, combinator::Token<I>, EqualSign);

pub fn equal_sign<I>() -> EqualSign<I> where I: Stream<Item=char> {
    EqualSign { 
        inner: char::char('='),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(Colon, combinator::Token<I>, Colon);

pub fn colon<I>() -> Colon<I> where I: Stream<Item=char> {
    Colon {
        inner: char::char(':'),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(Digit, char::Digit<I>, Digit);

pub fn digit<I>() -> Digit<I> where I: Stream<Item=char> {
    Digit {
        inner: char::digit(),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(WhiteSpace, char::Space<I>, WhiteSpace);

pub fn whitespace<I>() -> WhiteSpace<I> where I: Stream<Item=char> {
    WhiteSpace {
        inner: char::space(),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(Comma, combinator::Token<I>, Comma);

pub fn comma<I>() -> Comma<I> where I: Stream<Item=char> {
    Comma {
        inner: char::char(','),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(ForwardSlash, combinator::Token<I>, ForwardSlash);

pub fn forward_slash<I>() -> ForwardSlash<I> where I: Stream<Item=char> {
    ForwardSlash {
        inner: char::char('/'),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(NewLine, char::NewLine<I>, NewLine);

pub fn newline<I>() -> NewLine<I> where I: Stream<Item=char> {
    NewLine {
        inner: char::newline(),
        _marker: PhantomData,
    }
}

#[inline]
fn is_utf8(ch: char) -> bool {
    (ch >= 0x00 as char) || (ch <= 0xFF as char)
}

#[inline]
fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t'
}

#[inline]
fn is_newline(ch: char) -> bool {
    (ch == '\n') || (ch == '\r')
}

fn is_other_utf8(ch: char) -> bool {
    is_utf8(ch) && !(is_whitespace(ch) || ch.is_uppercase()
                                       || ch.is_lowercase()
                                       || ch.is_digit(10)
                                       || is_newline(ch)
                                       || ch == '/' 
                                       || ch == ',' 
                                       || ch == ':' 
                                       || ch == '=')
}

lexer_combinator_impl!(OtherUtf8, Expected<Satisfy<I, fn(I::Item) -> bool>>, OtherUtf8);

pub fn other_utf8<I>() -> OtherUtf8<I> where I: Stream<Item=char> {
    OtherUtf8 {
        inner: combinator::satisfy::<I, fn(char)-> bool>(is_other_utf8).expected("UTF-8 character"),
        _marker: PhantomData,
    }
}

pub struct InnerArmorLexer<I> where I: Stream<Item=char> {
    inner: Or<UpperCaseLetter<I>, 
           Or<LowerCaseLetter<I>, 
           Or<EqualSign<I>, 
           Or<Colon<I>, 
           Or<Digit<I>, 
           Or<WhiteSpace<I>, 
           Or<Comma<I>, 
           Or<ForwardSlash<I>, 
           Or<NewLine<I>, OtherUtf8<I>>>>>>>>>>,

    _marker: PhantomData<I>,
}

impl<I> Parser for InnerArmorLexer<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = ArmorToken;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

pub fn inner_armor_lexer<I>() -> InnerArmorLexer<I>
    where I: Stream<Item=char>, 
{
    InnerArmorLexer {
        inner: uppercase_letter().or(lowercase_letter()
                                 .or(equal_sign()
                                 .or(colon()
                                 .or(digit()
                                 .or(whitespace()
                                 .or(comma()
                                 .or(forward_slash()
                                 .or(newline()
                                 .or(other_utf8()))))))))),
        _marker: PhantomData,
    }
}

pub struct ArmorLexer<I> where I: Stream<Item=char> {
    lexer: InnerArmorLexer<I>,
}

pub fn armor_lexer<I>() -> ArmorLexer<I> where I: Stream<Item=char> {
    ArmorLexer {
        lexer: inner_armor_lexer(),
    }
}

#[cfg(test)]
mod tests {
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
        let armor_lexer = super::armor_lexer::<&str>();
    }
}
