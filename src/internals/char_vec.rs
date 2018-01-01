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

    pub fn peek(&mut self) -> Option<char> {
        if self.index >= self.chars.len() {
            return None;
        }

        let result = Some(self.chars[self.index]);
        result
    }
}
