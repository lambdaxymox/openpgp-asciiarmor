# OpenPGP Ascii Armor
A Rust implementation of the OpenPGP Ascii Armor format from RFC4880. See 
section 6 of RFC4880.

# Ascii Armor Grammar
The ascii armor parser uses the following grammer derived from the specification of
ascii armor in section 6 of RFC4880:
```
Text                := <<UTF-8>>
UpperCaseLetter     := 'A' | 'B' | ... | 'Z'
LowerCaseLetter     := 'a' | 'b' | ... | 'z'
Letter              := UpperCaseLetter | LowerCaseLetter
Digit               := '0' | '1' | ... | '9'
Number              := (Digit)+
EqualSign           := '='
Pad                 := EqualSign
NonPaddedBase64     := Letter | Number | ForwardSlash
Base64              := NonPaddedBase64 | Pad
ForwardSlash        := '/'
Colon               := ':'
Whitespace          := ' '
ColonSpace          := Colon Whitespace
NewLine             := '\r' | '\n'
BlankLine           := (Whitespace)* NewLine
FiveDashes          := "-----"
Begin               := "BEGIN"
End                 := "END"
Comma               := ','
Version             := "Version"
Comment             := "Comment"
MessageID           := "MessageID"
Hash                := "Hash"
Charset             := "Charset"
ListElement         := (Text)+
List                := ListElement (Comma ListElement)*
PGPMessage          := "PGP MESSAGE"
PGPPublicKeyBlock   := "PGP PUBLIC KEY BLOCK"
PGPPrivateKeyBlock  := "PGP PRIVATE KEY BLOCK"
PGPMessagePartXofY  := "PGP MESSAGE, PART "
PGPMessagePartX     := "PGP MESSAGE, PART "
PGPSignature        := "PGP SIGNATURE"
MessageType         :=  PGPMessage
                     |  PGPPublicKeyBlock
                     |  PGPPrivateKeyBlock
                     |  PGPSignature
                     |  PGPMessagePartXofY Number ForwardSlash Number
                     |  PGPMessagePartX    Number
ArmorHeaderLine     := FiveDashes Begin MessageType FiveDashes
ArmorTailLine       := FiveDashes End MessageType FiveDashes
ArmorHeaderKV       := Version ColonSpace (Text)*
                     | Comment ColonSpace (Text)*
                     | MessageID ColonSpace (Text)*(32)
                     | Hash ColonSpace List
                     | Charset (Text)*
ArmorHeader         := ArmorHeaderLine (ArmorHeaderKV)*
ArmorTail           := ArmorTailLine
ArmorDataLine       := (NonPaddedBase64)*(76)
ArmorPaddedDataLine := (NonPaddedBase64)* (Pad)*(76)
ArmorData           := (ArmorDataLine)* ArmorPaddedDataLine
ArmorDataChecksum   := Pad NonPaddedBase64 NonPaddedBase64 NonPaddedBase64 NonPaddedBase64
ArmorBlock          := ArmorHeader BlankLine ArmorData ArmorDataChecksum ArmorTail
Armor               := (ArmorBlock)+
```
The parser is a LL(k) recursive descent parser.