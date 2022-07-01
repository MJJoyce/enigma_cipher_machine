use std::io::{self, BufRead, BufReader};

use enigma::machine::EnigmaMachine;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
/// Enigma machine encoding and decoding CLI. See --help for more details.
///
/// enigma provides a CLI for encoding / decoding text via an Enigma Machine.
/// See the `enigma` library for details on exactly what is supported.
struct Cli {
    /// Enigma machine configuration string
    ///
    /// This must be of the form:
    ///
    ///     <reflector id>;<rotor ids>;<plugboard mappings>
    ///
    /// <reflector id> must be a valid reflector id. See enigma::reflector for
    /// specifics. Currently one of 'A', 'B', or 'C' is supported.
    ///
    /// <rotor ids> must contain three rotor ids in the below form. See
    /// enigma::rotor for specifics on rotor specifics. Currently one of
    /// 'I', 'II', 'III', 'IV', 'V', 'VI', 'VII', 'VIII' is supported.
    ///
    ///     <rotor spec1>,<rotor spec2>,<rotor spec3>
    ///
    /// Each rotor specification contains 3 pieces of data in a tuple:
    ///
    ///     (rotor id-rotor position-ring position)
    ///
    /// Rotor position should be an alphabetic character and defines the given
    /// rotors starting position. Likewise, ring position should also be an
    /// alphabetic character and defines the rotors ring position setting.
    ///
    /// The rotor id values and reflector id define the rotor structure as if
    /// you were looking at an Enigma Machine from the front. For example:
    ///
    ///     <reflector>;<rotor id1>,<rotor id2>,<rotor id3>   <- input char
    ///
    /// <plugboard mappings> contain character tuples representing plugboard
    /// cable configurations. Plugboard mappings are optional. Mapping example:
    ///
    ///     (char1-char2),(char3-char4), ...
    ///
    /// This would map char1 to char2 (and vice versa) and char3 to char4 (and
    /// vice versa) on the plugboard.
    #[clap(short, long, value_parser)]
    config: String,

    /// Input string for encoding / decoding.
    ///
    /// Defaults to stdin if this option is not provided.
    #[clap(short, long)]
    input: Option<String>,
}

#[derive(Debug)]
struct Config {
    reflector_config: ReflectorConfig,
    rotor_config: RotorConfig,
    plugboard_config: Option<PlugBoardConfig>,
}

impl Config {
    fn parse(config: String) -> Option<Config> {
        let mut cfg_parts: Vec<String> = config.split(';').map(|s| s.to_string()).collect();

        if cfg_parts.len() != 2 && cfg_parts.len() != 3 {
            eprintln!(
                "Invalid number of config components encountered: {} {:?}",
                cfg_parts.len(),
                config
            );
            return None;
        }

        let reflector_config = ReflectorConfig::parse(&mut cfg_parts[0]);
        let rotor_config = RotorConfig::parse(&mut cfg_parts[1]);
        let plugboard_config = if cfg_parts.len() == 3 {
            PlugBoardConfig::parse(&mut cfg_parts[2])
        } else {
            None
        };

        if reflector_config.is_none() || rotor_config.is_none() {
            eprintln!(
                "Missing required config component. Refl: {:?} Rotor: {:?}",
                reflector_config, rotor_config
            );
            return None;
        }

        Some(Config {
            reflector_config: reflector_config.unwrap(),
            rotor_config: rotor_config.unwrap(),
            plugboard_config,
        })
    }
}

#[derive(Debug)]
struct ReflectorConfig {
    reflector_id: String,
}

impl ReflectorConfig {
    fn parse(config: &mut String) -> Option<ReflectorConfig> {
        config.retain(|c| !c.is_whitespace());

        // TODO: This should be something supported in the library
        if config == "A" || config == "B" || config == "C" {
            Some(ReflectorConfig {
                reflector_id: config.clone(),
            })
        } else {
            eprintln!("Invalid reflector id: {}", config);
            None
        }
    }
}

#[derive(Debug)]
struct RotorConfig {
    rotor_ids: Vec<(String, u8, u8)>,
}

impl RotorConfig {
    fn parse(config: &mut String) -> Option<RotorConfig> {
        config.retain(|c| !c.is_whitespace() && c != '(' && c != ')');

        //TODO: This should be something supported in the library

        let rotor_cfgs: Vec<String> = config.split(',').map(|s| s.to_string()).rev().collect();

        if rotor_cfgs.len() != 3 && rotor_cfgs.len() != 4 {
            eprintln!("Invalid rotor config count: {:?}", rotor_cfgs);
            return None;
        }

        let mut rotor_ids: Vec<(String, u8, u8)> = Vec::new();
        for rotor_cfg in rotor_cfgs {
            let cfg_elems: Vec<&str> = rotor_cfg.split('-').collect();
            if cfg_elems.len() != 3 {
                eprintln!("Invalid rotor config: {}", rotor_cfg);
                return None;
            }

            if cfg_elems[0] != "I"
                && cfg_elems[0] != "II"
                && cfg_elems[0] != "III"
                && cfg_elems[0] != "IV"
                && cfg_elems[0] != "V"
                && cfg_elems[0] != "VI"
                && cfg_elems[0] != "VII"
                && cfg_elems[0] != "VIII"
            {
                eprintln!("Invalid rotor id: {}", cfg_elems[0]);
                return None;
            }

            let pos: char = cfg_elems[1].to_ascii_uppercase().chars().next().unwrap();
            if !pos.is_ascii_alphabetic() {
                eprintln!("Invalid rotor position: {}", pos);
                return None;
            }

            let ring_loc: char = cfg_elems[1].to_ascii_uppercase().chars().next().unwrap();
            if !ring_loc.is_ascii_alphabetic() {
                eprintln!("Invalid rotor ring location: {}", ring_loc);
                return None;
            }

            rotor_ids.push((
                cfg_elems[0].to_string(),
                pos as u8 - b'A',
                ring_loc as u8 - b'A',
            ))
        }

        Some(RotorConfig { rotor_ids })
    }
}

#[derive(Debug)]
struct PlugBoardConfig {
    plugboard_maps: Vec<(char, char)>,
}

impl PlugBoardConfig {
    fn parse(config: &mut String) -> Option<PlugBoardConfig> {
        config.retain(|c| !c.is_whitespace() && c != '(' && c != ')');

        // TODO: This should be something supported in the library
        let pb_maps: Vec<String> = config.split(',').map(|s| s.to_string()).collect();

        let mut plugboard_maps: Vec<(char, char)> = Vec::new();
        for map in pb_maps {
            let chs: Vec<char> = map.to_ascii_uppercase().chars().collect();
            if chs.len() != 3 {
                eprintln!("Invalid plugboard config received: {}", map);
                return None;
            }

            if !chs[0].is_ascii_alphabetic() || !chs[2].is_ascii_alphabetic() {
                eprintln!("Plugboard config contains non alphabetic chars: {}", map);
                return None;
            }

            plugboard_maps.push((chs[0], chs[2]));
        }

        Some(PlugBoardConfig { plugboard_maps })
    }
}

fn main() {
    let cli = Cli::parse();
    let cfg = Config::parse(cli.config).expect("Invalid enigma config provided");

    let input: Box<dyn BufRead> = if cli.input.is_none() {
        Box::new(BufReader::new(io::stdin().lock()))
    } else {
        Box::new(io::Cursor::new(cli.input.unwrap()))
    };

    let builder = EnigmaMachine::builder();
    let builder = builder
        .reflector(&cfg.reflector_config.reflector_id)
        .rotors(cfg.rotor_config.rotor_ids);

    // TODO: This is a friggen hideous way to need to handle this ...
    let builder = match cfg.plugboard_config {
        Some(pbcfg) => builder.plugboard(pbcfg.plugboard_maps),
        None => builder,
    };

    let mut em = builder.build().unwrap();

    for l in input.lines() {
        match l {
            Ok(in_line) => println!("{}", em.translate_text(in_line.chars())),
            Err(_) => println!("Done"),
        }
    }
}
