pub trait Logger {
    /// Помещает в лог сообщения заданного уровня.
    fn log(&self, verbosity: u8, message: &str);
}

struct StderrLogger;

impl Logger for StderrLogger {
    fn log(&self, verbosity: u8, message: &str) {
        eprintln!("verbosity={verbosity}: {message}");
    }
}

// TODO: Добавьте определение и реализацию Filter.
struct Filter<T> {
    inner: StderrLogger,
    max_verbosity: T,
}

impl<T: Fn(u8, &str) -> bool> Filter<T>{
    fn new(inner: StderrLogger, max_verbosity: T) -> Self{
        Filter {inner: inner, max_verbosity:  max_verbosity }
        
    }
}

impl<T: Fn(u8, &str) -> bool> Logger for Filter<T>{
    fn log(&self, verbosity: u8, message: &str) {
        if (self.max_verbosity)(verbosity, message){
            self.inner.log(verbosity, message);
        }
    }
}


fn main() {
    let logger = Filter::new(StderrLogger, |_verbosity, msg| msg.contains("yikes"));
    logger.log(5, "FYI");
    logger.log(1, "yikes, something went wrong");
    logger.log(2, "uhoh");
}
