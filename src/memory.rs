use std::fs;
use std::path::Path;

use log::warn;

pub struct Memory {
    raw: Vec<u8>,
}

impl Memory {
    pub fn read(&self, address: u16) -> u8 {
        if (address % 4) != 0 {
            warn!("Misaligned Address Requested: {}, returning {}", address, address / 4);
        }
        self.raw[usize::from(address / 4)]
    }

    pub fn new() -> Memory {
        Memory { raw: vec![] }
    }

    pub fn new_from_text_file(filename: &Path) -> Memory {
        let contents = match fs::read_to_string(filename) {
            Ok(contents) => contents,
            Err(err) => panic!("Could not open source program file: {:?}", err)
        };
        Memory::new_from_string(&contents)
    }

    pub fn new_from_string(program: &str) -> Memory {
        let raw_memory = program.replace("\n", " ").replace(" ", "");
        // Decode text in hex
        let raw_memory = match hex::decode(raw_memory) {
            Ok(raw_memory) => raw_memory,
            Err(err) => panic!("Could not parse file into memory: {:?}", err)
        };
        Memory { raw: raw_memory }
    }

    pub fn new_from_binary_file(filename: &Path, start_address: usize) -> Memory {
        let mut binary_contents = match fs::read(filename) {
            Ok(contents) => contents,
            Err(err) => panic!("Could not open binary file: {:?}", err)
        };
        let mut raw = vec![0x0; start_address / 4];
        raw.append(&mut binary_contents);
        Memory { raw }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rstest::rstest;

    use crate::memory::Memory;

    #[rstest]
    #[case::empty_string("", vec ! [])]
    #[case::simple_string("DEADBEEF", vec ! [0xde, 0xad, 0xbe, 0xef])]
    #[case::simple_string_with_linebreak("DEAD\nBEEF", vec ! [0xde, 0xad, 0xbe, 0xef])]
    #[case::simple_string_with_linebreak_and_spaces("DE AD\nBE EF", vec ! [0xde, 0xad, 0xbe, 0xef])]
    #[case::simple_string_with_linebreaks("DE\nAD\nBE\nEF", vec ! [0xde, 0xad, 0xbe, 0xef])]
    #[should_panic]
    #[case::odd_length("D", vec ! [])]
    #[should_panic]
    #[case::odd_length_valid_characters_prior("DEADB", vec ! [])]
    fn test_memory_initialise_string(#[case] input: &str, #[case] output: Vec<u8>) {
        let actual_mem = Memory::new_from_string(input);
        assert_eq!(actual_mem.raw, output)
    }

    #[rstest]
    #[case::existing_file("memory_contents.txt", vec ! [
    0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x12, 0x34, 0x56, 0x78
    ])]
    #[should_panic]
    #[case::nonexistent_file("i_dont_exist.txt", vec ! [])]
    fn test_memory_initialise_file_exists(#[case] filename: &str, #[case] output: Vec<u8>) {
        let actual_mem = Memory::new_from_text_file(get_resource_file(filename).as_path());
        assert_eq!(actual_mem.raw, output)
    }


    #[rstest]
    #[case::existing_file_start_at_0("small.nes", 0, vec ! [0xDE, 0xAD, 0xBE, 0xEF])]
    #[case::existing_file_start_at_0x8("small.nes", 8, vec ! [0x00, 0x00, 0xDE, 0xAD, 0xBE, 0xEF])]
    fn test_memory_initialise_binary_file(#[case] filename: &str, #[case] start_address: usize,
                                          #[case] output: Vec<u8>)
    {
        let actual_mem = Memory::new_from_binary_file(
            get_resource_file(filename).as_path(), start_address,
        );
        assert_eq!(actual_mem.raw, output)
    }

    #[rstest]
    #[case::read_from_aligned_address("DEADBEEF", 4, 0xAD)]
    #[case::read_from_misaligned_address("DEADBEEF", 5, 0xAD)]
    fn test_memory_read(#[case] input_string: &str, #[case] memory_location: u16,
                        #[case] result: u8)
    {
        let mem = Memory::new_from_string(input_string);
        assert_eq!(mem.read(memory_location), result);
    }

    fn get_resource_file(filename: &str) -> PathBuf {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("tests/resources");
        path.push(filename);
        path
    }
}