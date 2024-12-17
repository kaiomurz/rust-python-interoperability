// TODO: Expose a function named `max_k` that takes a list of unsigned integers and return as output
//   a list containing the `k` largest numbers in the list, in descending order.
//
// Hint: you can use the `num_bigint` crate if you think it'd be useful.

// use num_bigint::BigInt;
// use pyo3::{exceptions::PyTypeError, prelude::*, types::PyList, PyResult};

// #[pyfunction]
// fn max_k(mut int_list: Vec<BigInt>, k: usize) -> Vec<BigInt> {
//     int_list.sort();
//     int_list.reverse();
//     // int_list.iter().take(k).collect()
//     int_list.truncate(k);
//     int_list
// }

// Vec<BigUint>
// fn print_number_list(list: Bound<'_, PyList>) {
//     list.iter().for_each(|item| println!("{item}"));
// }

use num_bigint::BigInt;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;

#[pyfunction]
fn max_k(mut v: Vec<BigInt>, k: usize) -> PyResult<Vec<BigInt>> {
    if k > v.len() {
        return Err(PyValueError::new_err(
            "k must be less than the length of the list",
        ));
    }
    if v.iter().any(|x| x < &BigInt::from(0)) {
        return Err(PyTypeError::new_err("all numbers must be non-negative"));
    }
    v.sort();
    v.reverse();
    v.truncate(k);
    Ok(v)
}

#[pymodule]
fn outro1(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(max_k, m)?)?;
    Ok(())
}
