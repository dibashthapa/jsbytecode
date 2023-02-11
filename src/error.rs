#[derive(Debug)]
pub struct LoxError {
    line: usize, 
    message: String
}

impl LoxError {
    pub fn error(line:usize, message: &str) {
        LoxError { line , message : message.to_string()}.report(line.to_string());
    }

    pub fn report(&self, loc: String) {
        eprintln!("[line {}] Error {}:{}", self.line , loc , self.message)
    }
}


