// Copyright 2023 The Pigweed Authors
//
// Licensed under the Apache License, Version 2.0 (the "License"); you may not
// use this file except in compliance with the License. You may obtain a copy of
// the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS, WITHOUT
// WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the
// License for the specific language governing permissions and limitations under
// the License.

//! # Example
//! ```
//! use lib_crate::concat_string;
//! assert_eq!(concat_string!("Hello ", "Rust!"), "Hello Rust!");
//! ```

pub use proc_macro_crate::concat_string;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_string_concatenates_string() {
        assert_eq!(concat_string!("Hello ", "Rust!"), "Hello Rust!");
    }
}
