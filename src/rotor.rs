const ROTOR_1_ALPHABET: [char; 26] = [
    'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O',
    'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J'
];

const ROTOR_2_ALPHABET: [char; 26] = [
    'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W',
    'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E'
];

pub struct RotorTyre {
    notch: usize,
    alphabet: &'static [char; 26]
}

const ROTOR_I: RotorTyre = RotorTyre {
    // Rollover when 'Q' is in the "window" and moves to 'R'.
    // Notch is actually aligned with 'Y'.
    notch: 14,
    alphabet: &ROTOR_1_ALPHABET
};

const ROTOR_II: RotorTyre = RotorTyre {
    // Rollover when 'E' is in the "window" and moves to 'F'.
    // Notch is actually aligned with 'M'
    notch: 14,
    alphabet: &ROTOR_2_ALPHABET
};

pub struct Rotor {
    tyre: &'static RotorTyre,
    pos: usize,
    ring_loc: usize
}

impl Rotor {
    pub fn new(rotor_id: &str) -> Self {
        Rotor::new_with_state(rotor_id, 0, 0)
    }

    pub fn new_with_state(rotor_id: &str, pos: usize, ring_loc: usize) -> Self {
        let tyre = match rotor_id {
            "I" => &ROTOR_I,
            "II" => &ROTOR_II,
            _ => panic!("Invalid rotor identifier {}", rotor_id),
        };

        Rotor {
            tyre,
            pos: (pos).rem_euclid(26),
            ring_loc: (ring_loc).rem_euclid(26)
        }
    }

    pub fn new_custom_rotor(tyre: &'static RotorTyre, pos: usize, ring_loc: usize) -> Self {
        Rotor {tyre, pos, ring_loc}
    }

    pub fn rotate(&mut self) {
        self.pos = (self.pos + 1).rem_euclid(26);
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
}
