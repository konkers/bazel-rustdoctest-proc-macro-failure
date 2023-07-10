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

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr, Token,
};

struct ConcatString {
    a: LitStr,
    b: LitStr,
}

impl Parse for ConcatString {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        let a: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;
        let b: LitStr = input.parse()?;

        Ok(ConcatString { a, b })
    }
}

#[proc_macro]
pub fn concat_string(tokens: TokenStream) -> TokenStream {
    let input = parse_macro_input!(tokens as ConcatString);
    let mut value = input.a.value();
    value.push_str(&input.b.value());
    quote! {#value}.into()
}
