use std::marker::PhantomData;
use combine::combinator::{And, Or, Token, Satisfy, Expected};
use combine::primitives::Stream;
use combine::{ParserExt, Parser, State, ParseResult, ParseError};
use combine::char;
use combine::char::{Upper, Lower, Spaces, NewLine, CrLf};


macro_rules! parser_impl {
    ( $ name : ident, $ inner_parser_type : ty, $ output_type : ty ) => {
        #[derive(Clone)]
        pub struct $name<I> where I: Stream<Item=char> {
            inner: $inner_parser_type,
            _marker: PhantomData<I>,
        }

        impl<I> Parser for $name<I> where I: Stream<Item=char> {
            type Input = I;
            type Output = output_type;

            fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
                self.inner.parse_lazy(input)
            }

            fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
                self.inner.add_error(_error);
            }
        }
    }
}

pub struct ParseBlankLine<I> where I: Stream<Item=char> {
    inner: And<Spaces<I>, Or<NewLine<I>, CrLf<I>>>,
    _marker: PhantomData<I>,
}

pub fn parse_blankline<I>() -> BlankLine<I> where I: Stream<Item=char> {
    ParseBlankLine {
        inner: char::spaces().and(char::newline().or(char::crlf())),
        _marker: PhantomData,
    }
}

impl<I> Parser for ParseBlankLine<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = <And<Spaces<I>, Or<NewLine<I>, CrLf<I>>> as Parser>::Output;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

pub struct ParseArmorHeaderLine where I: Stream<Item=char> {
    inner: 
    _marker: PhantomData<I>,
}

pub fn parse_armor_header_line<I>() -> ParseArmorHeaderLine<I> where I: Stream<Item=char> {
    ParseArmorHeaderLine {
        inner:,
        _marker: PhantomData, 
    }
}

impl<I> Parser for ParseArmorHeaderLine<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = ArmorType;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}

pub enum ArmorType {
    Message,
    PublicKeyBlock,
    PrivateKeyBlock,
    Signature,
    MessagePartXofY(usize, usize),
    MessagePartX(usize),
}

fn atype(at: ArmorType) -> String {
    match at {
        ArmorType::Message              => String::from("MESSAGE"),
        ArmorType::PublicKeyBlock       => String::from("PUBLIC KEY BLOCK"),
        ArmorType::PrivateKeyBlock      => String::from("PRIVATE KEY BLOCK"),
        ArmorType::ArmorSignature       => String::from("SIGNATURE"),
        ArmorType::MessagePartXofY(x,y) => format!("MESSAGE, PART {} / {}", x ,y),
        ArmorType::MessagePartX(x)      => format!("MESSAGE, PART {}", x),
    }
}

pub struct ParserMessageType where I: Stream<Item=char> {
    inner: 
    _marker: PhantomData<I>,
}

pub fn parse_message_type<I>() -> ParserMessageType<I> where I: Stream<Item=char> {
    let message = combine::string("MESSAGE").map(|x| ArmorType::Message);
    let pubkey  = combine::string("PUBLIC KEY BLOCK").map(|x| ArmorType::PublicKeyBlock);
    let privkey = combine::string("PRIVATE KEY BLOCK").map(|x| ArmorType::PrivateKeyBlock);
    let signature = combine::string("SIGNATURE").map(|x| ArmorType::Signature);
    let parts_def = combine::string("MESSAGE, PART ")
    let parts_indef = combine::string("MESSAGE, PART ")
    let num = combine::many1(combine::digit());

    ParseMessageType {
        inner:,
        _marker: PhantomData, 
    }
}

impl<I> Parser for ParserMessageType<I> where I: Stream<Item=char> {
    type Input = I;
    type Output = MessageType;

    fn parse_lazy(&mut self, input: Self::Input) -> ParseResult<Self::Output, Self::Input> {
        self.inner.parse_lazy(input)
    }

    fn add_error(&mut self, _error: &mut ParseError<Self::Input>) {
        self.inner.add_error(_error);
    }
}