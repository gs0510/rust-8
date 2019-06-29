struct CPU {
    // pc: u16,         //Program Counter
    v: [u8; 16], //General Purpose 8 bit registers
    // sound_timer: u8, //Sound timer
    // delay_timer: u8, //Delay timer
    // sp: u8,          //Stack Pointer
    // i: u16,          //Address Register
    current_op: u16,
}

const ARITHMATIC_INSTRUCTION: u8 = 0x8;
const ADD_INSTRUCTION: u8 = 0x4;

impl CPU {
    fn run(&mut self) {
        let raw_op = self.current_op;
        let op_major = ((raw_op & 0xF000) >> 12) as u8;
        let reg_x = ((raw_op & 0x0F00) >> 8) as u8;
        let reg_y = ((raw_op & 0x00F0) >> 4) as u8;
        let op_minor = (raw_op & 0x000F) as u8;

        println!("Op_major {}, op_minor {}", op_major, op_minor);

        match (op_major, op_minor) {
            (ARITHMATIC_INSTRUCTION, ADD_INSTRUCTION) => self.add(reg_x, reg_y),
            _ => unimplemented!(),
        }
    }

    fn add(&mut self, x: u8, y: u8) {
        self.v[x as usize] += self.v[y as usize];
    }
}

fn main() {
    let mut cpu = CPU {
        current_op: 0x8014,
        v: [0; 16],
    };
    cpu.v[0] = 5;
    cpu.v[1] = 10;
    cpu.run();
    assert_eq!(cpu.v[0], 15);
    println!("5 + 10 = {}", cpu.v[0]);
}
