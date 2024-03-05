use std::io::Read;

const MOV_OPCODE: u8 = 0b0010_0010;

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
    println!("Buf: {:?}", buf);

    // -> Check the instruction code
    // Ignore D, W flags (the least significant two bits) from the first
    // byte by shifting right two bits.
    println!("First byte: {:#b}", buf[0]);
    let opcode = buf[0] >> 2;
    println!("Instruction code: {:#010b}", opcode);

    if opcode != MOV_OPCODE {
        eprintln!("OPCODE not supported");
        return;
    }
}
