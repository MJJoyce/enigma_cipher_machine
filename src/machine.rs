use lazy_static::lazy_static;
use regex::Regex;

use crate::plugboard::PlugBoard;
use crate::reflector::Reflector;
use crate::rotor::Rotor;

lazy_static! {
    static ref VALID_CHAR: Regex = Regex::new("^[a-zA-Z]$").unwrap();
}

#[derive(Debug, Default)]
pub struct EnigmaMachineBuilder {
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

#[derive(Debug, Default)]
pub struct EnigmaMachine {
    reflector: Reflector,
    rotors: Vec<Rotor>,
    plugboard: PlugBoard,
}

impl EnigmaMachine {
    pub fn new() -> EnigmaMachine {
        EnigmaMachine {
            reflector: Reflector::new("A"),
            rotors: vec![Rotor::new("I"), Rotor::new("II"), Rotor::new("III")],
            plugboard: PlugBoard::new(),
        }
    }

    pub fn builder() -> EnigmaMachineBuilder {
        EnigmaMachineBuilder::new()
    }

    pub fn translate(&mut self, input: char) -> char {
        let mut conv_buf = [0; 4];

        if !(VALID_CHAR.is_match(input.encode_utf8(&mut conv_buf))) {
            return input;
        }

        let mut trans_input = input.to_ascii_uppercase() as u8 - 65;
        trans_input = self.plugboard.map(trans_input);
        let mut rotation_triggered = false;

        for (i, rotor) in self.rotors.iter_mut().enumerate() {
            if i == 0 || rotor.will_step_next_rotor() || rotation_triggered {
                rotation_triggered = rotor.will_step_next_rotor();
                rotor.rotate();
            }

            trans_input = rotor.map_in(trans_input);
        }

        trans_input = self.reflector.map(trans_input);

        for rotor in (&self.rotors).iter().rev() {
            trans_input = rotor.map_out(trans_input);
        }

        trans_input = self.plugboard.map(trans_input);

        (trans_input + 65).into()
    }
}

#[cfg(test)]
mod builder_tests {
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

#[cfg(test)]
mod machine_tests {
    use super::*;

    #[test]
    fn simple_translation_test() {
        let builder = EnigmaMachine::builder();
        let mut em = builder
            .reflector("B")
            .plugboard(vec![('A', 'B')])
            .rotors(vec![("I", 0, 0), ("II", 0, 0), ("III", 0, 0)])
            .build()
            .unwrap();

        // Expected translation values pulled from
        // http://people.physik.hu-berlin.de/~palloks/js/enigma/enigma-u_v25_en.html
        // WUPGNWOJUSQGTULMNUNYRHZSHP
        assert_eq!(em.translate('A'), 'W');
        assert_eq!(em.translate('A'), 'U');
        assert_eq!(em.translate('A'), 'P');
        assert_eq!(em.translate('A'), 'G');
        assert_eq!(em.translate('A'), 'N');
        assert_eq!(em.translate('A'), 'W');
        assert_eq!(em.translate('A'), 'O');
        assert_eq!(em.translate('A'), 'J');
        assert_eq!(em.translate('A'), 'U');
        assert_eq!(em.translate('A'), 'S');
        assert_eq!(em.translate('A'), 'Q');
        assert_eq!(em.translate('A'), 'G');
        assert_eq!(em.translate('A'), 'T');
        assert_eq!(em.translate('A'), 'U');
        assert_eq!(em.translate('A'), 'L');
        assert_eq!(em.translate('A'), 'M');
        assert_eq!(em.translate('A'), 'N');
        assert_eq!(em.translate('A'), 'U');
        assert_eq!(em.translate('A'), 'N');
        assert_eq!(em.translate('A'), 'Y');
        assert_eq!(em.translate('A'), 'R');
        assert_eq!(em.translate('A'), 'H');
        assert_eq!(em.translate('A'), 'Z');
        assert_eq!(em.translate('A'), 'S');
        assert_eq!(em.translate('A'), 'H');
        assert_eq!(em.translate('A'), 'P');
    }

    #[test]
    fn test_that_invalid_chars_are_returned_unmodified() {
        let builder = EnigmaMachine::builder();
        let mut em = builder
            .reflector("B")
            .plugboard(vec![('A', 'B')])
            .rotors(vec![("I", 0, 0), ("II", 0, 0), ("III", 0, 0)])
            .build()
            .unwrap();

        assert_eq!(em.translate('A'), 'W');
        assert_eq!(em.translate('.'), '.');
        assert_eq!(em.translate('✅'), '✅');
    }

    #[test]
    fn test_triggering_second_rotor_rotation() {
        let builder = EnigmaMachine::builder();
        let mut em = builder
            .reflector("B")
            .plugboard(vec![('A', 'B')])
            .rotors(vec![("I", 0, 0), ("II", 0, 0), ("III", 0, 0)])
            .build()
            .unwrap();

        let input = "Loremipsumdolorsi";
        let expected_output = "ilfdfbruadonvisru";

        for (in_val, out_val) in input.chars().zip(expected_output.chars()) {
            let trans = em.translate(in_val);
            let out_trans = out_val.to_ascii_uppercase();
            assert_eq!(trans, out_trans);
        }
    }

    #[test]
    fn test_triggering_third_rotor_rotation() {
        let builder = EnigmaMachine::builder();
        let mut em = builder
            .reflector("B")
            .plugboard(vec![('A', 'B')])
            .rotors(vec![("I", 0, 0), ("II", 0, 0), ("III", 0, 0)])
            .build()
            .unwrap();

        let input =
            "Loremipsumdolorsitametconsecteturadipiscingelitseddoeiusmodtemporincididuntutl\
             aboreetdoloremagna";
        let expected_output =
            "ilfdfbruadonvisruknzqmndiycouhrlbiarmpylbznyngrmrmvbbjlnszfhsyaakfod\
             pchqphswoqrwjfkxnabzjnpzozau";

        for (in_val, out_val) in input.chars().zip(expected_output.chars()) {
            let trans = em.translate(in_val);
            let out_trans = out_val.to_ascii_uppercase();
            assert_eq!(trans, out_trans);
        }
    }

    #[test]
    fn test_large_translation() {
        let builder = EnigmaMachine::builder();
        let mut em = builder
            .reflector("B")
            .plugboard(vec![('A', 'B')])
            .rotors(vec![("I", 0, 0), ("II", 0, 0), ("III", 0, 0)])
            .build()
            .unwrap();

        let input =
            "Loremipsumdolorsitametconsecteturadipiscingelitseddoeiusmodtemporincididuntutl\
             aboreetdoloremagnaaliquaUtenimadminimveniamquisnostrudexercitationullamcolabor\
             isnisiutaliquipexeacommodoconsequatDuisauteiruredolorinreprehenderitinvoluptat\
             evelitessecillumdoloreeufugiatnullapariaturExcepteursintoccaecatcupidatatnonpr\
             oidentsuntinculpaquiofficiadeseruntmollitanimidestlaborum";

        let expected_output =
            "ilfdfbruadonvisruknzqmndiycouhrlbiarmpylbznyngrmrmvbbjlnszfhsyaakfod\
             pchqphswoqrwjfkxnabzjnpzozauwmnxoupxxevjuisapsdpjqawzxcthivuojjcsova\
             swfclgslfdxgwzwsnsbyrjbszsgqmmqrlkbulwtznghsrkvxfqlevddwhbwgaxvfaisb\
             bqcnnfyjxabwiykwceuocskmkheqfbjdonnhvnjtbdkkyrycyittbgpduzjkesmivdph\
             ojqoejioxfmkslnakczgbummcrsfvsimqkjgmcnemlsouhzbjxdraoxvmlmnzuqbmqhn\
             cfcokgesbvpjzsshnzaleejkapxrv";

        for (in_val, out_val) in input.chars().zip(expected_output.chars()) {
            let trans = em.translate(in_val);
            let out_trans = out_val.to_ascii_uppercase();
            assert_eq!(trans, out_trans);
        }
    }
}
