use pyo3::prelude::*;

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
fn fibonacci(n: usize) -> Vec<usize> {
    let mut fibonaccis: Vec<usize> = Vec::new();
    let mut fib_1: usize = 0;
    let mut fib_2: usize = 1;
    if n == 0 {
        return fibonaccis;
    }
    fibonaccis.push(fib_1);
    if n == 1 {
        return fibonaccis;
    }

    fibonaccis.push(fib_2);
    if n == 2 {
        return fibonaccis;
    } else {
        let mut tmp: usize;
        for _ in 3..(n + 1) {
            tmp = fib_2;
            fib_2 = fib_1 + tmp;
            fibonaccis.push(fib_2);
            fib_1 = tmp;
        }
        return fibonaccis;
    }
}

#[pymodule]
fn output(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
