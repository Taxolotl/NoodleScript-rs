pub struct Error {
    pub location: Location,
    pub code: i32,
    pub message: String
}

impl Error {
    fn new(location: Location, code: i32, message: &str) -> Error {
        Error {
            location: location,
            code: code,
            message: message
        }
    }
}

impl Default for Error {
    fn default() -> Error {
        Error {
            location: Location::Lexer,
            code: -1,
            message: "Noodles"
        }
    }
}

pub enum Location {
    Lexer, 
    Parser, 
    Interpreter, 
    End
}