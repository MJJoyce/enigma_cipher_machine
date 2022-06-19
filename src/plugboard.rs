use std::collections::HashMap;

use regex::Regex;

#[derive(Debug)]
pub struct PlugBoard {
    mapping: HashMap<u8, u8>,
}

impl PlugBoard {
    pub fn new() -> PlugBoard {
        PlugBoard {
            mapping: HashMap::new(),
        }
    }

    pub fn new_with_mapping<T>(mappings: T) -> Result<PlugBoard, String>
    where
        T: IntoIterator<Item = (char, char)>,
    {
        let mut pb = Self::new();

        for (in1, in2) in mappings {
            if let Err(msg) = pb.add_mapping(in1, in2) {
                return Err(msg);
            }
        }

        Ok(pb)
    }

    pub fn add_mapping(&mut self, in1: char, in2: char) -> Result<(), String> {
        let valid_char = Regex::new("^[a-zA-Z]$").unwrap();
        let mut conv_buf = [0; 4];

        if !(valid_char.is_match(in1.encode_utf8(&mut conv_buf))
            && valid_char.is_match(in2.encode_utf8(&mut conv_buf)))
        {
            let msg = format!(
                "Error in PlugBoard configuration. Can only map alphabetic characters. Received {} {}",
                in1, in2
            );
            eprintln!("{}", msg);
            return Err(msg);
        }

        let in1_val = in1.to_ascii_uppercase() as u8 - 65;
        let in2_val = in2.to_ascii_uppercase() as u8 - 65;

        if self.mapping.contains_key(&in1_val) {
            let msg = format!(
                "Error in PlugBoard configuration. Duplicate mapping for {} encountered",
                in1_val
            );
            eprintln!("{}", msg);
            return Err(msg);
        }

        if self.mapping.contains_key(&in2_val) {
            let msg = format!(
                "Error in plugboard configuration. Duplicate mapping for {} encountered",
                in2_val
            );
            eprintln!("{}", msg);
            return Err(msg);
        }

        self.mapping.insert(in1_val, in2_val);
        self.mapping.insert(in2_val, in1_val);
        Ok(())
    }

    pub fn map(&self, in_val: u8) -> u8 {
        if let Some(&v) = self.mapping.get(&in_val) {
            v
        } else {
            in_val
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_mapping_basic() {
        let mut pb = PlugBoard::new();

        let res = pb.add_mapping('A', 'B');

        assert!(res.is_ok());
        assert!(pb.mapping.contains_key(&('A' as u8 - 65)));
        assert!(pb.mapping.contains_key(&('B' as u8 - 65)));
    }

    #[test]
    fn add_mapping_bad_input() {
        let mut pb = PlugBoard::new();

        let res = pb.add_mapping('a', 'B');

        assert!(res.is_ok());
        assert!(pb.mapping.contains_key(&('A' as u8 - 65)));
        assert!(pb.mapping.contains_key(&('B' as u8 - 65)));
    }

    #[test]
    fn add_mapping_invalid_input() {
        let mut pb = PlugBoard::new();

        let err = pb.add_mapping('2', 'B');
        assert!(err.is_err());

        let err = pb.add_mapping('A', ',');
        assert!(err.is_err());
    }

    #[test]
    fn plugboard_new_with_mappings() {
        let pb = PlugBoard::new_with_mapping(vec![('A', 'B'), ('C', 'D')]).unwrap();
        assert!(pb.mapping.contains_key(&('A' as u8 - 65)));
        assert!(pb.mapping.contains_key(&('B' as u8 - 65)));
        assert!(pb.mapping.contains_key(&('C' as u8 - 65)));
        assert!(pb.mapping.contains_key(&('D' as u8 - 65)));
    }

    fn plugboard_new_with_mappings_percolates_errors() {
        let pb = PlugBoard::new_with_mapping(vec![('A', ','), ('C', 'D')]);

        assert!(pb.is_err());
    }
}
