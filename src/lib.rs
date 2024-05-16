use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

use zstd::Decoder;
use zstd::stream::AutoFinishEncoder;
use zstd::stream::write::Encoder;
use zstd::zstd_safe::CompressionLevel;

pub struct NdjsonZstWriter<'a> {
    encoder: AutoFinishEncoder<'a , BufWriter<File>>,
    str_buf: String
}

impl NdjsonZstWriter<'_> {
    pub fn new(path: &str, compression_level: CompressionLevel) -> Result<Self, std::io::Error> {
        let file = File::create(path).unwrap();
        let writer = BufWriter::new(file);
        return match Encoder::new(writer, compression_level) {
            Ok(encoder) => {
                let encoder = encoder.auto_finish();
                Ok(NdjsonZstWriter { encoder, str_buf: String::new() })
            }
            Err(x) => {
                Err(x)
            }
        };
    }

    pub fn write_with_remove_line(&mut self, data: &str) {
        self.str_buf.clear();
        for c in data.chars() {
            if c != '\n' && c != '\r' {
                self.str_buf.push(c);
            }
        }
        self.str_buf.push('\n');
        self.encoder.write_all(self.str_buf.as_bytes()).unwrap();
    }

    pub fn write(&mut self, data: &str) {
        self.str_buf.clear();
        self.str_buf.push_str(data);
        self.str_buf.push('\n');
        self.encoder.write_all(self.str_buf.as_bytes()).unwrap();
    }
}


pub struct NdjsonZstReader<'a> {
    lines: std::io::Lines<Box<BufReader<Decoder<'a, BufReader<File>>>>>,

}

impl NdjsonZstReader<'_> {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path).unwrap();
        return match Decoder::new(file) {
            Ok(decoder) => {
                let zstd_file_buf_reader = BufReader::new(decoder);
                Ok(NdjsonZstReader { lines: Box::new(zstd_file_buf_reader).lines() })
            },
            Err(e) => {Err(e)}
        };
    }
}

impl Iterator for NdjsonZstReader<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.lines.next() {
            Some(Ok(x)) => {
                return Some(x);
            },
            _ => {
                return None;
            }
        }
    }
}


#[cfg(test)]
mod test {
    use std::fs;

    use super::*;

    #[test]
    fn test_read() {
        {
            let path = "test.ndjson.zst";
            let mut ndjson_zst_writer = NdjsonZstWriter::new(path, CompressionLevel::default()).unwrap();
            ndjson_zst_writer.write("hello");
            ndjson_zst_writer.write("world");
            ndjson_zst_writer.write("!!");
        }

        let path = "test.ndjson.zst";
        let ndjson_zst_reader = NdjsonZstReader::new(path).unwrap();

        let mut r = String::new();
        for line in ndjson_zst_reader {
            r.push_str(&line);
        }

        assert_eq!(r, "helloworld!!");
        fs::remove_file(path).unwrap();
    }
}
