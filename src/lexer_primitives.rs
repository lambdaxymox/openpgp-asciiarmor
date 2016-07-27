use std::marker::PhantomData;
use combine::combinator::Token;
use combine::primitives::Stream;
use combine::{Parser, ParseResult, ParseError};
use combine::char;
use combine::char::{Upper, Lower, Space, CrLf};


#[derive(Clone)]
pub struct UpperCaseLetter<I> where I: Stream<Item=char> {
    inner: Upper<I>,
    _marker: PhantomData<I>,
}

pub fn uppercase_letter<I>() -> UpperCaseLetter<I> 
    where I: Stream<Item=char> {

    UpperCaseLetter {
        inner: char::upper(),
        _marker: PhantomData,
    }
}

impl<I> Parser for UpperCaseLetter<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <Upper<I> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

#[derive(Clone)]
pub struct LowerCaseLetter<I> where I: Stream<Item=char> {
    inner: Lower<I>,
    _marker: PhantomData<I>,
}

pub fn lowercase_letter<I>() -> LowerCaseLetter<I> 
    where I: Stream<Item=char> {

    LowerCaseLetter {
        inner: char::lower(),
        _marker: PhantomData,
    }

}

impl<I> Parser for LowerCaseLetter<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <Lower<I> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

#[derive(Clone)]
pub struct EqualSign<I> where I: Stream<Item=char> {
    inner: Token<I>,
    _marker: PhantomData<I>,
}

pub fn equal_sign<I>() -> EqualSign<I> where I: Stream<Item=char> {
    EqualSign { 
        inner: char::char('='),
        _marker: PhantomData,
    }
}

impl<I> Parser for EqualSign<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <Token<I> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

#[derive(Clone)]
pub struct Colon<I> where I: Stream<Item=char> {
    inner: Token<I>,
    _marker: PhantomData<I>,
}

pub fn colon<I>() -> Colon<I> where I: Stream<Item=char> {
    Colon {
        inner: char::char(':'),
        _marker: PhantomData,
    }
}

impl<I> Parser for Colon<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <Token<I> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

#[derive(Clone)]
pub struct Digit<I> where I: Stream<Item=char> {
    inner: char::Digit<I>,
    _marker: PhantomData<I>,
}

pub fn digit<I>() -> Digit<I> where I: Stream<Item=char> {
    Digit {
        inner: char::digit(),
        _marker: PhantomData,
    }
}

impl<I> Parser for Digit<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <char::Digit<I> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

#[derive(Clone)]
pub struct WhiteSpace<I> where I: Stream<Item=char> {
    inner: Space<I>,
    _marker: PhantomData<I>,
}

pub fn whitespace<I>() -> WhiteSpace<I> where I: Stream<Item=char> {
    WhiteSpace {
        inner: char::space(),
        _marker: PhantomData,
    }
}

impl<I> Parser for WhiteSpace<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <Space<I> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

#[derive(Clone)]
pub struct Comma<I> where I: Stream<Item=char> {
    inner: Token<I>,
    _marker: PhantomData<I>,
}

pub fn comma<I>() -> Comma<I> where I: Stream<Item=char> {
    Comma {
        inner: char::char(','),
        _marker: PhantomData,
    }
}

impl<I> Parser for Comma<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <Token<I> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

#[derive(Clone)]
pub struct ForwardSlash<I> where I: Stream<Item=char> {
    inner: Token<I>,
    _marker: PhantomData<I>,
}

pub fn forward_slash<I>() -> ForwardSlash<I> where I: Stream<Item=char> {
    ForwardSlash {
        inner: char::char('/'),
        _marker: PhantomData,
    }
}

impl<I> Parser for ForwardSlash<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <Token<I> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

#[derive(Clone)]
pub struct NewLine<I> where I: Stream<Item=char> {
    inner: char::NewLine<I>,
    _marker: PhantomData<I>,
}

pub fn newline<I>() -> NewLine<I> where I: Stream<Item=char> {
    NewLine {
        inner: char::newline(),
        _marker: PhantomData,
    }
}

impl<I> Parser for NewLine<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <char::NewLine<I> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
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

    }
}
