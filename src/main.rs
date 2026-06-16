// 1 1 1 1 1 1 1 1

fn u8_to_bool_array(number: u8) -> [bool; 8] {
    let mut bool_array = [false; 8];

    for i in 0..8 {
        bool_array[i] = number >> (7 - i) & 1 == 1;
    }

    bool_array
}

struct Chip_8 {
    memory: Memory,
    registers: [Register<u8>; 16],
    address_register: Register<u16>,
    delay_timer: Timer,
    sound_timer: Timer,
    display: Display,
}

struct Memory([u8; 4096]);

#[derive(Copy, Clone)]
struct Register<T>(T);
struct Timer(u8);

struct Screen([[bool; 64]; 32]);

struct Display {
    screen: Screen,
}

struct Coordinate((u16, u16));

impl Chip_8 {
    pub fn new() -> Chip_8 {
        Chip_8 {
            memory: Memory([0; 4096]),
            registers: [Register::<u8>(0); 16],
            address_register: Register::<u16>(0),
            delay_timer: Timer(0),
            sound_timer: Timer(0),
            display: Display::new(),
        }
    }
}

impl Display {
    pub fn new() -> Display {
        Display {
            screen: Screen([[false; 64]; 32]),
        }
    }

    fn set(&mut self, coordinate: Coordinate) {
        let x = (coordinate.0.0 % 64) as usize;
        let y = (coordinate.0.1 % 32) as usize;
        let current_value = self.screen.0[x][y];

        self.screen.0[x][y] = if current_value { false } else { true };
    }

    pub fn draw_row(&mut self, coordinate: Coordinate, pixel_row: u8) {
        let bool_array = u8_to_bool_array(pixel_row);

        let x = coordinate.0.0;
        let mut y = coordinate.0.1;

        for b in bool_array {
            if b {
                self.set(Coordinate((x, y)));
            }
            y += 1;
        }
    }

    pub fn print(&self) {
        for i in 0..32 {
            for j in 0..64 {
                let pixel = self.screen.0[i][j];

                if pixel {
                    print!("▮");
                } else {
                    print!("▯");
                }
            }
            print!("\n");
        }
    }
}

fn main() {
    let mut chip_8 = Chip_8::new();

    chip_8.display.draw_row(Coordinate((0,0)), 0xFF);
    chip_8.display.print();

}
