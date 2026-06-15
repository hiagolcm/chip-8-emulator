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

struct Display {
    screen: Screen
}

struct Coordinate((u16, u16));

impl Display {
    fn set(&mut self, coordinate: Coordinate) {
        let x = (coordinate.0.0 % 64 ) as usize;
        let y = (coordinate.0.1  % 32 ) as usize;
        let current_value = self.screen.0[x][y];

        self.screen.0[x][y] = if current_value { false } else { true };
    }
}

fn main() {
    println!("Hello, world!");
}
