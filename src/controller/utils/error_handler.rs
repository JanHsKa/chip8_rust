pub struct ErrorHandler {}

impl Default for ErrorHandler {
    fn default() -> Self {
        ErrorHandler::new()
    }
}

impl ErrorHandler {
    pub fn new() -> ErrorHandler {
        ErrorHandler {}
    }
}
