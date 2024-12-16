use pyo3::{exceptions::PyTypeError, prelude::*};

#[pyfunction]
// TODO: Implement a function that returns a list containing the first `n` numbers in Fibonacci's sequence.
//  It must raise a `TypeError` if `n` is not an integer or if it is less than 0.
fn fibonacci(n: i64) -> PyResult<Vec<i64>> {
    if n < 0 {
        return Err(PyTypeError::new_err("n is less than zero"));
    }

    let mut fibonaccis: Vec<i64> = Vec::new();
    let mut fib_1: i64 = 0;
    let mut fib_2: i64 = 1;
    if n == 0 {
        return Ok(fibonaccis);
    }
    fibonaccis.push(fib_1);
    if n == 1 {
        return Ok(fibonaccis);
    }

    fibonaccis.push(fib_2);
    if n == 2 {
        return Ok(fibonaccis);
    } else {
        let mut tmp: i64;
        for _ in 3..(n + 1) {
            tmp = fib_2;
            fib_2 = fib_1 + tmp;
            fibonaccis.push(fib_2);
            fib_1 = tmp;
        }
        return Ok(fibonaccis);
    }
}

#[pymodule]
fn exceptions(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}
