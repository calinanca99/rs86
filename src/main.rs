use std::{fmt::Display, io::Read};

const MOV_OPCODE: u8 = 0b0010_0010;

#[derive(Debug)]
enum Register {
    AL,
    CL,
    DL,
    BL,
    AH,
    CH,
    DH,
    BH,
    AX,
    CX,
    DX,
    BX,
    SP,
    BP,
    SI,
    DI,
}

impl Register {
    fn new(value: u8, is_word: bool) -> Result<Register, String> {
        if value > 7 {
            return Err("Value cannot be higher than 7".to_string());
        }

        let r = match value {
            0 if is_word => Register::AX,
            1 if is_word => Register::CX,
            2 if is_word => Register::DX,
            3 if is_word => Register::BX,
            4 if is_word => Register::SP,
            5 if is_word => Register::BP,
            6 if is_word => Register::SI,
            7 if is_word => Register::DI,
            0 => Register::AL,
            1 => Register::CL,
            2 => Register::DL,
            3 => Register::BL,
            4 => Register::AH,
            5 => Register::CH,
            6 => Register::DH,
            7 => Register::BH,
            _ => unreachable!(),
        };
        Ok(r)
    }
}

impl Display for Register {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Register::AL => write!(f, "al"),
            Register::CL => write!(f, "cl"),
            Register::DL => write!(f, "dl"),
            Register::BL => write!(f, "bl"),
            Register::AH => write!(f, "ah"),
            Register::CH => write!(f, "ch"),
            Register::DH => write!(f, "dh"),
            Register::BH => write!(f, "bh"),
            Register::AX => write!(f, "ax"),
            Register::CX => write!(f, "cx"),
            Register::DX => write!(f, "dx"),
            Register::BX => write!(f, "bx"),
            Register::SP => write!(f, "sp"),
            Register::BP => write!(f, "bp"),
            Register::SI => write!(f, "si"),
            Register::DI => write!(f, "di"),
        }
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    let file_path = if let Some(path) = args.get(1) {
        path
    } else {
        eprintln!("USAGE: rs86 <FILE_PATH>");
        return;
    };

    let mut file = std::fs::File::open(file_path).expect("Cannot open file");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Cannot read from file");

    println!("bits 16\n");

    buf.chunks(2).for_each(|buf| {
        let first_byte = buf[0];
        let second_byte = buf[1];

        // --- Parse the first byte ---
        // Look at the most significant 6 bits to get the OPCODE
        let opcode = first_byte >> 2;
        if opcode != MOV_OPCODE {
            eprintln!("OPCODE not supported");
            return;
        }

        let operates_on_word = (first_byte & 0b0000_0001) == 1;
        let destination_in_reg = (first_byte & 0b0000_0010) >> 1 == 1;

        // --- Parse the second byte ---
        // Look at the most significant 2 bits to get the MOD
        let mode = second_byte >> 6;
        if !(mode) == 3 {
            eprintln!("MOD not supported");
            return;
        };

        let reg = (second_byte & 0b0011_1000) >> 3;
        let rm = second_byte & 0b0000_0111;

        let reg_register = Register::new(reg, operates_on_word).unwrap();
        let rm_register = Register::new(rm, operates_on_word).unwrap();

        let (source, destination) = if destination_in_reg {
            (rm_register, reg_register)
        } else {
            (reg_register, rm_register)
        };

        println!("mov {}, {}", destination, source);
    })
}
