// src/lib.rs
#![feature(custom_inner_attributes)]

use pyo3::prelude::*;
use wgpu::Instance as WgpuInstance;
use physx::*;
use cpal::traits::{DeviceTrait, HostTrait};

#[pymodule]
fn gizmo_engine(_py: Python, m: &PyModule) -> PyResult<()> {
    let _ = m.add_function(wrap_pyfunction!(create_game_object, m)?);
    let _ = m.add_function(wrap_pyfunction!(move_game_object, m)?);
    Ok(())
}
#[pyfunction]
fn create_game_object(name: &str)  {

}

#[pyfunction]
fn move_game_object(name: &str, x: f32, y: f32, z: f32) {

}
