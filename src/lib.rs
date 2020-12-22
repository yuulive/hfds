use syn;
use quote::quote;
use proc_macro::TokenStream;


#[proc_macro_attribute]
pub fn wrapper(_meta: TokenStream, input: TokenStream) -> TokenStream {
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
