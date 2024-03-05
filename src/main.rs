use std::io::Read;

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

        let r = if is_word {
            match value {
                0 => Register::AX,
                1 => Register::CX,
                2 => Register::DX,
                3 => Register::BX,
                4 => Register::SP,
                5 => Register::BP,
                6 => Register::SI,
                7 => Register::DI,
                _ => unreachable!(),
            }
        } else {
            match value {
                0 => Register::AL,
                1 => Register::CL,
                2 => Register::DL,
                3 => Register::BL,
                4 => Register::AH,
                5 => Register::CH,
                6 => Register::DH,
                7 => Register::BH,
                _ => unreachable!(),
            }
        };
        Ok(r)
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
    let mut buf = [0_u8; 2];
    file.read_exact(&mut buf).expect("Cannot read from file");

    let first_byte = buf[0];
    let second_byte = buf[1];

    println!("Buf: {:?}", buf);
    println!("First byte: {:#b}", first_byte);
    println!("Second byte: {:#b}", second_byte);

    // Look at the most significant 6 bits to get the OPCODE
    let opcode = first_byte >> 2;
    if opcode != MOV_OPCODE {
        eprintln!("OPCODE not supported");
        return;
    }

    // Look at the most significant 2 bits to get the addressing mode
    if !(second_byte >> 6) == 3 {
        eprintln!("MOD not supported");
        return;
    };

    let operates_on_word = (first_byte & 0b0000_0001) == 1;
    let destination_in_reg = (first_byte & 0b0000_0010) >> 1 == 1;

    let reg = (second_byte & 0b0011_1000) >> 3;
    let rm = second_byte & 0b0000_0111;

    let reg_register = Register::new(reg, operates_on_word).unwrap();
    let rm_register = Register::new(rm, operates_on_word).unwrap();

    let (source, destination) = if destination_in_reg {
        (rm_register, reg_register)
    } else {
        (reg_register, rm_register)
    };

    dbg!(&source);
    dbg!(&destination);
}
