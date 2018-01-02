# char_stream

Unified character reading interface to str, String, bytes, File and Stdin for Rust language.

# Installation

In Cargo.toml:

```
[dependencies]
char_stream = "*"
```

# Examples

## for str

```rust
use char_stream::CharStream;

let mut stream = CharStream::from("Hello 世界❤");

assert_eq!('H', stream.next().unwrap());
assert_eq!('e', stream.next().unwrap());
assert_eq!('l', stream.next().unwrap());
assert_eq!('l', stream.next().unwrap());
assert_eq!('o', stream.next().unwrap());
assert_eq!(' ', stream.next().unwrap());
assert_eq!('世', stream.next().unwrap());
assert_eq!('界', stream.next().unwrap());
assert_eq!('❤', stream.next().unwrap());
assert_eq!(None, stream.next());
```

## for String

```rust
use char_stream::CharStream;

let s = String::from("Hello 世界❤");
let mut stream = CharStream::from_string(s);

assert_eq!('H', stream.next().unwrap());
assert_eq!('e', stream.next().unwrap());
assert_eq!('l', stream.next().unwrap());
assert_eq!('l', stream.next().unwrap());
assert_eq!('o', stream.next().unwrap());
assert_eq!(' ', stream.next().unwrap());
assert_eq!('世', stream.next().unwrap());
assert_eq!('界', stream.next().unwrap());
assert_eq!('❤', stream.next().unwrap());
assert_eq!(None, stream.next());
```

## for bytes

```rust
use char_stream::CharStream;

let bytes: [u8; 15] = [72, 101, 108, 108, 111, 32, 228, 184, 150, 231, 149, 140, 226, 157, 164];
if let Ok(mut stream) = CharStream::from_bytes(&bytes) {
    assert_eq!('H', stream.next().unwrap());
    assert_eq!('e', stream.next().unwrap());
    assert_eq!('l', stream.next().unwrap());
    assert_eq!('l', stream.next().unwrap());
    assert_eq!('o', stream.next().unwrap());
    assert_eq!(' ', stream.next().unwrap());
    assert_eq!('世', stream.next().unwrap());
    assert_eq!('界', stream.next().unwrap());
    assert_eq!('❤', stream.next().unwrap());
    assert_eq!(None, stream.next());
}
```

## for File

```rust
extern crate tempfile;
extern crate char_stream;

use std::io::prelude::*;
use std::io::{Seek, SeekFrom};
use std::fs::File;
use char_stream::CharStream;

fn main(){
    let test_data = "Hello\n 世界❤";

    // write test data to tempfile
    let mut tmpfile: File = tempfile::tempfile().unwrap();
    tmpfile.write_all(test_data.as_bytes()).unwrap();

    // Seek to start
    tmpfile.seek(SeekFrom::Start(0)).unwrap();

    // read test data from tempfile
    let mut stream = CharStream::from_file(tmpfile);

    assert_eq!('H', stream.next().unwrap());
    assert_eq!('e', stream.next().unwrap());
    assert_eq!('l', stream.next().unwrap());
    assert_eq!('l', stream.next().unwrap());
    assert_eq!('o', stream.next().unwrap());
    assert_eq!('\n', stream.next().unwrap());
    assert_eq!(' ', stream.next().unwrap());
    assert_eq!('世', stream.next().unwrap());
    assert_eq!('界', stream.next().unwrap());
    assert_eq!('❤', stream.next().unwrap());
    assert_eq!(None, stream.next());
}
```

## for Stdin

```rust
extern crate char_stream;

use char_stream::CharStream;

fn main() {
     let mut stream = CharStream::from_stdin();
     while let Some(ch) = stream.next() {
         println!("ch: {}", ch);
     }
}
```


## reverse str

```rust
extern crate char_stream;
use char_stream::CharStream;

fn main() {
    let input = "stressed";
    let stream = CharStream::from(input);
    let rev_stream = stream.wend_iter().rev();
    let result: String = rev_stream.collect();
    println!("'{}' reverse to '{}'", input , result);
}
```
