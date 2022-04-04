use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
extern crate sqlhelper;

#[pyfunction]
fn get_users()-> PyResult<String> {
    Ok(sqlhelper::get_users())
    // println!("{:?}",res)
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pymodule]
fn my_rust(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_users, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}