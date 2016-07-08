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

use super::types::{ArgumentDirection, Interface, PropertyAccess};

pub fn signature_to_type(sig: &str) -> String {
    unimplemented!()
}

pub fn write(iface: Interface) -> String {
    let mut builder_buf = String::new();
    //let mut impl_buf = String::new();

    builder_buf.push_str("pub fn create_interface() -> InterfaceBuilder {");
    builder_buf.push_str("\n    Interface::new()");

    for method in iface.methods.iter() {
        builder_buf.push_str(&format!("\n        .add_method(\"{}\", Method::new())",
                                      method.name));

        for argument in method.arguments.iter() {
            let direction = match argument.direction {
                ArgumentDirection::In => "add_argument",
                ArgumentDirection::Out => "add_result",
            };

            builder_buf.push_str(&format!("\n            .{}(Argument::new(\"{}\", \"{}\"))",
                                          direction,
                                          argument.name,
                                          argument.type_));
        }

        for annotation in method.annotations.iter() {
            builder_buf.push_str(&format!("\n                    .add_annotation(Annotation::new(\"{}\", \"{}\"))",
                                          annotation.name,
                                          annotation.value));
        }
    }

    for signal in iface.signals.iter() {
        builder_buf.push_str(&format!("\n        .add_signal(\"{}\", Signal::new())",
                                      signal.name));

        for argument in signal.arguments.iter() {
            builder_buf.push_str(&format!("\n            .add_argument(Argument::new(\"{}\", \"{}\"))",
                                          argument.name,
                                          argument.type_));
        }

        for annotation in signal.annotations.iter() {
            builder_buf.push_str(&format!("\n                .add_annotation(Annotation::new(\"{}\", \"{}\"))",
                                          annotation.name,
                                          annotation.value));
        }
    }

    for property in iface.properties.iter() {
        let access = match property.access {
            PropertyAccess::Read => "ro",
            PropertyAccess::ReadWrite => "rw",
            PropertyAccess::Write => "wo",
        };

        builder_buf.push_str(&format!("\n        .add_property(\"{}\", Property::new_{}(Signature(\"{}\".to_string()), unimplemented!()))",
                                      property.name,
                                      access,
                                      property.type_));

        for annotation in property.annotations.iter() {
            builder_buf.push_str(&format!("\n                .add_annotation(Annotation::new(\"{}\", \"{}\"))\n",
                                          annotation.name,
                                          annotation.value));
        }
    }

    builder_buf.push_str("\n}\n");

    builder_buf
}
