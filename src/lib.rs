// src/lib.rs
#![feature(custom_inner_attributes)]

use pyo3::prelude::*;




#[pymodule]
fn gizmo_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    let _ = m.add_function(wrap_pyfunction!(create_game_object, m)?);
    let _ = m.add_function(wrap_pyfunction!(move_game_object, m)?);
    Ok(())
}
#[pyfunction]
fn create_game_object(_name: &str)  {

}

#[pyfunction]
fn move_game_object(_name: &str, _x: f32, _y: f32, _z: f32) {

}
