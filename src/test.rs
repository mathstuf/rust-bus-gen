// Copyright (c) 2016, Ben Boeckel
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of this project nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
// ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
// DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
// ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
// (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
// LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON
// ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
// (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

extern crate serde;
use self::serde::Deserialize;

extern crate serde_xml;
use self::serde_xml::de::from_iter;

use super::write::*;
use super::types::*;

use std::fs::File;
use std::io::{Bytes, Read};

fn read_test_file<T: Deserialize>(name: &str) -> T {
    let fin = File::open(format!(concat!(env!("CARGO_MANIFEST_DIR"), "/data/{}.xml"), name)).unwrap();

    from_iter::<Bytes<File>, T>(fin.bytes()).unwrap()
}

#[test]
fn test_load_secret_service_collection() {
    let collection: Interface = read_test_file("secretservice/collection");

    assert_eq!(collection.name, "org.freedesktop.Secret.Collection");
    assert_eq!(collection.methods.len(), 3);
    assert_eq!(collection.signals.len(), 3);
    assert_eq!(collection.properties.len(), 5);
    assert!(collection.docstring.is_some());
    assert_eq!(collection.docstring.as_ref().unwrap(), "A collection of items containing secrets.");

    println!("{}", write(collection));

    assert!(false);
}
