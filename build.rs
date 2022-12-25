extern crate serde;

use serde::serde_if_integer128;

fn main() {
    serde_if_integer128! {
        println!("cargo:rustc-cfg=has_i128");
    }
}
