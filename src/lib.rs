#![feature(box_syntax)]
#![feature(convert)]
#![feature(libc)]
#![feature(link_args)]
#![feature(plugin)]
#![plugin(regex_macros)]

extern crate libc;
extern crate regex;

extern crate iron_llvm;
extern crate llvm_sys;

pub mod builder;
pub mod driver;
pub mod lexer;
//pub mod missing_llvm_bindings;
pub mod parser;
