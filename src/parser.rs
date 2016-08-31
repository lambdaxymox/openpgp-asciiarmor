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
