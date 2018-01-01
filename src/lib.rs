mod internals;

use std::str;
use std::fs::File;
use std::io;
use std::iter::Iterator;
use internals::{InternalCharVec, InternalFile, InternalStdin};

pub enum CharStream {
    Chars { chars: InternalCharVec },
    File { file: InternalFile },
    StdIn { stdin: InternalStdin },
}

impl CharStream {
    pub fn from(s: &str) -> CharStream {
        CharStream::Chars {
            chars: InternalCharVec::new(s.chars().collect())
        }
    }

    pub fn from_string(s: String) -> CharStream {
        CharStream::Chars {
            chars: InternalCharVec::new(s.chars().collect()),
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<CharStream, &str> {
        if let Ok(s) = str::from_utf8(bytes) {
            Ok(CharStream::Chars {
                chars: InternalCharVec::new(s.chars().collect())
            })
        }else{
            Err("can't convert utf8 string from bytes.")
        }
    }

    pub fn from_file(file: File) -> CharStream {
        CharStream::File {
            file: InternalFile::new(file)
        }
    }

    pub fn from_stdin(stdin: io::Stdin) -> CharStream {
        let internal = InternalStdin::new(stdin);
        CharStream::StdIn {
            stdin: internal
        }
    }
}

impl Iterator for CharStream {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self {
            &mut CharStream::Chars { ref mut chars } => {
                chars.next()
            },
            &mut CharStream::File { ref mut file } => {
                file.next()
            },
            &mut CharStream::StdIn { ref mut stdin } => {
                stdin.next()
            },
        }
    }
}


#[cfg(test)]
mod tests {
    extern crate tempfile;

    use super::*;
    use std::io::prelude::*;
    use std::io::{Seek, SeekFrom};
    use std::fs::File;

    #[test]
    fn from_str() {
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
    }

    #[test]
    fn from_string() {
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
    }

    #[test]
    fn from_bytes() {
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
        }else{
            panic!("can't convert stream from bytes.");
        }
    }

    #[test]
    fn from_file() {
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
}
