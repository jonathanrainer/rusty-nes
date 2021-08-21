use crate::memory::Memory;

pub struct Processor {
    pc: u16,
    memory: Memory,
}

impl Processor {
    pub(crate) fn new(mem: Memory) -> Processor {
        Processor { pc: 0, memory: mem }
    }

    pub(crate) fn start(&self) {
        loop {}
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::cpu::Processor;
    use crate::memory::MemoryInitialiser;

    #[rstest]
    fn test_processor_starts_with_pc_at_0() {
        let new_cpu = Processor::new(MemoryInitialiser::initialise_empty());
        assert_eq!(new_cpu.pc, 0)
    }
}