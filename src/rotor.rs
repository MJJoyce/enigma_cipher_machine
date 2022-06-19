// [E, K, M, F, L, G, D, Q, V, Z, N, T, O, W, Y, H, X, U, S, P, A, I, B, R, C, J]
const ROTOR_1_ALPHABET_IN: [u8; 26] = [
    4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9,
];

const ROTOR_1_ALPHABET_OUT: [u8; 26] = [
    20, 22, 24, 6, 0, 3, 5, 15, 21, 25, 1, 4, 2, 10, 12, 19, 7, 23, 18, 11, 17, 8, 13, 16, 14, 9,
];

// [A, J, D, K, S, I, R, U, X, B, L, H, W, T, M, C, Q, G, Z, N, P, Y, F, V, O, E];
const ROTOR_2_ALPHABET_IN: [u8; 26] = [
    0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4,
];

const ROTOR_2_ALPHABET_OUT: [u8; 26] = [
    0, 9, 15, 2, 25, 22, 17, 11, 5, 1, 3, 10, 14, 19, 24, 20, 16, 6, 4, 13, 7, 23, 12, 8, 21, 18,
];

// [B, D, F, H, J, L, C, P, R, T, X, V, Z, N, Y, E, I, W, G, A, K, M, U, S, Q, O]
const ROTOR_3_ALPHABET_IN: [u8; 26] = [
    1, 3, 5, 7, 9, 11, 2, 15, 17, 19, 23, 21, 25, 13, 24, 4, 8, 22, 6, 0, 10, 12, 20, 18, 16, 14,
];

const ROTOR_3_ALPHABET_OUT: [u8; 26] = [
    19, 0, 6, 1, 15, 2, 18, 3, 16, 4, 20, 5, 21, 13, 25, 7, 24, 8, 23, 9, 22, 11, 17, 10, 14, 12,
];

// [E, S, O, V, P, Z, J, A, Y, Q, U, I, R, H, X, L, N, F, T, G, K, D, C, M, W, B]
const ROTOR_4_ALPHABET_IN: [u8; 26] = [
    4, 18, 14, 21, 15, 25, 9, 0, 24, 16, 20, 8, 17, 7, 23, 11, 13, 5, 19, 6, 10, 3, 2, 12, 22, 1,
];

const ROTOR_4_ALPHABET_OUT: [u8; 26] = [
    7, 25, 22, 21, 0, 17, 19, 13, 11, 6, 20, 15, 23, 16, 2, 4, 9, 12, 1, 18, 10, 3, 24, 14, 8, 5,
];

// [V, Z, B, R, G, I, T, Y, U, P, S, D, N, H, L, X, A, W, M, J, Q, O, F, E, C, K]
const ROTOR_5_ALPHABET_IN: [u8; 26] = [
    21, 25, 1, 17, 6, 8, 19, 24, 20, 15, 18, 3, 13, 7, 11, 23, 0, 22, 12, 9, 16, 14, 5, 4, 2, 10,
];

const ROTOR_5_ALPHABET_OUT: [u8; 26] = [
    16, 2, 24, 11, 23, 22, 4, 13, 5, 19, 25, 14, 18, 12, 21, 9, 20, 3, 10, 6, 8, 0, 17, 15, 7, 1,
];

// [J, P, G, V, O, U, M, F, Y, Q, B, E, N, H, Z, R, D, K, A, S, X, L, I, C, T, W]
const ROTOR_6_ALPHABET_IN: [u8; 26] = [
    9, 15, 6, 21, 14, 20, 12, 5, 24, 16, 1, 4, 13, 7, 25, 17, 3, 10, 0, 18, 23, 11, 8, 2, 19, 22,
];

const ROTOR_6_ALPHABET_OUT: [u8; 26] = [
    18, 10, 23, 16, 11, 7, 2, 13, 22, 0, 17, 21, 6, 12, 4, 1, 9, 15, 19, 24, 5, 3, 25, 20, 8, 14,
];

// [N, Z, J, H, G, R, C, X, M, Y, S, W, B, O, U, F, A, I, V, L, P, E, K, Q, D, T]
const ROTOR_7_ALPHABET_IN: [u8; 26] = [
    13, 25, 9, 7, 6, 17, 2, 23, 12, 24, 18, 22, 1, 14, 20, 5, 0, 8, 21, 11, 15, 4, 10, 16, 3, 19,
];

const ROTOR_7_ALPHABET_OUT: [u8; 26] = [
    16, 12, 6, 24, 21, 15, 4, 3, 17, 2, 22, 19, 8, 0, 13, 20, 23, 5, 10, 25, 14, 18, 11, 7, 9, 1,
];

// [F, K, Q, H, T, L, X, O, C, B, J, S, P, D, Z, R, A, M, E, W, N, I, U, Y, G, V]
const ROTOR_8_ALPHABET_IN: [u8; 26] = [
    5, 10, 16, 7, 19, 11, 23, 14, 2, 1, 9, 18, 15, 3, 25, 17, 0, 12, 4, 22, 13, 8, 20, 24, 6, 21,
];

const ROTOR_8_ALPHABET_OUT: [u8; 26] = [
    16, 9, 8, 13, 18, 0, 24, 3, 21, 10, 1, 5, 17, 20, 7, 12, 2, 15, 11, 4, 22, 25, 19, 6, 23, 14,
];

#[derive(Debug)]
enum RotorTyreNotch {
    Single(u8),
    Double([u8; 2]),
}

#[derive(Debug)]
pub struct RotorTyre {
    notch: RotorTyreNotch,
    alphabet_in: &'static [u8; 26],
    alphabet_out: &'static [u8; 26],
}

const ROTOR_I: RotorTyre = RotorTyre {
    // Rollover when stepping from 'Q' to 'R' (16 -> 17)
    notch: RotorTyreNotch::Single(16),
    alphabet_in: &ROTOR_1_ALPHABET_IN,
    alphabet_out: &ROTOR_1_ALPHABET_OUT,
};

const ROTOR_II: RotorTyre = RotorTyre {
    // Rollover when stepping from 'E' to 'F' (4 -> 5)
    notch: RotorTyreNotch::Single(4),
    alphabet_in: &ROTOR_2_ALPHABET_IN,
    alphabet_out: &ROTOR_2_ALPHABET_OUT,
};

const ROTOR_III: RotorTyre = RotorTyre {
    // Rollover when stepping from 'V' to 'W' (20 -> 21)
    notch: RotorTyreNotch::Single(20),
    alphabet_in: &ROTOR_3_ALPHABET_IN,
    alphabet_out: &ROTOR_3_ALPHABET_OUT,
};

const ROTOR_IV: RotorTyre = RotorTyre {
    // Rollover when stepping from 'J' to 'K' (8 -> 9)
    notch: RotorTyreNotch::Single(8),
    alphabet_in: &ROTOR_4_ALPHABET_IN,
    alphabet_out: &ROTOR_4_ALPHABET_OUT,
};

const ROTOR_V: RotorTyre = RotorTyre {
    // Rollover when stepping from 'Z' to 'A' (25 -> 0)
    notch: RotorTyreNotch::Single(25),
    alphabet_in: &ROTOR_5_ALPHABET_IN,
    alphabet_out: &ROTOR_5_ALPHABET_OUT,
};

const ROTOR_VI: RotorTyre = RotorTyre {
    // Rollover when stepping from 'Z' to 'A' (25 -> 0) or 'M' to 'N' (11 -> 12)
    notch: RotorTyreNotch::Double([25, 11]),
    alphabet_in: &ROTOR_6_ALPHABET_IN,
    alphabet_out: &ROTOR_6_ALPHABET_OUT,
};

const ROTOR_VII: RotorTyre = RotorTyre {
    // Rollover when stepping from 'Z' to 'A' (25 -> 0) or 'M' to 'N' (11 -> 12)
    notch: RotorTyreNotch::Double([25, 11]),
    alphabet_in: &ROTOR_7_ALPHABET_IN,
    alphabet_out: &ROTOR_7_ALPHABET_OUT,
};

const ROTOR_VIII: RotorTyre = RotorTyre {
    // Rollover when stepping from 'Z' to 'A' (25 -> 0) or 'M' to 'N' (11 -> 12)
    notch: RotorTyreNotch::Double([25, 11]),
    alphabet_in: &ROTOR_8_ALPHABET_IN,
    alphabet_out: &ROTOR_8_ALPHABET_OUT,
};

#[derive(Debug)]
pub struct Rotor {
    tyre: &'static RotorTyre,
    pos: u8,
    ring_loc: u8,
}

impl Rotor {
    pub fn new(rotor_id: &str) -> Self {
        Rotor::new_with_state(rotor_id, 0, 0)
    }

    pub fn new_with_state(rotor_id: &str, pos: u8, ring_loc: u8) -> Self {
        let tyre = match rotor_id {
            "I" => &ROTOR_I,
            "II" => &ROTOR_II,
            "III" => &ROTOR_III,
            "IV" => &ROTOR_IV,
            "V" => &ROTOR_V,
            "VI" => &ROTOR_VI,
            "VII" => &ROTOR_VII,
            "VIII" => &ROTOR_VIII,
            _ => panic!("Invalid rotor identifier {}", rotor_id),
        };

        Rotor {
            tyre,
            pos: (pos).rem_euclid(26),
            ring_loc: (ring_loc).rem_euclid(26),
        }
    }

    pub fn new_custom_rotor(tyre: &'static RotorTyre, pos: u8, ring_loc: u8) -> Self {
        Rotor {
            tyre,
            pos,
            ring_loc,
        }
    }

    pub fn rotate(&mut self) {
        self.pos = (self.pos + 1).rem_euclid(26);
    }

    pub fn will_step_next_rotor(&self) -> bool {
        match self.tyre.notch {
            RotorTyreNotch::Single(notch_index) => self.pos == notch_index,
            RotorTyreNotch::Double([notch1, notch2]) => self.pos == notch1 || self.pos == notch2,
        }
    }

    pub fn map_in(&self, input_val: u8) -> u8 {
        let trans = self.tyre.alphabet_in
            [(input_val + self.pos + self.ring_loc).rem_euclid(26) as usize]
            as i16;
        (trans - self.pos as i16).rem_euclid(26) as u8
    }

    pub fn map_out(&self, input_val: u8) -> u8 {
        let trans = self.tyre.alphabet_out
            [(input_val + self.pos + self.ring_loc).rem_euclid(26) as usize]
            as i16;
        (trans - self.pos as i16).rem_euclid(26) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut r = Rotor::new("I");
        assert_eq!(r.pos, 0);
        r.rotate();
        assert_eq!(r.pos, 1);

        // Test proper rollover at end of alphabet
        let mut r = Rotor::new_with_state("I", 25, 0);
        assert_eq!(r.pos, 25);
        r.rotate();
        assert_eq!(r.ring_loc, 0);
    }

    #[test]
    fn test_rotor_init() {
        // Ring and rotor position values >= 26 should be modulo'd
        let r = Rotor::new_with_state("I", 26, 26);
        assert_eq!(r.pos, 0);
        assert_eq!(r.ring_loc, 0);
    }

    #[test]
    fn test_rotor_step() {
        let r = Rotor::new_with_state("I", 17, 0);
        assert_eq!(false, r.will_step_next_rotor());

        let r = Rotor::new_with_state("I", 15, 0);
        assert_eq!(false, r.will_step_next_rotor());

        let r = Rotor::new_with_state("I", 16, 0);
        assert_eq!(true, r.will_step_next_rotor());
    }

    #[test]
    fn test_rotor_step_multinotch() {
        // Check the first notch values
        let (notch1, notch2) = match ROTOR_VI.notch {
            RotorTyreNotch::Double([notch1, notch2]) => (notch1, notch2),
            _ => panic!("This is literally impossible"),
        };

        let r = Rotor::new_with_state("VI", (notch1 - 1).rem_euclid(26), 0);
        assert_eq!(false, r.will_step_next_rotor());

        let r = Rotor::new_with_state("VI", notch1, 0);
        assert_eq!(true, r.will_step_next_rotor());

        let r = Rotor::new_with_state("VI", (notch1 + 1).rem_euclid(26), 0);
        assert_eq!(false, r.will_step_next_rotor());

        let r = Rotor::new_with_state("VI", (notch2 - 1).rem_euclid(26), 0);
        assert_eq!(false, r.will_step_next_rotor());

        let r = Rotor::new_with_state("VI", notch2, 0);
        assert_eq!(true, r.will_step_next_rotor());

        let r = Rotor::new_with_state("VI", (notch2 + 1).rem_euclid(26), 0);
        assert_eq!(false, r.will_step_next_rotor());
    }

    #[test]
    fn test_verify_alphabet_mappings() {
        let rotors = vec![
            ROTOR_I, ROTOR_II, ROTOR_III, ROTOR_IV, ROTOR_V, ROTOR_VI, ROTOR_VII, ROTOR_VIII,
        ];

        for rotor in rotors {
            for i in 0usize..rotor.alphabet_in.len() {
                assert_eq!(
                    i,
                    rotor.alphabet_out[rotor.alphabet_in[i] as usize] as usize
                );
            }
        }
    }

    #[test]
    fn test_sample_mapping() {
        let mut r = Rotor::new("I");
        assert_eq!(r.map_in(0), b'E' - b'A');
        assert_eq!(r.map_out(b'E' - b'A'), 0);
        r.rotate();
        assert_eq!(r.map_in(0), b'J' - b'A');
        assert_eq!(r.map_out(b'J' - b'A'), 0);
        r.rotate();
        assert_eq!(r.map_in(0), b'K' - b'A');
        assert_eq!(r.map_out(b'K' - b'A'), 0);
        r.rotate();
        assert_eq!(r.map_in(0), b'C' - b'A');
        assert_eq!(r.map_out(b'C' - b'A'), 0);
    }

    #[test]
    fn test_mapping_basic() {
        // Check that our mapping calculation handles the simple case
        // at the beginning of an alphabet.
        // Rotor1: E, K, M, F
        let mut r = Rotor::new("I");
        assert_eq!(r.map_in(0), b'E' - b'A');
        r.rotate();
        assert_eq!(r.map_in(0), b'K' - b'A' - r.pos);
        r.rotate();
        assert_eq!(r.map_in(0), b'M' - b'A' - r.pos);
        r.rotate();
        assert_eq!(r.map_in(0), b'F' - b'A' - r.pos);
        r.rotate();

        // Check that our ring adjustment is shifting the alphabet as expected
        // Rotor1 with ring_loc 1 shifts our answers 'up' 1
        // Rotor1: K, M, F
        let mut r = Rotor::new_with_state("I", 0, 1);
        assert_eq!(r.map_in(0), b'K' - b'A' - r.pos);
        r.rotate();
        assert_eq!(r.map_in(0), b'M' - b'A' - r.pos);
        r.rotate();
        assert_eq!(r.map_in(0), b'F' - b'A' - r.pos);
        r.rotate();
    }

    #[test]
    fn test_mapping_rollover() {
        // Check our mapping when we need to rollover to the start of our rings alphabet.
        let mut r = Rotor::new_with_state("I", 22, 0);
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'R' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'C' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'J' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );
        // We rollover here     (           )
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'E' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'K' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );

        // The above should be the same if we adjust the ring position as well
        let mut r = Rotor::new_with_state("I", 0, 22);
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'R' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'C' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'J' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );
        // We rollover here
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'E' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );
        r.rotate();
        assert_eq!(
            r.map_in(0) as i16,
            ((b'K' - b'A') as i16 - r.pos as i16).rem_euclid(26)
        );
    }
}
