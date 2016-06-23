#![allow(dead_code)]
// The constants CRC24_INIT and CRC24_POLY are defined in section 6.1
// of RFC4880 along with the definition of the CRC octet validator.
const CRC24_INIT: usize = 0xB704CE;
const CRC24_POLY: usize = 0x1864CFB;

type Crc24 = usize;

// This is an adaption of the CRC-24 algorithm from section 6.1 of TFC4880.
#[allow(unused_variables)]
fn crc_octets(octets: &[u8]) -> Crc24 {
    let mut crc: Crc24 = CRC24_INIT;
    
    for octet in octets {
        crc ^= (*octet as usize) << 16;
        for i in 0..8 {
            crc <<= 1;
            if crc & 0x1000000 != 0 {
                crc ^= CRC24_POLY;
            }
        }
    }
    // Fit crc24 into 24 bits.
    crc & 0xFFFFFF
}
