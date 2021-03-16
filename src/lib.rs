//! # Ut
//!
//! Wraps asynchronous functions in order to make them synchronous based on if a
//! "sync" feature is enabled. This is useful when writting http clients as you
//! can write async methods and wrap them for use in synchronous code bases
//! automatically.
//!
//! # Usage
//! ```toml
//! [dependencies]
//! ut = "0.2.0"
//! ```
//!
//! Then in the crate that you want to have synchronous functions created for you
//! you create a sync feature. When this feature is enabled ut will create
//! synchronous functions on what you have wrapped.
//!
//! you can either: 
//! - Replace your asynchronous function with a synchronous one
//! - Clone your asynchronous function with a synchronous one ending in _blocking
//! - Clone all methods in an impl with synchronous ones
//!
//! # Replacing async functions
//!
//! You can replace your asynchronous function with a synchronous one by doing
//! something like the following:
//!
//!
//! ```rust
//! #[ut::wrap]
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
//! # Cloning async functions 
//!
//! You can clone your asynchronous function with a synchronous one ending in _blocking
//! by doing something like the following:
//!
//!
//! ```
//! #[ut::clone]
//! async fn foo(input: &str) -> String {
//!  format!("I am {} now", input)
//! }
//!
//! let out = foo_blocking("sync");
//! assert_eq!(out, "I am sync now".to_owned())
//! ```
//!
//! # Cloning async methods in implementations
//!
//! You can clone all methods in an impl with synchronous ones by using
//! ut::clone_impl. This is useful when you want to support both
//! async and sync functions in a struct implementation.
//!
//!
//! ```
//! // The original struct
//! #[derive(Default)]
//! pub struct Example {
//!   pub fooers: Fooers,
//! }
//!
//! // You also need to create the struct to place the cloned impls in
//! // This is done so you can choose what structs/impls to clone/wrap
//! // The cloned structs/impls should end in Blocking
//! #[derive(Default)]
//! pub struct ExampleBlocking {
//!   pub fooers: FooersBlocking,
//! }
//!
//! // The original struct with async functions
//! #[derive(Default)]
//! pub struct Fooers;
//!
//! // The blocking struct that we are cloning impls into
//! // You have to create this so you can add custom derives
//! #[derive(Default)]
//! pub struct FooersBlocking;
//!
//! // The async impls that you want to wrap
//! // All methods within this impl must be async
//! #[ut::clone_impl]
//! impl Fooers {
//!   pub async fn foo(&self, input: &str) -> String {
//!     format!("I am {} now", input)
//!   }
//!
//!   pub async fn bar(&self, input: &str) -> String {
//!     format!("I am also {} now", input)
//!   }
//! }
//! let example = ExampleBlocking::default();
//! let out = example.fooers.foo("sync");
//! assert_eq!(out, "I am sync now".to_owned());
//! let out = example.fooers.bar("sync");
//! assert_eq!(out, "I am also sync now".to_owned())
//! ```
//!
//! Currently the wrapping is very naive and simply wraps the function in
//! tokio::main. This is likely more expensive then it needs to be and I hope
//! to make it more efficient later.

use syn;
use quote::quote;
use proc_macro::TokenStream;

/// Wraps an async function in order to make it synchronous
///
/// # Examples
///
/// ```
/// #[ut::wrap]
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
  // get attributes (docstrings/examples) for our function
  let attrs = &func.attrs;
  // get visibility of function
  let vis = &func.vis;
  // get the name of our function
  let name = &func.sig.ident;
  // get information on the generics to pass
  let generics = &func.sig.generics;
  // get the arguments for our function
  let args = &func.sig.inputs;
  // get our output
  let output = &func.sig.output;
  // get the block of instrutions that are going to be called
  let block = &func.block;
  // cast back to a token stream
  let output = quote!{
    // iterate and add all of our attributes
    #(#attrs)*
    // conditionally add tokio::main if the sync feature is enabled
    #[cfg_attr(feature = "sync", tokio::main)]
    #vis async fn #name #generics(#args) #output { #block }
  };
  output.into()
}

/// Clones an async function in order to make it also synchronous
///
/// This will add _blocking to the name of the function to clone.
///
/// # Examples
///
/// ```
/// #[ut::clone]
/// async fn foo(input: &str) -> String {
///  format!("I am {} now", input)
/// }
///
/// let out = foo_blocking("sync");
/// assert_eq!(out, "I am sync now".to_owned())
/// ```
#[proc_macro_attribute]
pub fn clone(_meta: TokenStream, input: TokenStream) -> TokenStream {
  // parse the input stream into our async function
  let func = syn::parse_macro_input!(input as syn::ItemFn);
  // get attributes (docstrings/examples) for our function
  let attrs = &func.attrs;
  // get visibility of function
  let vis = &func.vis;
  // get the name of our function
  let name = &func.sig.ident;
  // get the name of our cloned function
  let sync_name = syn::Ident::new(&format!("{}_blocking", name), name.span());
  // get information on the generics to pass
  let generics = &func.sig.generics;
  // get the arguments for our function
  let args = &func.sig.inputs;
  // get our output
  let output = &func.sig.output;
  // get the block of instrutions that are going to be called
  let block = &func.block;
  // cast back to a token stream
  let output = quote!{
    // iterate and add all of our attributes
    #(#attrs)*
    // conditionally add tokio::main if the sync feature is enabled
    #vis async fn #name #generics(#args) #output { #block }
    
    // iterate and add all of our attributes
    #(#attrs)*
    // conditionally add tokio::main if the sync feature is enabled
    #[cfg_attr(feature = "sync", tokio::main)]
    #vis async fn #sync_name #generics(#args) #output { #block }
  };
  output.into()
}


/// Clones an group of async functions in an impl to a new sub structure
///
/// This is useful when you want to support both async and sync functions
/// in a struct implementation.
///
/// # Examples
///
/// ```
/// #[derive(Default)]
/// pub struct Example {
///   pub fooers: Fooers,
/// }
///
/// #[derive(Default)]
/// pub struct ExampleBlocking {
///   pub fooers: FooersBlocking,
/// }
///
/// #[derive(Default)]
/// pub struct Fooers;
///
/// #[derive(Default)]
/// pub struct FooersBlocking;
///
/// #[ut::clone_impl]
/// impl Fooers {
///   pub async fn foo(&self, input: &str) -> String {
///     format!("I am {} now", input)
///   }
/// }
///
/// let out = ExampleBlocking::default().fooers.foo("sync");
/// assert_eq!(out, "I am sync now".to_owned())
/// ```
#[proc_macro_attribute]
pub fn clone_impl(_meta: TokenStream, input: TokenStream) -> TokenStream {
  // parse the input stream into our async function
  let imp = syn::parse_macro_input!(input as syn::ItemImpl);
  // get attributes (docstrings/examples) for our function
  let attrs = &imp.attrs;
  // get the methods implemented in this impl
  let items = &imp.items;
  // get the self type for this impl
  let self_ty = match *imp.self_ty {
    syn::Type::Path(path)  =>  path,
    _ => panic!("Only type paths are supported"),
  };
  // build sync name
  let ident = self_ty.path.get_ident().unwrap();
  let sync_name = syn::Ident::new(&format!("{}Blocking", ident), ident.span());
  // get information on the generics to pass
  let generics = &imp.generics;
  // cast back to a token stream
  let output = quote!{
    // iterate and add all of the original async methods
    #(#attrs)*
    #generics 
    impl #self_ty {
      #(#items)*
    }

    // Clone our async methods but wrap them
    impl #sync_name {
      // wrap them to make the synchronous
      #(
        #[ut::wrap]
        #items
      )*
    }  
  };
  output.into()
}

