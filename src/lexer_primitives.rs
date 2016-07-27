use std::marker::PhantomData;
use combine::combinator::{Expected, Satisfy};
use combine::combinator;
use combine::primitives::Stream;
use combine::{ParserExt, Parser, ParseResult, ParseError};
use combine::char;


macro_rules! lexer_combinator_def {
    ( $ name : ident, $ inner_parser_type : ty ) => {
        #[derive(Clone)]
        pub struct $name<I> where I: Stream<Item=char> {
            inner: $inner_parser_type,
            _marker: PhantomData<I>,
        }
    }
}

macro_rules! lexer_combinator_parser_impl {
    ( $ name : ident, $ inner_parser_type : ty ) => {
        impl<I> Parser for $name<I> where I: Stream<Item=char> {
            type Input = I;
            type Output = <$inner_parser_type as Parser>::Output;

            fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
                self.inner.parse_lazy(input)
            }

            fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
                self.inner.add_error(_error);
            }
        }
    }
}

macro_rules! lexer_combinator_impl {
    ( $ name : ident, $ inner_parser_type : ty ) => {
        lexer_combinator_def!($name, $inner_parser_type);

        lexer_combinator_parser_impl!($name, $inner_parser_type);
    }
}

lexer_combinator_impl!(UpperCaseLetter, char::Upper<I>);

pub fn uppercase_letter<I>() -> UpperCaseLetter<I> 
    where I: Stream<Item=char> {

    UpperCaseLetter {
        inner: char::upper(),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(LowerCaseLetter, char::Lower<I>);

pub fn lowercase_letter<I>() -> LowerCaseLetter<I> 
    where I: Stream<Item=char> {

    LowerCaseLetter {
        inner: char::lower(),
        _marker: PhantomData,
    }

}

lexer_combinator_impl!(EqualSign, combinator::Token<I>);

pub fn equal_sign<I>() -> EqualSign<I> where I: Stream<Item=char> {
    EqualSign { 
        inner: char::char('='),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(Colon, combinator::Token<I>);

pub fn colon<I>() -> Colon<I> where I: Stream<Item=char> {
    Colon {
        inner: char::char(':'),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(Digit, char::Digit<I>);

pub fn digit<I>() -> Digit<I> where I: Stream<Item=char> {
    Digit {
        inner: char::digit(),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(WhiteSpace, char::Space<I>);

pub fn whitespace<I>() -> WhiteSpace<I> where I: Stream<Item=char> {
    WhiteSpace {
        inner: char::space(),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(Comma, combinator::Token<I>);

pub fn comma<I>() -> Comma<I> where I: Stream<Item=char> {
    Comma {
        inner: char::char(','),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(ForwardSlash, combinator::Token<I>);

pub fn forward_slash<I>() -> ForwardSlash<I> where I: Stream<Item=char> {
    ForwardSlash {
        inner: char::char('/'),
        _marker: PhantomData,
    }
}

lexer_combinator_impl!(NewLine, char::NewLine<I>);

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

lexer_combinator_impl!(OtherUtf8, Expected<Satisfy<I, fn(I::Item) -> bool>>);

pub fn other_utf8<I>() -> OtherUtf8<I> where I: Stream<Item=char> {
    OtherUtf8 {
        inner: combinator::satisfy::<I, fn(char)-> bool>(is_other_utf8).expected("UTF-8 character"),
        _marker: PhantomData,
    }
}

/*
pub struct ArmorLexer<I> where I: Stream<Item=char> {
    inner: 
}

pub fn armor_lexer<I>() -> ArmorLexer<I>
    where I: Stream<item=char> {

    ArmorLexer {
        inner: 
    }
}
*/

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

    }
}
