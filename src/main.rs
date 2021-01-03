//! # fl2rust
//! A fluid (fltk ui designer) file to Rust transpiler.

//! To run on the command-line, install using cargo-install:
//! ```
//! $ cargo install fl2rust
//! ``` 
//! Then run:
//! ```
//! $ fl2rust <fl file>.fl > <output file>.rs
//! ```

//! To automate through cargo, you can use fl2rust as a library by adding it to your build-dependencies:
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! fltk = "0.12"

//! [build-dependencies]
//! fl2rust = "0.1"
//! ```

//! ```rust
//! //! build.rs
//! fn main() {
//!     use std::path::PathBuf;
//!     use std::env;
//!     println!("cargo:rerun-if-changed=src/myuifile.fl");
//!     let g = fl2rust::Generator::default();
//!     let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
//!     g.in_out("src/myuifile.fl", out_path.join("myuifile.rs")).expect("Failed to generate rust from fl file!");
//! }
//! ```

//! ```
//! # src/myuifile.fl -> generated via fluid
//! # data file for the Fltk User Interface Designer (fluid)
//! version 1.0400
//! header_name {.h}
//! code_name {.cxx}
//! class UserInterface {open
//! } {
//!   Function {make_window()} {open
//!   } {
//!     Fl_Window {} {open selected
//!       xywh {138 161 440 355} type Double visible
//!     } {
//!       Fl_Button but {
//!         label {Click me}
//!         xywh {175 230 95 45}
//!       }
//!     }
//!   }
//! }
//! ```

//! ```rust
//! //! src/myuifile.rs
//! include!(concat!(env!("OUT_DIR"), "/myuifile.rs"));
//! ```

//! ```rust
//! //! src/main.rs
//! use fltk::*;
//! mod myuifile;

//! fn main() {
//!     let app = app::App::default();
//!     let mut ui = myuifile::UserInterface::make_window();
//!     ui.but.set_callback(|| {
//!         println!("Works!");
//!     })
//!     app.run().unwrap();
//! }
//! ```


use std::error;
use std::fs;

fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let f = fs::read_to_string(&args[1])?;
    println!("{}", fl2rust::gen::generate(&fl2rust::parser::parse(&f)));
    Ok(())
}