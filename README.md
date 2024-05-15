# NdjsonZst

NdjsonZst is a Rust library for writing and reading newline-delimited JSON (NDJSON) files that are compressed using Zstandard (zstd). This library provides two primary structures: `NdjsonZstWriter` and `NdjsonZstReader`, enabling you to efficiently handle NDJSON files with compression.

## Features

- **Write NDJSON with Zstandard Compression**: Use `NdjsonZstWriter` to write NDJSON data to a compressed file.
- **Read NDJSON from Compressed Files**: Use `NdjsonZstReader` to read NDJSON data from a compressed file.
- **Optional Line Removal**: `NdjsonZstWriter` allows writing data while removing newline (`\n`) and carriage return (`\r`) characters from the input string.

## Usage

### Adding Dependencies

To use NdjsonZst, add the following dependencies to your `Cargo.toml`:

```toml
[dependencies]
ndjson_zst = "0.1.0"
```

### Example

#### Writing NDJSON Data

```rust
use std::fs::File;
use ndjson_zst::NdjsonZstWriter;

fn main() {
    let file_path = "data.zst";
    let mut writer = NdjsonZstWriter::from(file_path);

    writer.write(r#"{"name": "Alice", "age": 30}"#);
    writer.write_with_remove_line("Line with\nnewlines\n");
}
```

#### Reading NDJSON Data

```rust
use ndjson_zst::NdjsonZstReader;

fn main() {
    let file_path = "data.zst";
    let reader = NdjsonZstReader::from(file_path);

    for line in reader.lines() {
        println!("{}", line);
    }
}
```

### API Documentation

#### `NdjsonZstWriter`

##### `from(value: &str) -> Self`

Creates a new `NdjsonZstWriter` instance from a file path.

##### `new(f: File) -> Self`

Creates a new `NdjsonZstWriter` instance from a `File`.

##### `write(&mut self, data: &str)`

Writes a string to the NDJSON file, followed by a newline character.

##### `write_with_remove_line(&mut self, data: &str)`

Writes a string to the NDJSON file after removing all newline (`\n`) and carriage return (`\r`) characters, followed by a newline character.

#### `NdjsonZstReader`

##### `from(value: &str) -> Self`

Creates a new `NdjsonZstReader` instance from a file path.

##### `new(f: File) -> Self`

Creates a new `NdjsonZstReader` instance from a `File`.

##### `lines(&self) -> std::str::Lines`

Returns an iterator over the lines of the NDJSON file.

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have any suggestions or improvements.

## Acknowledgements

- [zstd-rs](https://github.com/gyscos/zstd-rs) for the Zstandard bindings for Rust.

## Notes

- The current implementation of `NdjsonZstReader` reads the entire file into memory. Future improvements may include supporting streaming reads for handling large files efficiently.

---

This README provides a basic overview and example usage of the NdjsonZst library. For more detailed information, please refer to the source code and inline documentation.