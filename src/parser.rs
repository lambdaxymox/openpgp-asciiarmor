use std::str;
use std::fmt;
use nom;


#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MessageType {
    PGPMessage,
    PGPPublicKeyBlock,
    PGPPrivateKeyBlock,
    PGPSignature,
    PGPMessagePartXofY(usize, usize),
    PGPMessagePartX(usize)
}

impl fmt::Display for MessageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MessageType::PGPMessage => {
                write!(f, "PGP MESSAGE")
            }
            MessageType::PGPPublicKeyBlock => {
                write!(f, "PGP PUBLIC KEY BLOCK")
            }
            MessageType::PGPPrivateKeyBlock => {
                write!(f, "PGP PRIVATE KEY BLOCK")
            }
            MessageType::PGPSignature => {
                write!(f, "PGP SIGNATURE")
            }
            MessageType::PGPMessagePartXofY(x, y) => {
                write!(f, "PGP MESSAGE PART {}/{}", x ,y)
            }
            MessageType::PGPMessagePartX(x) => {
                write!(f, "PGP MESSAGE PART {}", x)
            }
        }
    }
}

named!(five_dashes,      tag!("-----"));
named!(begin_symbol,     tag!("BEGIN "));
named!(end_symbol,       tag!("END "));
named!(pgp_symbol,       tag!("PGP "));
named!(part_symbol,      tag!(", PART "));
named!(forward_slash,    tag!("/"));
named!(message_symbol,   tag!("MESSAGE"));
named!(signature_symbol, tag!("SIGNATURE"));
named!(public_key_block_symbol,  tag!("PUBLIC KEY BLOCK"));
named!(private_key_block_symbol, tag!("PRIVATE KEY BLOCK"));


named!(number <usize>,
    map!(
        take_while1!(nom::is_digit),
        |bytes: &[u8]| {
            str::parse::<usize>(str::from_utf8(bytes).unwrap()).unwrap()
        }
    )
);

named!(number_slash_number <(usize, usize)>,
    chain!(
        num_x: number ~
        forward_slash ~
        num_y: number,
        ||{ (num_x, num_y) }
    )
);

named!(parse_pgp_message_part_x_of_y <MessageType>,
    chain!(
        pair: number_slash_number,
        || { MessageType::PGPMessagePartXofY(pair.0, pair.1) }
    )
);

named!(parse_pgp_message_part_x <MessageType>,
    chain!(
        x: number,
        || { MessageType::PGPMessagePartX(x) }
    )
);

named!(parse_pgp_message_numbered <MessageType>,
    chain!(
        part_symbol ~
        message_type: alt!(
              parse_pgp_message_part_x_of_y
            | parse_pgp_message_part_x
        ),
        ||{ message_type }
    )
);

named!(parse_pgp_message_unnumbered <MessageType>,
    chain!(
        take!(0),
        || { MessageType::PGPMessage }
    )
);

named!(parse_pgp_message_block <MessageType>,
    chain!(
        message_symbol ~
        message_type: alt!(
              parse_pgp_message_numbered
            | parse_pgp_message_unnumbered
        ),
        || { message_type }
    )
);

named!(parse_pgp_public_key_block <MessageType>,
    chain!(
        public_key_block_symbol,
        || { MessageType::PGPPublicKeyBlock }
    )
);

named!(parse_pgp_private_key_block <MessageType>,
    chain!(
        private_key_block_symbol,
        || { MessageType::PGPPrivateKeyBlock }
    )
);

named!(parse_pgp_signature <MessageType>,
    chain!(
        signature_symbol,
        || { MessageType::PGPSignature }
    )
);

named!(parse_header_line <MessageType>,
    chain!(
        five_dashes  ~
        begin_symbol ~
        pgp_symbol   ~
        message_type: alt!(
              parse_pgp_message_block
            | parse_pgp_public_key_block
            | parse_pgp_private_key_block
            | parse_pgp_signature
        ) ~
        five_dashes,
        || { message_type }
    )
);

named!(parse_footer_line <MessageType>,
    chain!(
        five_dashes  ~
        end_symbol ~
        pgp_symbol   ~
        message_type: alt!(
              parse_pgp_message_block
            | parse_pgp_public_key_block
            | parse_pgp_private_key_block
            | parse_pgp_signature
        ) ~
        five_dashes,
        || { message_type }
    )
);

pub enum HeaderLineType {
    Version,
    Comment,
    MessageID,
    Hash,
    Charset,
    Other(String),
}

named!(version_symbol <HeaderLineType>,
    chain!(
        tag!("Version"), ||{ HeaderLineType::Version }
    )
);
named!(colon_space_symbol, tag!(": "));

named!(comment_symbol <HeaderLineType>,
    chain!(
        tag!("Comment"), ||{ HeaderLineType::Comment }
    )
);

named!(message_id_symbol <HeaderLineType>,
    chain!(
        tag!("MessageID"), ||{ HeaderLineType::MessageID }
    )
);

named!(hash_symbol <HeaderLineType>,
    chain!(
        tag!("Hash"), ||{ HeaderLineType::Hash }
    )
);

named!(charset_symbol <HeaderLineType>,
    chain!(
        tag!("Charset"), ||{ HeaderLineType::Charset }
    )
);

named!(other_header_symbol <HeaderLineType>,
    map!(
        take_until!(": "),
        |tag: &[u8]| {
            let string = String::from(str::from_utf8(tag).unwrap());

            HeaderLineType::Other(string)
        }
    )
);

named!(parse_header_line_type <HeaderLineType>,
    alt!( version_symbol
        | comment_symbol
        | message_id_symbol
        | hash_symbol
        | charset_symbol
        | other_header_symbol
    )
);

named!(parse_header_line_data <String>,
    chain!(
        line: is_not!("\r\n") ~
        is_a!("\r\n"),
        || { String::from(str::from_utf8(line).unwrap()) }
    )
);

named!(parse_header_data_line <(HeaderLineType, String)>,
    chain!(
        header_line_type: parse_header_line_type ~
        colon_space_symbol ~
        header_line_data: parse_header_line_data,
        ||{ (header_line_type, header_line_data) }
    )
);

named!(parse_header_data <(Vec<(HeaderLineType, String)>)>, many0!(parse_header_data_line));

named!(blankline <()>, chain!(is_a!(" ") ~ is_a!("\r\n"), ||{}));

named!(parse_header <(MessageType, Vec<(HeaderLineType, String)>)>,
    chain!(
        message_type: parse_header_line ~
        header_data: parse_header_data ~
        blankline,
        ||{ (message_type, header_data) }
    )
);

named!(pad_symbol, tag!("="));

named!(parse_footer <MessageType>, chain!(message_type: parse_footer_line, ||{ message_type }));

fn is_base64(ch: u8) -> bool {
    b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789/+".contains(&ch)
}

named!(parse_body_line <&[u8]>,
    chain!(
        line: take_while!(is_base64) ~
        is_a!("\r\n"),
        ||{ line }
    )
);

named!(parse_body <(Vec<u8>)>,
    chain!(
        lines: many0!(parse_body_line),
        || {
            let mut vec: Vec<u8> = Vec::new();
            for line in &lines {
                let mut line_vec = line.iter().cloned().collect::<Vec<u8>>();
                vec.append(&mut line_vec);
            }

            vec
        }
    )
);
