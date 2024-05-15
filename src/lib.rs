use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Lines, Read, Write};
use zstd::Decoder;
use zstd::stream::AutoFinishEncoder;

use zstd::stream::write::Encoder;

pub struct NdjsonZstWriter<'a> {
    encoder: AutoFinishEncoder<'a , BufWriter<File>>,
    str_buf: String
}

impl From<&str> for NdjsonZstWriter<'_> {
    fn from(value: &str) -> Self {
        let file = File::create(value).unwrap();
        NdjsonZstWriter::new(file)
    }
}

impl NdjsonZstWriter<'_> {
    pub fn new(f: File) -> Self {
        let writer = BufWriter::new(f);
        let mut encoder = Encoder::new(writer, 0).unwrap();
        let mut encoder = encoder.auto_finish();
        NdjsonZstWriter { encoder, str_buf: String::new() }
    }

    pub fn write_with_remove_line(&mut self, data: &str) {
        self.str_buf.clear();
        for c in data.chars() {
            if c != '\n' && c != '\r' {
                self.str_buf.push(c);
            }
        }
        self.encoder.write_all(self.str_buf.as_bytes()).unwrap();
        self.encoder.write_all(b"\n").unwrap();
    }

    pub fn write(&mut self, data: &str) {
        self.encoder.write_all(data.as_bytes()).unwrap();
        self.encoder.write_all(b"\n").unwrap();
    }
}


pub struct NdjsonZstReader {
    buf: String
}

impl From<&str> for NdjsonZstReader {
    fn from(value: &str) -> Self {
        let file = File::open(value).unwrap();
        NdjsonZstReader::new(file)
    }
}

impl NdjsonZstReader {
    pub fn new(f: File) -> Self {
        let mut decoder = Decoder::new(f).unwrap();
        let mut buf = String::new();
        let _ = decoder.read_to_string(&mut buf);
        // TODO support stream
        NdjsonZstReader { buf }
    }

    pub fn lines(&self) -> std::str::Lines<'_> {
        self.buf.lines()
    }
}