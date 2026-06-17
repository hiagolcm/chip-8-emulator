// 1 1 1 1 1 1 1 1

const PRELOADED_SPRITES: [[u8; 5]; 16] = [
    [0xF0, 0x90, 0x90, 0x90, 0xF0], // 0
    [0x20, 0x60, 0x20, 0x20, 0x70], // 1
    [0xF0, 0x10, 0xF0, 0x80, 0xF0], // 2
    [0xF0, 0x10, 0xF0, 0x10, 0xF0], // 3
    [0x90, 0x90, 0xF0, 0x10, 0x10], // 4
    [0xF0, 0x80, 0xF0, 0x10, 0xF0], // 5
    [0xF0, 0x80, 0xF0, 0x90, 0xF0], // 6
    [0xF0, 0x10, 0x20, 0x40, 0x40], // 7
    [0xF0, 0x90, 0xF0, 0x90, 0xF0], // 8
    [0xF0, 0x90, 0xF0, 0x10, 0xF0], // 9
    [0xF0, 0x90, 0xF0, 0x90, 0x90], // A
    [0xE0, 0x90, 0xE0, 0x90, 0xE0], // B
    [0xF0, 0x80, 0x80, 0x80, 0xF0], // C
    [0xE0, 0x90, 0x90, 0x90, 0xE0], // D
    [0xF0, 0x80, 0xF0, 0x80, 0xF0], // E
    [0xF0, 0x80, 0xF0, 0x80, 0x80], // F
];

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

impl Chip_8 {
    pub fn new() -> Chip_8 {
        Chip_8 {
            memory: Memory::new(),
            registers: [Register::<u8>(0); 16],
            address_register: Register::<u16>(0),
            delay_timer: Timer(0),
            sound_timer: Timer(0),
            display: Display::new(),
        }
    }
}

struct Memory([u8; 4096]);

impl Memory {
    fn new() -> Memory {
        let mut memory = Memory([0; 4096]);
        let mut address = 0;

        for s in PRELOADED_SPRITES {
            for byte in s {
                memory.set(address, byte);
                address += 1;
            }
        }

        memory
    }

    pub fn set(&mut self, address: usize, val: u8) {
        self.0[address] = val;
    }

    pub fn get(&self, address: u16) -> u8 {
        self.0[address as usize]
    }
}

#[derive(Copy, Clone)]
struct Register<T>(T);

impl<T> Register<T> {
    pub fn set(&mut self, val: T) {
        self.0 = val;
    }

    pub fn get(self) -> T {
        self.0
    }
}

struct Timer(u8);

impl Timer {
    pub fn set(&mut self, val: u8) {
        self.0 = val;
    }

    pub fn get(self) -> u8 {
        self.0
    }
}

struct Screen([[bool; 64]; 32]);

struct Coordinate((u8, u8));

struct Display {
    screen: Screen,
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
                    print!("▮ ");
                } else {
                    print!("▯ ");
                }
            }
            print!("\n");
        }
    }
}

trait Chip8Interpreter {
    // FX29
    fn store_i_with_sprite_in_vx(&mut self, register_id: usize);

    // 6xNN
    fn store_nn_to_vx(&mut self, number: u8, register: u8);

    // 7XNN
    fn add_nn_to_vx(&mut self, number: u8, register: u8);

    // 8XY0	Store the value of register VY in register VX
    fn store_vy_to_vx(&mut self, register_x: u8, register_y: u8);

    // 8XY1	Set VX to VX OR VY
    fn set_vx_to_vx_or_vy(&mut self, register_x: u8, register_y: u8);
    
    // 8XY2	Set VX to VX AND VY
    fn set_vx_to_vx_and_vy(&mut self, register_x: u8, register_y: u8);

    // 8XY3	Set VX to VX XOR VY
    fn set_vx_to_vx_xor_vy(&mut self, register_x: u8, register_y: u8);

    // 8XY4 vx += vy vf = 1 on carry
    fn set_vx_to_vx_plus_vy(&mut self, register_x: u8, register_y: u8);

    // DXYN
    fn draw(&mut self, bytes: u8, register_x: u8, register_y: u8);
}

impl Chip8Interpreter for Chip_8 {
    // FX29
    fn store_i_with_sprite_in_vx(&mut self, register_id: usize) {
        let hex_sprite = self.registers[register_id].0;
        let address = (hex_sprite * 5) as u16;
        self.address_register.set(address);
    }

    // 6XNN
    fn store_nn_to_vx(&mut self, val: u8, register: u8) {
        self.registers[register as usize].set(val);
    }

    // 7XNN
    fn add_nn_to_vx(&mut self, val: u8, register: u8) {
        let current_value = self.registers[register as usize].0;
        self.registers[register as usize].set(current_value + val);
    }

    // 8XY0	Store the value of register VY in register VX
    fn store_vy_to_vx(&mut self, register_x: u8, register_y: u8) {
        let y = self.registers[register_y as usize].get();
        self.registers[register_x as usize].set(y);
    }

    // 8XY1	Set VX to VX OR VY
    fn set_vx_to_vx_or_vy(&mut self, register_x: u8, register_y: u8) {
        let x = self.registers[register_x as usize].get();
        let y = self.registers[register_y as usize].get();
        
        let val = x | y;

        self.registers[register_x as usize].set(val);
    }

    // 8XY2	Set VX to VX AND VY
    fn set_vx_to_vx_and_vy(&mut self, register_x: u8, register_y: u8) {
        let x = self.registers[register_x as usize].get();
        let y = self.registers[register_y as usize].get();
        
        let val = x & y;

        self.registers[register_x as usize].set(val);
    }

    // 8XY3	Set VX to VX XOR VY
    fn set_vx_to_vx_xor_vy(&mut self, register_x: u8, register_y: u8) {
        let x = self.registers[register_x as usize].get();
        let y = self.registers[register_y as usize].get();
        
        let val = x ^ y;

        self.registers[register_x as usize].set(val);
    }

    // 8XY4 vx += vy vf = 1 on carry
    fn set_vx_to_vx_plus_vy(&mut self, register_x: u8, register_y: u8) {
        let x = self.registers[register_x as usize].get();
        let y = self.registers[register_y as usize].get();

        let sum: u16 = x as u16 + y as u16;

        if sum > 255 {
            self.registers[0xF].set(1);
        } else  {
            self.registers[0xF].set(0);
        }

        self.registers[register_x as usize].set(sum as u8);
    }

    // DXYN
    fn draw(&mut self, bytes: u8, register_x: u8, register_y: u8) {
        let x = self.registers[register_x as usize].0;
        let y = self.registers[register_y as usize].0;
        let mut sprite_segment_address = self.address_register.0;

        for i in 0..bytes {
            let sprite_segment = self.memory.get(sprite_segment_address);
            self.display
                .draw_row(Coordinate((x + i, y)), sprite_segment);
            sprite_segment_address += 1;
        }
    }
}

fn main() {
    let mut chip_8 = Chip_8::new();

    chip_8.store_nn_to_vx(0, 0);
    chip_8.store_nn_to_vx(0, 1);
    chip_8.store_nn_to_vx(0xF, 2);
    chip_8.store_i_with_sprite_in_vx(2);
    chip_8.draw(5, 0, 1);


    chip_8.store_nn_to_vx(0, 0);
    chip_8.store_nn_to_vx(5, 1);
    chip_8.store_nn_to_vx(0x0, 2);
    chip_8.store_i_with_sprite_in_vx(2);
    chip_8.draw(5, 0, 1);


    chip_8.store_nn_to_vx(0, 0);
    chip_8.store_nn_to_vx(10, 1);
    chip_8.store_nn_to_vx(0xD, 2);
    chip_8.store_i_with_sprite_in_vx(2);
    chip_8.draw(5, 0, 1);

    chip_8.store_nn_to_vx(0, 0);
    chip_8.store_nn_to_vx(15, 1);
    chip_8.store_nn_to_vx(0xA, 2);
    chip_8.store_i_with_sprite_in_vx(2);
    chip_8.draw(5, 0, 1);


    chip_8.display.print();
}
