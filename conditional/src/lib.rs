// Copyright 2019 Alexander Krivács Schrøder
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

use proc_macro_hack::proc_macro_hack;

/// Lets you use the syntax of the conditional operator in Rust. Also known as the ternary operator.
///
/// # Examples
/// ```
/// # #[macro_use] extern crate conditional;
/// #
/// # fn main() {
/// let x = conditional!(69 > 42 ? "hello" : "world");
/// assert_eq!(x, "hello");
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate conditional;
/// #
/// # fn main() -> Result<(), String> {
/// let r: Result<i32, String> = Ok(5);
/// // More complicated expressions are supported, but you might have to use
/// // parentheses to satisfy the macro parser
/// let x = conditional!((r? > 42) ? "hello" : "world");
/// assert_eq!(x, "world");
/// # Ok(())
/// # }
/// ```
/// ```
/// # #[macro_use] extern crate conditional;
/// #
/// # fn main() -> Result<(), String> {
/// // More complicated expressions are supported, but you might have to use
/// // parentheses to satisfy the macro parser
/// let x = conditional!(("3".parse::<i32>().map_err(|e| e.to_string())? > 42)
///     ? "hello" : "world");
/// assert_eq!(x, "world");
/// # Ok(())
/// # }
/// ```
#[proc_macro_hack]
pub use conditional_impl::conditional;
