//! This handles all possible unused warnings (but it's a little overkill, so you can remove it if you want to see all the warnings)
#![allow(unused)]

// todo: Improve the dev_utils using something like: `use dev_utils::prelude::*;`
// * `dev_utils::prelude::*` cause it will be easier to use all the functions from the prelude
use dev_utils::{log::rlog::RLog, print_app_data};
use log::{info, LevelFilter};

use rubix_realm::{
    init_window,
    math::polynomial_eval::*,
    math::{surface::*, Matrix, Point},
};

fn main() {
    // * Initialize the logger
    print_app_data(file!());
    // RLog::init_logger(LevelFilter::Info);

    // info!("Starting the main tester...");

    init_window();
}
