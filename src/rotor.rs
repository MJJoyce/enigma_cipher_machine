// ['E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O',
// 'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J']
const ROTOR_1_ALPHABET: [u8; 26] = [
    4, 10, 12, 5, 11, 6, 3, 16, 21, 25, 13, 19, 14, 22, 24, 7, 23, 20, 18, 15, 0, 8, 1, 17, 2, 9,
];

// ['A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W',
// 'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E'];
const ROTOR_2_ALPHABET: [u8; 26] = [
    0, 9, 3, 10, 18, 8, 17, 20, 23, 1, 11, 7, 22, 19, 12, 2, 16, 6, 25, 13, 15, 24, 5, 21, 14, 4,
];

pub struct RotorTyre {
    notch: u8,
    alphabet: &'static [u8; 26],
}

const ROTOR_I: RotorTyre = RotorTyre {
    // Rollover when stepping from 'Q' to 'R' (16 -> 17)
    notch: 16,
    alphabet: &ROTOR_1_ALPHABET,
};

const ROTOR_II: RotorTyre = RotorTyre {
    // Rollover when 'stepping from E' to 'F' (4 -> 5)
    notch: 4,
    alphabet: &ROTOR_2_ALPHABET,
};

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

    pub fn will_step_next_rotor(self) -> bool {
        self.pos == self.tyre.notch
    }

    pub fn map(&self, input_val: u8) -> u8 {
        self.tyre.alphabet[(input_val + self.pos + self.ring_loc).rem_euclid(26) as usize]
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
    fn test_mapping_basic() {
        // Check that our mapping calculation handles the simple case
        // at the beginning of an alphabet.
        // Rotor1: E, K, M, F
        let mut r = Rotor::new("I");
        assert_eq!(r.map(0), b'E' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'K' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'M' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'F' - b'A');
        r.rotate();

        // Check that our ring adjustment is shifting the alphabet as expected
        // Rotor1 with ring_loc 1 shifts our answers 'up' 1
        // Rotor1: K, M, F
        let mut r = Rotor::new_with_state("I", 0, 1);
        assert_eq!(r.map(0), b'K' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'M' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'F' - b'A');
        r.rotate();
    }

    #[test]
    fn test_mapping_rollover() {
        // Check our mapping when we need to rollover to the start of our
        // rings alphabet.
        let mut r = Rotor::new_with_state("I", 22, 0);
        r.rotate();
        assert_eq!(r.map(0), b'R' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'C' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'J' - b'A');
        // We rollover here
        r.rotate();
        assert_eq!(r.map(0), b'E' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'K' - b'A');

        // The above should be the same if we adjust the ring position as well
        let mut r = Rotor::new_with_state("I", 0, 22);
        r.rotate();
        assert_eq!(r.map(0), b'R' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'C' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'J' - b'A');
        // We rollover here
        r.rotate();
        assert_eq!(r.map(0), b'E' - b'A');
        r.rotate();
        assert_eq!(r.map(0), b'K' - b'A');
    }
}
