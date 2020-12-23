//! # Syncwrap
//!
//! Wraps asynchronous functions in order to make them synchronous based on if a
//! "sync" feature is enabled. This is useful when writting http clients as you
//! can write async methods and wrap them for use in synchronous code bases
//! automatically.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! syncwrap = "0.1.0"
//! ```
//!
//! # Examples
//!
//! ```rust
//! #[syncwrap::wrap]
//! async fn foo(input: &str) -> String {
//!   format!("I am {} now", input)
//! }
//!
//! fn main() {
//!   let out = foo("sync");
//!   assert_eq!(out, "I am sync now".to_owned())
//! }
//! ```
//!
//! Currently the wrapping is very naive and simply wraps the function in
//! tokio::main. This is likey more expensive then it needs to be and I hope
//! to make it more efficient later.

use syn;
use quote::quote;
use proc_macro::TokenStream;

/// Wraps an async function in order to make it synchronous
///
/// # Examples
///
/// ```
/// #[syncwrap::wrap]
/// async fn foo(input: &str) -> String {
///  format!("I am {} now", input)
/// }
///
/// let out = foo("sync");
/// assert_eq!(out, "I am sync now".to_owned())
/// ```
#[proc_macro_attribute]
pub fn wrap(_meta: TokenStream, input: TokenStream) -> TokenStream {
  // parse the input stream into our async function
  let func = syn::parse_macro_input!(input as syn::ItemFn);
  // get visibility of function
  let vis = &func.vis;
  // get the name of our function
  let name = &func.sig.ident;
  // get the arguments for our function
  let args = &func.sig.inputs;
  // get our output
  let output = &func.sig.output;
  // get the block of instrutions that are going to be called
  let block = &func.block;
  // cast back to a token stream
  let output = quote!{
    #[cfg_attr(feature = "sync", tokio::main)]
    #vis async fn #name(#args) #output { #block }
  };
  output.into()
}
