use crate::plugboard::PlugBoard;
use crate::reflector::Reflector;
use crate::rotor::Rotor;

#[derive(Debug)]
struct EnigmaMachineBuilder {
    reflector: Option<Reflector>,
    rotors: Option<Vec<Rotor>>,
    plugboard: Option<PlugBoard>,
}

impl EnigmaMachineBuilder {
    pub fn new() -> EnigmaMachineBuilder {
        EnigmaMachineBuilder {
            reflector: None,
            rotors: None,
            plugboard: None,
        }
    }

    pub fn reflector(mut self, id: &str) -> EnigmaMachineBuilder {
        let refl = Reflector::new(id);
        self.reflector = Some(refl);
        self
    }

    pub fn plugboard<T>(mut self, mappings: T) -> EnigmaMachineBuilder
    where
        T: IntoIterator<Item = (char, char)>,
    {
        if let Ok(pb) = PlugBoard::new_with_mapping(mappings) {
            self.plugboard = Some(pb);
        } else {
            eprintln!("Invalid mappings provided for plugboard");
        }

        self
    }

    pub fn rotors<T>(mut self, rotor_ids: T) -> EnigmaMachineBuilder
    where
        T: IntoIterator<Item = (&'static str, u8, u8)>,
    {
        let mut rtrs = Vec::new();
        for (rotor_id, pos, ring_loc) in rotor_ids {
            rtrs.push(Rotor::new_with_state(rotor_id, pos, ring_loc));
        }

        if let Some(ref mut rotors) = self.rotors {
            rotors.append(&mut rtrs);
        } else {
            self.rotors = Some(rtrs);
        }
        self
    }

    pub fn rotor(mut self, rotor_id: &str, pos: u8, ring_loc: u8) -> EnigmaMachineBuilder {
        let rotor = Rotor::new_with_state(rotor_id, pos, ring_loc);

        if let Some(ref mut rotors) = self.rotors {
            rotors.push(rotor);
        } else {
            self.rotors = Some(vec![rotor]);
        }

        self
    }

    pub fn build(self) -> Option<EnigmaMachine> {
        if self.reflector.is_none() || self.rotors.is_none() || self.plugboard.is_none() {
            return None;
        }

        Some(EnigmaMachine {
            reflector: self.reflector.unwrap(),
            rotors: self.rotors.unwrap(),
            plugboard: self.plugboard.unwrap(),
        })
    }
}

#[derive(Debug)]
struct EnigmaMachine {
    reflector: Reflector,
    rotors: Vec<Rotor>,
    plugboard: PlugBoard,
}

impl EnigmaMachine {
    fn new() -> EnigmaMachine {
        EnigmaMachine {
            reflector: Reflector::new("A"),
            rotors: vec![Rotor::new("I"), Rotor::new("II"), Rotor::new("III")],
            plugboard: PlugBoard::new(),
        }
    }

    fn builder() -> EnigmaMachineBuilder {
        EnigmaMachineBuilder::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_builder() {
        let builder = EnigmaMachine::builder();
        let em = builder
            .reflector("A")
            .plugboard(vec![('A', 'B')])
            .rotors(vec![("I", 0, 0), ("III", 0, 0), ("II", 0, 0)])
            .build();
    }

    #[test]
    fn test_builder_missing_item() {
        let builder = EnigmaMachine::builder();
        let em = builder
            .plugboard(vec![('A', 'B')])
            .rotors(vec![("I", 0, 0), ("III", 0, 0), ("II", 0, 0)])
            .build();
        assert!(em.is_none());

        let builder = EnigmaMachine::builder();
        let em = builder
            .reflector("A")
            .rotors(vec![("I", 0, 0), ("III", 0, 0), ("II", 0, 0)])
            .build();
        assert!(em.is_none());

        let builder = EnigmaMachine::builder();
        let em = builder.reflector("A").plugboard(vec![('A', 'B')]).build();
        assert!(em.is_none());
    }

    #[test]
    fn test_builder_rotor_handling() {
        let builder = EnigmaMachine::builder();
        let em = builder
            .rotors(vec![("I", 0, 0), ("III", 0, 0), ("II", 0, 0)])
            .rotor("IV", 0, 0);
        assert_eq!(em.rotors.unwrap().len(), 4);

        let builder = EnigmaMachine::builder();
        let em = builder
            .rotor("IV", 0, 0)
            .rotors(vec![("I", 0, 0), ("III", 0, 0), ("II", 0, 0)]);
        assert_eq!(em.rotors.unwrap().len(), 4);
    }
}
