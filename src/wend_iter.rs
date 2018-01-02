use std::iter::{Iterator, DoubleEndedIterator};
use internals::InternalCharVec;

#[derive(Debug)]
pub enum WendIterator {
    Chars { chars: InternalCharVec },
}

impl WendIterator {
    pub fn from_chars(chars: InternalCharVec) -> WendIterator {
        WendIterator::Chars {
            chars: chars
        }
    }
}

impl Iterator for WendIterator {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self {
            &mut WendIterator::Chars { ref mut chars } => {
                chars.next()
            },
        }
    }
}

impl DoubleEndedIterator for WendIterator {
    fn next_back(&mut self) -> Option<char> {
        match self {
            &mut WendIterator::Chars { ref mut chars } => {
                chars.next_back()
            },
        }
    }
}