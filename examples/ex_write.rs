use std::fs::File;
use std::panic::catch_unwind;
use ndjson_zst::{NdjsonZstReader, NdjsonZstWriter};


const DATA: &'static str = r#"{"a": 1}"#;
const PATH: &'static str = "test.ndjson.zst";

fn main() {

    let _ = catch_unwind(|| {
        write_example();
        read_example();
    });

    std::fs::remove_file(PATH).unwrap();
}


fn write_example(){
    let file = File::create(PATH).unwrap();
    let mut ndjson_zst_writer = NdjsonZstWriter::new(file);
    ndjson_zst_writer.write(DATA);
    ndjson_zst_writer.write(DATA);
    ndjson_zst_writer.write(DATA);
}

fn read_example() {
    let file = File::open(PATH).unwrap();
    let mut ndjson_zst_reader = NdjsonZstReader::new(file);

    for line in ndjson_zst_reader.lines() {
        assert_eq!(line, DATA);
    }

}