use pyo3::prelude::*;
use std::collections::HashMap;

/// Formats the sum of two numbers as string.
/// Using PyResult can define the err message.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Sum two numbers.
#[pyfunction]
fn rsum(a: i32, b: i32) -> i32 {
    a + b
}

/// Sum the integer elements of an array.
#[pyfunction]
fn sum_i32_array(arr: Vec<i32>) -> i32 {
    arr.iter().sum()
}

/// Sum the integer elements of a dict.
#[pyfunction]
fn sum_i32_dict(map: HashMap<String, i32>) -> i32 {
    map.values().sum()
}

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn my_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_i32_dict, m)?)?;
    m.add_function(wrap_pyfunction!(rsum, m)?)?;
    m.add_function(wrap_pyfunction!(sum_i32_array, m)?)?;

    Ok(())
}
