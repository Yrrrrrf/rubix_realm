#![allow(unused)]


// ? Library metadata -------------------------------------------------------------------------
// * External crates used in the library
#[cfg(feature = "3d_render")]
extern crate winit;
#[cfg(feature = "3d_render")]
extern crate image;

// * Modules used in the library
mod solver;
pub mod render;


// ? General library functionalities ----------------------------------------------------------
pub fn general_functionality() {
    // Code for general functionality of your library
}

#[cfg(feature = "3d_render")]
pub fn render_specific_functionality() {
    // Code specific to 3D rendering feature
}
