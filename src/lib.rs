// Copyright (C) 2017 <Jun Suzuki jun.suzuki.japan@gmail.com>
//
// This software may be modified and distributed under the terms
// of the MIT license.  See the LICENSE file for details.
//! # char_stream
//!
//! char_stream gives a unified character reading interface to str, String, bytes, File and Stdin.
//!
mod internals;
mod wend_iter;

use std::str;
use std::fs::File;
use std::io;
use std::iter::Iterator;
use internals::{InternalCharVec, InternalFile, InternalStdin};
use wend_iter::WendIterator;

#[derive(Debug)]
pub enum CharStream {
    Chars { chars: InternalCharVec },
    File { file: InternalFile },
    StdIn { stdin: InternalStdin },
}

impl CharStream {

    ///
    /// make new CharStream from str.
    ///
    /// Example:
    ///
    /// ```
    /// use char_stream::CharStream;
    ///
    /// let mut stream = CharStream::from("Hello 世界❤");
    ///
    /// assert_eq!('H', stream.next().unwrap());
    /// assert_eq!('e', stream.next().unwrap());
    /// assert_eq!('l', stream.next().unwrap());
    /// assert_eq!('l', stream.next().unwrap());
    /// assert_eq!('o', stream.next().unwrap());
    /// assert_eq!(' ', stream.next().unwrap());
    /// assert_eq!('世', stream.next().unwrap());
    /// assert_eq!('界', stream.next().unwrap());
    /// assert_eq!('❤', stream.next().unwrap());
    /// assert_eq!(None, stream.next());
    /// ```
    ///
    pub fn from(s: &str) -> CharStream {
        CharStream::Chars {
            chars: InternalCharVec::new(s.chars().collect())
        }
    }

    ///
    /// make new CharStream from String.
    ///
    /// Example:
    ///
    /// ```
    /// use char_stream::CharStream;
    ///
    /// let s = String::from("Hello 世界❤");
    /// let mut stream = CharStream::from_string(s);
    ///
    /// assert_eq!('H', stream.next().unwrap());
    /// assert_eq!('e', stream.next().unwrap());
    /// assert_eq!('l', stream.next().unwrap());
    /// assert_eq!('l', stream.next().unwrap());
    /// assert_eq!('o', stream.next().unwrap());
    /// assert_eq!(' ', stream.next().unwrap());
    /// assert_eq!('世', stream.next().unwrap());
    /// assert_eq!('界', stream.next().unwrap());
    /// assert_eq!('❤', stream.next().unwrap());
    /// assert_eq!(None, stream.next());
    /// ```
    ///
    pub fn from_string(s: String) -> CharStream {
        CharStream::Chars {
            chars: InternalCharVec::new(s.chars().collect()),
        }
    }

    ///
    /// make new CharStream from bytes.
    ///
    /// Example:
    ///
    /// ```
    /// use char_stream::CharStream;
    ///
    /// let bytes: [u8; 15] = [72, 101, 108, 108, 111, 32, 228, 184, 150, 231, 149, 140, 226, 157, 164];
    /// if let Ok(mut stream) = CharStream::from_bytes(&bytes) {
    ///     assert_eq!('H', stream.next().unwrap());
    ///     assert_eq!('e', stream.next().unwrap());
    ///     assert_eq!('l', stream.next().unwrap());
    ///     assert_eq!('l', stream.next().unwrap());
    ///     assert_eq!('o', stream.next().unwrap());
    ///     assert_eq!(' ', stream.next().unwrap());
    ///     assert_eq!('世', stream.next().unwrap());
    ///     assert_eq!('界', stream.next().unwrap());
    ///     assert_eq!('❤', stream.next().unwrap());
    ///     assert_eq!(None, stream.next());
    /// }
    /// ```
    ///
    pub fn from_bytes(bytes: &[u8]) -> Result<CharStream, &str> {
        if let Ok(s) = str::from_utf8(bytes) {
            Ok(CharStream::Chars {
                chars: InternalCharVec::new(s.chars().collect())
            })
        }else{
            Err("can't convert utf8 string from bytes.")
        }
    }

    ///
    /// make new CharStream from File.
    ///
    /// Example:
    ///
    /// ```
    /// extern crate tempfile;
    /// extern crate char_stream;
    ///
    /// use std::io::prelude::*;
    /// use std::io::{Seek, SeekFrom};
    /// use std::fs::File;
    /// use char_stream::CharStream;
    ///
    /// # fn main(){
    /// let test_data = "Hello\n 世界❤";
    ///
    /// // write test data to tempfile
    /// let mut tmpfile: File = tempfile::tempfile().unwrap();
    /// tmpfile.write_all(test_data.as_bytes()).unwrap();
    ///
    /// // Seek to start
    /// tmpfile.seek(SeekFrom::Start(0)).unwrap();
    ///
    /// // read test data from tempfile
    /// let mut stream = CharStream::from_file(tmpfile);
    ///
    /// assert_eq!('H', stream.next().unwrap());
    /// assert_eq!('e', stream.next().unwrap());
    /// assert_eq!('l', stream.next().unwrap());
    /// assert_eq!('l', stream.next().unwrap());
    /// assert_eq!('o', stream.next().unwrap());
    /// assert_eq!('\n', stream.next().unwrap());
    /// assert_eq!(' ', stream.next().unwrap());
    /// assert_eq!('世', stream.next().unwrap());
    /// assert_eq!('界', stream.next().unwrap());
    /// assert_eq!('❤', stream.next().unwrap());
    /// assert_eq!(None, stream.next());
    /// # }
    /// ```
    ///
     pub fn from_file(file: File) -> CharStream {
        CharStream::File {
            file: InternalFile::new(file)
        }
    }

    ///
    /// make new CharStream from stdin.
    ///
    /// Example:
    ///
    /// ```no_run
    /// extern crate char_stream;
    ///
    /// use char_stream::CharStream;
    ///
    /// fn main() {
    ///     let mut stream = CharStream::from_stdin();
    ///     while let Some(ch) = stream.next() {
    ///         println!("ch: {}", ch);
    ///     }
    /// }
    /// ```
    ///
    pub fn from_stdin() -> CharStream {
        let internal = InternalStdin::new(io::stdin());
        CharStream::StdIn {
            stdin: internal
        }
    }

    ///
    /// peek a next char
    ///
    pub fn peek(&mut self) -> Option<char> {
        match self {
            &mut CharStream::Chars { ref mut chars } => {
                chars.peek()
            },
            &mut CharStream::File { ref mut file } => {
                file.peek()
            },
            &mut CharStream::StdIn { ref mut stdin } => {
                stdin.peek()
            },
        }
    }

    ///
    /// read a line
    ///
    /// Example:
    ///
    /// ```
    /// extern crate char_stream;
    ///
    /// use char_stream::CharStream;
    ///
    /// fn main(){
    ///     let test_data = "Hello\n 世界❤";
    ///     let mut stream = CharStream::from(test_data);
    ///     assert_eq!("Hello", stream.read_line().unwrap());
    ///     assert_eq!(" 世界❤", stream.read_line().unwrap());
    ///     assert_eq!(None, stream.next());
    /// }
    /// ```
    ///
    pub fn read_line(&mut self) -> Option<String> {
        if let None = self.peek() {
            return None;
        }

        let mut result = String::new();

        while let Some(c) = self.next() {
            if c == '\n' {
                break;

            } else if c == '\r' {
                if let Some(c2) = self.peek() {
                    if c2 == '\n' {
                        self.next();
                        break;
                    }
                }else{
                    break;
                }
            }

            result.push(c);
        }

        Some(result)
    }

    ///
    /// to string
    ///
    /// Example:
    /// 
    /// ```
    /// use char_stream::CharStream;
    /// 
    /// let s = String::from("Hello 世界❤");
    /// let mut stream = CharStream::from_string(s);
    /// let result = stream.to_string();
    ///
    /// assert_eq!("Hello 世界❤", result);
    /// ```
    ///
    pub fn to_string(&mut self) -> String {
        let mut string = String::new();

        while let Some(c) = self.next() {
            string.push(c);
        }

        string
    }

    ///
    /// Convert to DoubleEndedIterator.
    ///  caution: CharStream made by 'from_stdin' can't convert.
    ///
    /// Example:
    ///
    /// ```
    /// extern crate char_stream;
    ///
    /// use char_stream::CharStream;
    ///
    /// fn main() {
    ///     let input = "stressed";
    ///     let stream = CharStream::from(input);
    ///     let mut rev_stream = stream.wend_iter().rev();
    ///     let mut result = String::new();
    ///     while let Some(c) = rev_stream.next() {
    ///         result.push(c);
    ///     }
    ///     println!("'{}' reverse to '{}'", input , result);
    /// }
    /// ```
    ///
    pub fn wend_iter(self) -> WendIterator {
        match self {
            CharStream::Chars { chars } => WendIterator::from_chars(chars),
            CharStream::File { file } => {
                let chars = file.read_and_get_all_chars();
                let char_vec = InternalCharVec::new(chars);
                WendIterator::from_chars(char_vec)
            },
            CharStream::StdIn { .. } => panic!("can't convert DoubleEndedIterator from CharStream made by 'from_stdin'"),
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

    #[test]
    fn from_file_wend_iter() {
        let test_data = "Hello\n 世界❤";

        // write test data to tempfile
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write_all(test_data.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // read test data from tempfile
        let file_stream = CharStream::from_file(tmpfile);
        let mut stream = file_stream.wend_iter().rev();

        assert_eq!('❤', stream.next().unwrap());
        assert_eq!('界', stream.next().unwrap());
        assert_eq!('世', stream.next().unwrap());
        assert_eq!(' ', stream.next().unwrap());
        assert_eq!('\n', stream.next().unwrap());
        assert_eq!('o', stream.next().unwrap());
        assert_eq!('l', stream.next().unwrap());
        assert_eq!('l', stream.next().unwrap());
        assert_eq!('e', stream.next().unwrap());
        assert_eq!('H', stream.next().unwrap());
        assert_eq!(None, stream.next());
    }

    #[test]
    fn read_line() {
        let test_data = "Hello\n 世界❤";
        let mut stream = CharStream::from(test_data);
        assert_eq!("Hello", stream.read_line().unwrap());
        assert_eq!(" 世界❤", stream.read_line().unwrap());
        assert_eq!(None, stream.next());
    }

    #[test]
    fn from_file_read_line() {
        let test_data = "Hello\n 世界❤";

        // write test data to tempfile
        let mut tmpfile: File = tempfile::tempfile().unwrap();
        tmpfile.write_all(test_data.as_bytes()).unwrap();

        // Seek to start
        tmpfile.seek(SeekFrom::Start(0)).unwrap();

        // read test data from tempfile
        let mut stream = CharStream::from_file(tmpfile);
        assert_eq!("Hello", stream.read_line().unwrap());
        assert_eq!(" 世界❤", stream.read_line().unwrap());
        assert_eq!(None, stream.next());
    }
}
