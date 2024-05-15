use std::fs::File;
use std::panic::catch_unwind;
use ndjson_zst::{NdjsonZstReader, NdjsonZstWriter};


const DATA: &'static str = r#"{
    "a": 1,
    "b": 2
}"#;

const DATA_EXPECTED: &'static str = r#"{    "a": 1,    "b": 2}{    "a": 1,    "b": 2}{    "a": 1,    "b": 2}"#;
const PATH: &'static str = "test.ndjson.zst";

fn main() {
    let _ = catch_unwind(|| {
        write_example();
        read_example();
    });

    std::fs::remove_file(PATH).unwrap();
}


fn write_example(){
    let mut ndjson_zst_writer = NdjsonZstWriter::from(PATH);
    ndjson_zst_writer.write_with_remove_line(DATA);
    ndjson_zst_writer.write_with_remove_line(DATA);
    ndjson_zst_writer.write_with_remove_line(DATA);
}

fn read_example() {
    let file = File::open(PATH).unwrap();
    let mut ndjson_zst_reader = NdjsonZstReader::from(PATH);

    let mut r = String::new();
    for line in ndjson_zst_reader.lines() {
        r.push_str(line);
    }
    assert_eq!(r, DATA_EXPECTED);

}