#![allow(warnings)]

use std::env;
use std::fs;
use std::fs::create_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::io::Read;
use std::fmt;
use std::env::consts::OS;


#[cfg(target_os = "linux")]
include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));

#[cfg(target_os = "windows")]
include!(concat!(env!("OUT_DIR"), "\\gl_bindings.rs"));


impl fmt::Debug for Gl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "opengl fmt")
    }
}