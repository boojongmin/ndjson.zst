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
    let mut ndjson_zst_writer = NdjsonZstWriter::new(PATH, 0).unwrap();
    ndjson_zst_writer.write(DATA);
    ndjson_zst_writer.write(DATA);
    ndjson_zst_writer.write(DATA);
}

fn read_example() {
    let ndjson_zst_reader = NdjsonZstReader::new(PATH).unwrap();

    for line in ndjson_zst_reader {
        assert_eq!(line, DATA);
    }

}