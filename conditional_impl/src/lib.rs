// Copyright 2019 Alexander Krivács Schrøder
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

extern crate proc_macro;
use proc_macro_hack::proc_macro_hack;

use proc_macro::TokenStream;

use proc_macro2::TokenTree;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Token};

struct Conditional {
    condition: Expr,
    r#true: Expr,
    r#false: Expr,
}

fn get_until_question_mark(input: ParseStream) -> Result<Expr> {
    let mut skipped_tokens = vec![];
    while !input.peek(Token![?]) || (input.peek(Token![?]) && input.peek2(Token![?])) {
        let tree = input.parse::<TokenTree>()?;
        skipped_tokens.push(tree);
    }
    Ok(syn::parse2(skipped_tokens.into_iter().collect())?)
}

fn get_until_colon(input: ParseStream) -> Result<Expr> {
    let mut skipped_tokens = vec![];
    while !input.peek(Token![:]) || input.peek(Token![::]) {
        let tree = input.parse::<TokenTree>()?;
        skipped_tokens.push(tree);
    }
    Ok(syn::parse2(skipped_tokens.into_iter().collect())?)
}

impl Parse for Conditional {
    fn parse(input: ParseStream) -> Result<Self> {
        let condition = get_until_question_mark(input)?;
        input.parse::<Token![?]>()?;
        let r#true = get_until_colon(input)?;
        input.parse::<Token![:]>()?;
        let r#false: Expr = input.parse()?;

        Ok(Conditional {
            condition,
            r#true,
            r#false,
        })
    }
}

#[proc_macro_hack]
pub fn conditional(input: TokenStream) -> TokenStream {
    let Conditional {
        condition,
        r#true,
        r#false,
    } = parse_macro_input!(input as Conditional);

    let result = quote! {
        if #condition {
            #r#true
        } else {
            #r#false
        }
    };
    result.into()
}
