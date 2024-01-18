use cpu::CPU;

mod cpu;

fn main() {
    let mut cpu = CPU::new();
    let data = vec![0];
    cpu.interpret(&data);
}
