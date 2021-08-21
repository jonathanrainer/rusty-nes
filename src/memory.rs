use std::fs;
use std::path::Path;

pub struct Memory {
    raw: Vec<u8>,
}

pub struct MemoryInitialiser {}

impl MemoryInitialiser {
    pub fn new() -> MemoryInitialiser {
        MemoryInitialiser {}
    }

    pub fn initialise_from_text_file(&self, filename: &Path) -> Memory {
        let contents = match fs::read_to_string(filename) {
            Ok(contents) => contents,
            Err(err) => panic!("Could not open source program file: {:?}", err)
        };
        MemoryInitialiser::initialise_from_string(&contents)
    }

    pub fn initialise_from_string(program: &str) -> Memory {
        // Initialise the vector with the contents of said file
        let raw_memory = program.replace("\n", " ").replace(" ", "");
        // Decode text in hex
        let raw_memory = match hex::decode(raw_memory) {
            Ok(raw_memory) => raw_memory,
            Err(err) => panic!("Could not parse file into memory: {:?}", err)
        };
        Memory { raw: raw_memory }
    }

    pub fn initialise_empty() -> Memory {
        Memory { raw: vec![] }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use rstest::rstest;

    use crate::memory::MemoryInitialiser;

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
        let mi = MemoryInitialiser::new();
        let actual_mem = mi.initialise_from_string(input);
        assert_eq!(actual_mem.raw, output)
    }

    #[rstest]
    #[case::existing_file("memory_contents.txt", vec ! [
    0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x12, 0x34, 0x56, 0x78
    ])]
    #[should_panic]
    #[case::nonexistent_file("i_dont_exist.txt", vec ! [])]
    fn test_memory_initialise_file_exists(#[case] filename: &str, #[case] output: Vec<u8>) {
        let mi = MemoryInitialiser::new();
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("resources/test");
        path.push(filename);
        let actual_mem = mi.initialise_from_text_file(path.as_path());
        assert_eq!(actual_mem.raw, output)
    }
}