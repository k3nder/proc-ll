

pub mod context;
pub mod program;
pub mod values;
pub mod token;
#[cfg(test)]
mod tests;
///measures the execution time and prints it on the screen,
/// example:
///```
/// use processor::measure_time;
/// measure_time! ({println! ("hello ") });
/// ```
/// return the result of the code inside, only print the execution time if the RUST_LOG = debug
#[macro_export]
macro_rules! measure_time {
    ($expression:expr) => {{
        use std::time::Instant;

        let start = Instant::now();
        let result = $expression;  // Ejecuta la expresión
        let duration = start.elapsed();

        println!("Run Time: {:?}", duration);

        result  // Retorna el resultado de la expresión
    }};
}
