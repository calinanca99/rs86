use std::io::Read;

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
}
