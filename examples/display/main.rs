// Copyright 2017 The gltf Library Developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

extern crate gltf as gltf_crate;

use gltf_crate::v1 as gltf;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    match gltf::import::<_, gltf::extras::Any>(path) {
        Ok(root) => {
            println!("glTF version 1.0");
            println!("{:#?}", root);
        },
        Err(err) => println!("Error: {:?}", err),
    }
}