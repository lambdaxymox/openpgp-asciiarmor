#![allow(dead_code)]
// The constants CRC24_INIT and CRC24_POLY are defined in section 6.1
// of RFC4880 along with the definition of the CRC octet validator.
const CRC24_INIT: usize = 0xB704CE;
const CRC24_POLY: usize = 0x1864CFB;

pub type Crc24 = usize;

// This is an adaption of the CRC-24 algorithm from section 6.1 of RFC4880.
#[allow(unused_variables)]
pub fn crc_octets(octets: &[u8]) -> Crc24 {
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


#[cfg(test)]
mod tests {
    use super::Crc24;


    struct TestCase {
        octets: Vec<u8>,
        crc: Crc24
    }

    struct Test {
        data: Vec<TestCase>
    }

    // An empty octet set should compute the crc as the
    // constant CRC24_INIT.
    fn crc_octets_empty_case() -> Test {
        Test {
            data: vec![
                TestCase {
                    octets: vec![],
                    crc: super::CRC24_INIT
                }
            ]
        }
    }

    fn crc_octets_test_cases() -> Test {
        Test {
            data: vec![
                TestCase {
                    octets: vec![0x14, 0xFB, 0x9C, 0x03, 0xD9, 0x7E],
                    crc: 6927321
                },
                TestCase {
                    octets: vec![0x14, 0xFB, 0x9C, 0x03, 0xD9],
                    crc: 8726480
                },
                TestCase {
                    octets: vec![0x14, 0xFB, 0x9C, 0x03],
                    crc: 15804535
                },
                TestCase {
                    octets: vec![0xD5, 0xF0, 0x32, 0x8A, 0xA0, 0xA9,
                                 0x1F, 0xAD, 0x1D, 0xDC, 0x22, 0xB6],
                    crc: 6688197
                }
            ]
        }
    }

    fn run_tests(test: &Test) {
        for test_case in test.data.iter() {
            let crc24 = super::crc_octets(test_case.octets.as_ref());
            assert_eq!(test_case.crc, crc24);
        }
    }

    #[test]
    fn test_crc_octets() {
        run_tests(&crc_octets_test_cases());
    }

    #[test]
    fn test_crc_octets_empty_case() {
        run_tests(&crc_octets_empty_case());
    }
}
