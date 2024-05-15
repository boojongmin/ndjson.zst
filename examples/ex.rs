use std::io;
use std::io::{BufWriter, Cursor, Read, Write};
use zstd::stream::read::Decoder;

fn main() {
    let writer = BufWriter::new(Cursor::new(Vec::new()));
    let mut encoder = zstd::stream::write::Encoder::new(writer, 0).unwrap();
    for _ in 0..1000 {
        encoder.write_all("hello world\n".as_bytes()).unwrap();
    }
    let r = encoder.finish().unwrap();
    let encoded = r.buffer();
    println!("compressed size: {:?}", encoded.len());

    let mut decoder = Decoder::new(io::Cursor::new(encoded)).unwrap();

    let mut decompressed_size = 0;
    let mut buf = [0u8; 10];
    loop {
        match decoder.read(&mut buf) {
            Ok(size) => {
                if size > 0 {
                    // println!("{:?}", String::from_utf8_lossy(&buf[..size]));
                }
                decompressed_size += size;
                if size < 10 {
                    break;
                }
            }
            Err(e) => {
                break;
            }
        }
    }
    println!("decompressed size: {:?}", decompressed_size);
}
