struct Chip_8 {
    memory: Memory,
    registers: [Register<u8>; 16],
    address_register: Register<u16>,
    delay_timer: Timer,
    sound_timer: Timer,
}

struct Memory([u8; 4096]);
struct Register<T>(T);
struct Timer(u8);

struct Screen([[bool; 64]; 32]);

fn main() {
    println!("Hello, world!");
}
