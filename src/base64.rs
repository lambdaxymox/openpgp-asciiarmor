#![allow(dead_code)]
pub type Octet = u32;
// Mask for keeping octets as 3 bytes.
const OCTET_MASK: u32 = 0x00FF_FFFF;

pub type Sextet = u32;
// Mask for keeping sextets as 6 bits.
const SEXTET_MASK: u32 = 0x3F;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Base64 {
    data: Vec<Sextet>
}

impl Base64 {
    pub fn new(data: &[Octet]) -> Base64 {
        Base64 {
            data: Vec::from(data)
        }
    }

    pub fn push(&mut self, sextet: Sextet) {
        self.data.push(sextet & SEXTET_MASK);
    }

    pub fn push_octet(&mut self, octet: Octet) {
        for i in 0..4 {
            let sextet_i = octet & (SEXTET_MASK << (6*(3-i)));
            let sextet   = sextet_i >> (6*(3-i));
            self.data.push(sextet);
        }
    }

    pub fn from_octet(octet: Octet) -> Base64 {
        let mut data = Vec::new();
        for i in 0..4 {
            let sextet_i = octet & (SEXTET_MASK << (6*(3-i)));
            let sextet   = sextet_i >> (6*(3-i));
            data.push(sextet);
        }

        Base64::new(data.as_ref())
    }

    pub fn to_octet(&self) -> Option<Octet> {
        if self.data.len() <= 4 {
            let mut octet = 0;
            for sextet in &self.data {
                octet <<= 6;
                octet |= sextet & SEXTET_MASK;
            }

            Some(octet)
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use super::Octet;
    use super::Base64;


    struct Test {
        data: Vec<Octet>
    }

    fn base64_test_cases() -> Test {
        Test {
            data: vec![
                0x6E7995, 0xDB36D1, 0x5BB967, 0x68BC48, 0x6A9552, 0x67C806, 0x5E1AFE, 0x8CB6AA,
                0xE6F722, 0xE9B626, 0x2E72D1, 0xE5ABFD, 0x3CDEA8, 0x428010, 0xACD964, 0x6F902C
            ]
        }
    }

    fn run_tests(test: &Test) {
        for test_case in test.data.iter() {
            let base64 = Base64::from_octet(*test_case);
            let result = base64.to_octet().unwrap();
            assert_eq!(*test_case, result);
        }
    }

    #[test]
    fn test_base64() {
        run_tests(&base64_test_cases());
    }
}
