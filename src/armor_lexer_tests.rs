use armor_lexer::ArmorLexer;

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