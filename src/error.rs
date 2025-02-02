use std::sync::{LazyLock, Mutex};

pub trait CompilerError: Sync + Send + std::fmt::Debug {
    fn line(&self) -> usize;
    fn index(&self) -> usize;
    fn to_string(&self) -> String;
}

type BoxError = Box<dyn CompilerError>;

#[derive(Debug)]
pub struct ErrorList(Vec<BoxError>);

impl ErrorList {
    fn new() -> Self {
        ErrorList(vec![])
    }
    pub fn push(&mut self, error: BoxError) {
        self.0.push(error);
    }
    pub fn print(&self) {
        for error in &self.0 {
            eprintln!(
                "error on line {} at position {}: {}",
                error.line(),
                error.index(),
                error.to_string()
            );
        }
    }
}

pub static ERRORS: LazyLock<Mutex<ErrorList>> = LazyLock::new(|| Mutex::new(ErrorList::new()));
#[macro_export]
macro_rules! errors {
    () => {
        ERRORS.lock().unwrap()
    };
}
pub fn len_errors() -> usize {
    ERRORS.lock().unwrap().0.len()
}
