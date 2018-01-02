use std::io::{BufReader, BufRead};
use std::fs::File;
use internals::InternalCharVec;

#[derive(Debug)]
pub struct InternalFile {
    reader: BufReader<File>,
    buf: Option<InternalCharVec>,
    is_eof: bool,
}

impl InternalFile {
    pub fn open(path: &str) -> Result<InternalFile, &'static str> {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            let mut f = InternalFile {
                reader: reader,
                buf: None,
                is_eof: false,
            };
            f.read_next_line();

            Ok(f)
        }else{
            Err("can't open file.")
        }
    }

    pub fn new(file: File) -> InternalFile {
        let reader = BufReader::new(file);
        let mut f = InternalFile {
            reader: reader,
            buf: None,
            is_eof: false,
        };
        f.read_next_line();

        f
    }

    pub fn next(&mut self) -> Option<char> {
        if self.is_eof {
            return None;
        }

        let mut result = None;
        if let Some(ref mut char_vec) = self.buf {
            result = char_vec.next();
        }
        self.check_next_char_and_reset_buf_if_need();

        result
    }

    fn check_next_char_and_reset_buf_if_need(&mut self){
        // 次に読める文字がない場合、次の行を先読みする。
        let mut need_read = false;
        if let Some(ref mut char_vec) = self.buf {
            if let None = char_vec.peek() {
                need_read = true;
            }
        }else{
            need_read = true;
        }
        if need_read {
            self.read_next_line();
        }
    }

    fn read_next_line(&mut self){
        let mut buffer = String::new();
        if let Ok(_) = self.reader.read_line(&mut buffer) {
            let char_vec = InternalCharVec::new(buffer.chars().collect());
            self.buf = Some(char_vec);
        }else{
            self.is_eof = true;
        }
    }
}
