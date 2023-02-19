#[derive(Debug, Clone)]
pub struct Error {
    line: usize,
    message: String,
}

impl Error {
    pub fn new(line: usize, message: String) -> Self {
        Self { line, message }
    }
}

impl ToString for Error {
    fn to_string(&self) -> String {
        format!("Error occured at line {}: {}", self.line, self.message)
    }
}

#[derive(Debug, Clone)]
pub enum LoxErrors {
    ParseError(Error),
    RunTimeException(Error),
}

impl ToString for LoxErrors {
    fn to_string(&self) -> String {
        match self {
            LoxErrors::ParseError(error) => error.to_string(),
            LoxErrors::RunTimeException(error) => error.to_string(),
        }
    }
}

impl LoxErrors {
    pub fn report(&self) {
        eprintln!("{:?}\n{}", self, self.to_string());
    }
}

pub type LoxResult<T> = Result<T, LoxErrors>;
