use std::str;

pub enum CharStream {
    Chars { chars: InternalCharVec },
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

    pub fn next(&mut self) -> Option<char> {
        match self {
            &mut CharStream::Chars { ref mut chars } => {
                chars.next()
            }
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

#[cfg(test)]
mod tests {
    use super::*;

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
}
