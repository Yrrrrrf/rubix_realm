// This handles all possible unused warnings (but it's a little overkill, so you can remove it if you want to see all the warnings)
// #![allow(unused)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]


use rubix_realm::render::setup::init_window;


fn main() {
    if let Err(e) =  pollster::block_on(init_window()) {eprintln!("Error: {}", e);}
}
