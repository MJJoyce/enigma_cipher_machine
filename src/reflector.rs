// E, J, M, Z, A, L, Y, X, V, B, W, F, C, R, Q, U, O, N, T, S, P, I, K, H, G, D
const REFLECTOR_A_ALPHABET: [u8; 26] = [
    4, 9, 12, 25, 0, 11, 24, 23, 21, 1, 22, 5, 2, 17, 16, 20, 14, 13, 19, 18, 15, 8, 10, 7, 6, 3,
];

//Y, R, U, H, Q, S, L, D, P, X, N, G, O, K, M, I, E, B, F, Z, C, W, V, J, A, T
const REFLECTOR_B_ALPHABET: [u8; 26] = [
    24, 17, 20, 7, 16, 18, 11, 3, 15, 23, 13, 6, 14, 10, 12, 8, 4, 1, 5, 25, 2, 22, 21, 9, 0, 19,
];

//F, V, P, J, I, A, O, Y, E, D, R, Z, X, W, G, C, T, K, U, Q, S, B, N, M, H, L
const REFLECTOR_C_ALPHABET: [u8; 26] = [
    5, 21, 15, 9, 8, 0, 14, 24, 4, 3, 17, 25, 23, 22, 6, 2, 19, 10, 20, 16, 18, 1, 13, 12, 7, 11,
];

#[derive(Debug, Clone)]
pub struct Reflector {
    alphabet: &'static [u8; 26],
}

impl Reflector {
    pub fn new(reflector_id: &str) -> Reflector {
        match reflector_id {
            "A" => REFLECTOR_A,
            "B" => REFLECTOR_B,
            "C" => REFLECTOR_C,
            _ => panic!("Invalid Reflector id: {}", reflector_id),
        }
    }

    pub fn map(&self, input_val: u8) -> u8 {
        self.alphabet[input_val as usize]
    }
}

impl Default for Reflector {
    fn default() -> Self {
        Self::new("A")
    }
}

const REFLECTOR_A: Reflector = Reflector {
    alphabet: &REFLECTOR_A_ALPHABET,
};
const REFLECTOR_B: Reflector = Reflector {
    alphabet: &REFLECTOR_B_ALPHABET,
};
const REFLECTOR_C: Reflector = Reflector {
    alphabet: &REFLECTOR_C_ALPHABET,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn super_userful_reflector_test() {
        for (cnt, &item) in REFLECTOR_A.alphabet.iter().enumerate() {
            assert_eq!(REFLECTOR_A.map(cnt as u8), item);
        }
    }
}
