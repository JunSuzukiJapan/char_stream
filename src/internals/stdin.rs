use std::io::Stdin;
use internals::InternalCharVec;

pub struct InternalStdin {
    stdin: Stdin,
    buf: Option<InternalCharVec>,
    need_read: bool,
    is_eof: bool,
}

impl InternalStdin {
    pub fn new(stdin: Stdin) -> InternalStdin {
        InternalStdin {
            stdin: stdin,
            buf: None,
            need_read: true,
            is_eof: false,
        }
    }

    pub fn next(&mut self) -> Option<char> {
        if self.is_eof {
            return None;
        }

        if self.need_read {
            self.need_read = false;
            self.read_line();
        }

        if let Some(ref mut char_vec) = self.buf {
            let result = char_vec.next();
            if let None = char_vec.peek() {
                self.need_read = true;
            }
            result

        }else{
            None
        }

    }

    fn read_line(&mut self){
        let mut input = String::new();
        if let Err(_) = self.stdin.read_line(&mut input) {
            self.is_eof = true;
            self.buf = None;
            self.need_read = false;
            return;
        }

        let char_vec = InternalCharVec::new(input.chars().collect());
        self.buf = Some(char_vec);
    }
}