// This handles all possible unused warnings (but it's a little overkill, so you can remove it if you want to see all the warnings)
// #![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


use rubix_realm::render::{
    window::init_window,
    cube::*,
};


fn main() {
    if let Err(e) = init_window() {eprintln!("Error: {}", e);}
}
