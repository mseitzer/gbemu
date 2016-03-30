use cpu::Cpu;
use cpu::debug::DebugInfo;
use hardware::Bus;
use int_controller::Interrupt;
use instructions::Instr;

mod test_load;

struct TestHardware {
    memory: Vec<u8>
}

impl Bus for TestHardware {
    fn read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.memory[addr as usize] = value;
    }

    fn has_irq(&self) -> bool { false }
    fn ack_irq(&mut self) -> Option<Interrupt> { None }
    fn update(&mut self, cycles: u8) {}
}

fn create_hardware(memory: Vec<u8>) -> TestHardware {
    TestHardware {
        memory: memory
    }
}

fn run_test<F: Fn(&mut Cpu<TestHardware>)->()> (code: &[u8], init: F) 
    -> Cpu<TestHardware> {
    let mut cpu = Cpu::new(create_hardware(code.to_vec()));
    init(&mut cpu);

    let mut db = DebugInfo::new();
    cpu.single_step(&mut db);

    return cpu;
}

fn test_instr<F: Fn(&mut Cpu<TestHardware>)->()> (instr: Instr, memory: &[u8], init: F) 
    -> Cpu<TestHardware> {
    let mut cpu = Cpu::new(create_hardware(memory.to_vec()));
    init(&mut cpu);

    cpu.execute_instr(instr);

    return cpu;
}

#[test]
fn test_00() {
    let cpu = run_test(
        &[0x00], // NOP
        |_| {}
    );

    assert_eq!(cpu.total_cycles, 1);
}