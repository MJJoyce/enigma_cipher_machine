const ROTOR_1_ALPHABET: [char; 26] = [
    'E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O',
    'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J'
];

const ROTOR_2_ALPHABET: [char; 26] = [
    'A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W',
    'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E'
];

//enum ROTOR_ALPHABET(

    //I(['E', 'K', 'M', 'F', 'L', 'G', 'D', 'Q', 'V', 'Z', 'N', 'T', 'O',
        //'W', 'Y', 'H', 'X', 'U', 'S', 'P', 'A', 'I', 'B', 'R', 'C', 'J']),
    //II(['A', 'J', 'D', 'K', 'S', 'I', 'R', 'U', 'X', 'B', 'L', 'H', 'W',
        //'T', 'M', 'C', 'Q', 'G', 'Z', 'N', 'P', 'Y', 'F', 'V', 'O', 'E'])
//)

struct RotorTyre {
    notch: usize,
    alphabet: &'static [char; 26]
}

struct RotorI {
    pos: usize,
    ring_loc: usize,
    tyre: RotorTyre
}

impl RotorI {
    fn new() -> Self {
        return RotorI::new_with_state(0, 0)
    }

    fn new_with_state(pos: usize, ring_loc: usize) -> Self {
        RotorI {
            pos: (pos).rem_euclid(26),
            ring_loc: (ring_loc).rem_euclid(26),
            tyre: RotorTyre {
                 // Rollover when 'Q' is in the "window". Notch is actually aligned with 'Y'
                notch: 16,
                alphabet: &ROTOR_1_ALPHABET
            }
        }
    }

    fn rotate(&mut self) {
        self.pos = (self.pos + 1).rem_euclid(26);

        let x = vec![(1, 2), (3, 4)]
            .into_iter()
            .map(|(a, b)| a + b)
            .collect::<Vec<usize>>();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate() {
        let mut r = RotorI::new();
        assert_eq!(r.pos, 0);
        r.rotate();
        assert_eq!(r.pos, 1);

        // Test proper rollover at end of alphabet
        let mut r = RotorI::new_with_state(25, 0);
        assert_eq!(r.pos, 25);
        r.rotate();
        assert_eq!(r.ring_loc, 0);
    }

    #[test]
    fn test_rotor_init() {
        // Ring and rotor position values >= 26 should be modulo'd
        let r = RotorI::new_with_state(26, 26);
        assert_eq!(r.pos, 0);
        assert_eq!(r.ring_loc, 0);
    }
}
