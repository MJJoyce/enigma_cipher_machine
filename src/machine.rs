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
        T: IntoIterator<Item = &'static str>,
    {
        let mut rotors = Vec::new();
        for rotor_id in rotor_ids {
            rotors.push(Rotor::new(rotor_id));
        }

        self.rotors = Some(rotors);
        self
    }

    pub fn build(self) -> Option<EnigmaMachine> {
        if self.reflector.is_none() || self.rotors.is_none() || self.plugboard.is_none() {
            return None;
        }

        Some(
            EnigmaMachine {
                reflector: self.reflector.unwrap(),
                rotors: self.rotors.unwrap(),
                plugboard: self.plugboard.unwrap()
            }
        )
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
