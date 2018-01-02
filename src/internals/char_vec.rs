#[derive(Debug)]
pub struct InternalCharVec {
    chars: Vec<char>,
    index: usize,
    back_index: Option<usize>,
}

impl InternalCharVec {
    pub fn new(chars: Vec<char>) -> InternalCharVec {
        let len = chars.len();
        let mut back_index = None;
        if len > 0 {
            back_index = Some(len - 1);
        }
        InternalCharVec {
            chars: chars,
            index: 0,
            back_index: back_index,
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

        Some(self.chars[self.index])
    }

    pub fn next_back(&mut self) -> Option<char> {
        if let None = self.back_index {
            return None;
        }

        let index = self.back_index.unwrap();
        let result = Some(self.chars[index]);
        if index == 0 {
            self.back_index = None;
        }else{
            self.back_index = Some(index - 1);
        }
        result
    }

    pub fn peek_back(&mut self) -> Option<char> {
        if let None = self.back_index {
            return None;
        }

        let index = self.back_index.unwrap();
        Some(self.chars[index])
    }
}
