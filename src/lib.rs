//! The **enigma** crate implements an Enigma machine cipher along with the
//! components (i.e., rotors and reflectors) for the Wehrmacht and Luftwaffe
//! versions.
//!
//! # Usage
//!
//! ## CLI
//!
//! The **enigma** binary takes text (encoded or otherwise) and an
//! Enigma machine configuration string as input, runs the text through
//! the configured machine, and outputs the result.
//!
//! ```
//! enigma -i "hello, how are you doing" -c "A;III-A-A,II-A-A,I-A-A;a-b"
//! LZFAD, AMT GPJ FND IFMJY
//! ```
//!
//! Encryption and decyption is handled in the same was due to the nature of
//! Enigma.
//!
//! ```
//! cargo run -- -i "LZFAD, AMT GPJ FND IFMJY" -c "A;III-A-A,II-A-A,I-A-A;a-b"
//! HELLO, HOW ARE YOU DOING
//! ```
//! ## Configuration
//! The **enigma** configuration string specifies all details for the machine.
//!
//! ```
//! "A;III-A-A,II-A-A,I-A-A;a-b"
//! ```
//!
//! This configuration is laid out as through you're looking at an Enigma
//! machine from the front and represents the reflect, rotor, and plugboard
//! configurations.
//!
//! The configuration string is broken up into three components.
//!
//! ```
//! <Reflector Id>;<Rotor Configurations>;<Plugboard mappings>
//! ```
//!
//! The Reflector Id is one of the Wehrmacht and Luftwaffe reflectors. See the
//! [Reflector](reflector::Reflector) page for valid options.
//!
//! A rotor configuration specifies a rotor id and the position and ring location
//! settings. See the [rotor](rotor) module page for valid rotor ids.
//!
//! ```
//! III-A-A
//! <Rotor Id>-<Position>-<Ring Location>
//! ```
//!
//! The plugboard mappings specify the character ports that a plug is connecting.
//! For example, the following connects `A` and `B`  and `C` to `D` on the
//! plugboard. Note that the order doesn't matter.
//!
//! ```
//! <Reflector Id>;<Rotor Configuration>;a-b,d-c
//! ```
//!
//! ## API
//!
//! [EnigmaMachine](machine::EnigmaMachine) and
//! [EnigmaMachineBuilder](machine::EnigmaMachineBuilder) can be used
//! to build and run an Enigma programmatically.
//!
//! ```
//! let builder = EnigmaMachine::builder();
//! let mut machine = builder
//!     .reflector("A")
//!     .plugboard(vec![('A', 'B')])
//!     .rotors(vec![
//!         ("I".to_string(), 0, 0),
//!         ("III".to_string(), 0, 0),
//!         ("II".to_string(), 0, 0),
//!     ])
//!     .build();
//! let trans = em.translate_text("Hello, how are you".chars());
//! ```
//!
//! # Future Improvements
//!
//! Only a portion of the available rotors and reflectors are currently
//! implemented. Specifically, the Beta and Gamma rotors and the thin
//! Kriegsmarine M4 reflectors are not implemented. The implementation
//! of the machine will handle all of these without issue, including
//! the fourth rotor of the Kriegsmarine M4.
//!
//! Some cryptanalysis tooling would be fun to implement. See the
//! Practical Cryptography and Computerphile links for some motivation.
//!
//! # Additional Details
//!
//! There are a lot of amazing resources for studying the internal workings
//! of the Enigma and the incredible work undertaken to break the code. The
//! following contained useful technical details or inspiration when
//! implementing this.
//!
//! - [Technical Details of the Enigma Machine, Cipher Machines and Cryptology](https://www.ciphermachinesandcryptology.com/en/enigmatech.htm)
//! - [Enigma Rotor Details, Wikipedia](https://en.wikipedia.org/wiki/Enigma_rotor_details)
//! - [Quadram Statistics as a Fitness Measure, Practical Cryptography](http://practicalcryptography.com/cryptanalysis/text-characterisation/quadgrams/)
//! - [Cracking Enigma in 2021, Computerphile](https://www.youtube.com/watch?v=RzWB5jL5RX0)

pub mod machine;
pub mod plugboard;
pub mod reflector;
pub mod rotor;
