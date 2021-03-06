#![no_std]

extern crate print;
pub use print::print_main;

extern crate asku;
pub use asku::asku_main;

extern crate shutdown;
pub use shutdown::shutdown_main;

extern crate makefile;
pub use makefile::makefile_main;

extern crate editfile;
pub(crate) use editfile::editfile_main;
