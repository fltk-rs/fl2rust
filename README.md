# fl2rust

## Usage

A fluid (fltk ui designer) file to Rust transpiler.

## MSRV
The minimum supported Rust version for 0.7 is 1.63 and fltk-rs > 1.5.4

### As an executable

You can run fl2rust on the command-line by installing using cargo-install:
```
$ cargo install fl2rust
``` 
Then run:
```
$ fl2rust <fl file>.fl > <output file>.rs
```

### As a proc-macro

(A template repo usable via [cargo-generate](https://crates.io/crates/cargo-generate) can be found [here](https://github.com/fltk-rs/fl2rust-template))
Add fl2rust-macro to your list of dependencies:
```toml
# Cargo.toml
[dependencies]
fltk = "1.5.4"
fl2rust-macro = "0.7"
```

The ui file that's generated by fluid, we'll name it myuifile.fl and keep it in our src directory:
```
# data file for the Fltk User Interface Designer (fluid)
version 1.0400
header_name {.h}
code_name {.cxx}
class UserInterface {open
} {
  Function {make_window()} {open
  } {
    Fl_Window {} {open selected
      xywh {138 161 440 355} type Double visible
    } {
      Fl_Button but {
        label {Click me}
        xywh {175 230 95 45}
      }
    }
  }
}
```

In our main source file:
```rust
use fltk::{prelude::*, *};

mod ui {
    fl2rust_macro::include_ui!("src/myuifile.fl");
}

fn main() {
    let a = app::App::default();
    let mut ui = ui::UserInterface::make_window();
    ui.but.set_callback(|b| println!("Button clicked!"));
    a.run().unwrap();
}
```

In our build.rs file:
```rust
fn main() {
    println!("cargo:rerun-if-changed=src/myuifile.fl");
}
```

### As a build-dependency

To automate things through cargo, you can use fl2rust as a library by adding it to your build-dependencies:

```toml
# Cargo.toml
[dependencies]
fltk = "1.5.4"

[build-dependencies]
fl2rust = "0.7"
```

```rust
// build.rs
fn main() {
    use std::path::PathBuf;
    use std::env;
    println!("cargo:rerun-if-changed=src/myuifile.fl");
    let g = fl2rust::Generator::default();
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    g.in_out("src/myuifile.fl", out_path.join("myuifile.rs").to_str().unwrap()).expect("Failed to generate rust from fl file!");
}
```

The ui file that's generated by fluid, we'll name it myuifile.fl and keep it in our src directory:
```
# data file for the Fltk User Interface Designer (fluid)
version 1.0400
header_name {.h}
code_name {.cxx}
class UserInterface {open
} {
  Function {make_window()} {open
  } {
    Fl_Window {} {open selected
      xywh {138 161 440 355} type Double visible
    } {
      Fl_Button but {
        label {Click me}
        xywh {175 230 95 45}
      }
    }
  }
}
```

```rust
// src/myuifile.rs
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::needless_update)]

include!(concat!(env!("OUT_DIR"), "/myuifile.rs"));
```

```rust
// src/main.rs
use fltk::{prelude::*, *};
mod myuifile;

fn main() {
    let app = app::App::default();
    let mut ui = myuifile::UserInterface::make_window();
    ui.but.set_callback(move |_| {
        println!("Works!");
    });
    app.run().unwrap();
}
```

## Where you can get FLUID?
There are several options:
- `cargo install fltk-fluid`
- Through a package manager.
- By building the fltk library yourself using cmake.

## i18n support
Version 0.4.4 adds i18n support via the `tr!` macro from the [tr crate](https://crates.io/crates/tr). 
To enable it:
- In fluid, go to Edit->Project Settings...->Internationalization.
- Change the dropdown to use GNU gettext (which the tr crate supports in both forms gettext-rs and gettext).
- Add tr to you dependencies in you Cargo.toml.
- Add to your main.rs file:
```rust
#[macro_use]
extern crate tr;
```
- Initialize tr as described in the tr crate's documentation.

## Known limitations
- Only constructor methods are supported.
- fl2rust doesn't check the generated Rust code for correctness.

## Tutorial
- [Use FLUID (RAD tool) with Rust](https://www.youtube.com/watch?v=k_P0wG3-dNk)

