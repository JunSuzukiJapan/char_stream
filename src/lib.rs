use std::str;
use std::io::{BufReader, BufRead};
use std::fs::File;

pub enum CharStream {
    Chars { chars: InternalCharVec },
    File { file: InternalFile },
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

    pub fn from_file(path: &str) -> Result<CharStream, &'static str> {
        if let Ok(f) = InternalFile::open(path) {
            Ok(CharStream::File { file: f })
        }else{
            Err("can't open file.")
        }
    }

    pub fn next(&mut self) -> Option<char> {
        match self {
            &mut CharStream::Chars { ref mut chars } => {
                chars.next()
            },
            &mut CharStream::File { ref mut file } => {
                file.next()
            },
        }
    }
}

pub struct InternalCharVec {
    chars: Vec<char>,
    index: usize,
}

impl InternalCharVec {
    pub fn new(chars: Vec<char>) -> InternalCharVec {
        InternalCharVec {
            chars: chars,
            index: 0,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.index >= self.chars.len() {
            return None;
        }

        let result = Some(self.chars[self.index]);
        self.index += 1;
        result
    }
}

pub struct InternalFile {
    reader: BufReader<File>,
    buf: Option<InternalCharVec>,
}

impl InternalFile {
    pub fn open(path: &str) -> Result<InternalFile, &'static str> {
        if let Ok(f) = File::open(path) {
            let reader = BufReader::new(f);
            Ok(InternalFile {
                reader: reader,
                buf: None,
            })
        }else{
            Err("can't open file.")
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if let Some(ref mut char_vec) = self.buf {
            char_vec.next()

        }else{
            loop {
                let mut buffer = String::new();
                if let Ok(_) = self.reader.read_line(&mut buffer) {
                    if buffer.len() == 0 {
                        continue;
                    }
                    let mut char_vec = InternalCharVec::new(buffer.chars().collect());
                    let result = char_vec.next();
                    self.buf = Some(char_vec);
                    return result;

                }else{
                    return None;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate tempfile;

    use super::*;
    use std::io::prelude::*;
    use std::io::{BufReader, Seek, SeekFrom};
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
        let test_data = "Hello 世界❤";

        // write test data to tempfile
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write_all(test_data.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // read test data from tempfile
        let mut buf = String::new();
        {
            let mut reader = BufReader::new(tmpfile);
            reader.read_line(&mut buf).unwrap();
        }
        assert_eq!(test_data, buf);
    }
}
