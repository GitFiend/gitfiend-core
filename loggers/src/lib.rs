use proc_macro::TokenStream;
use quote::quote;
use syn::{Item, ItemFn};

/*
This is taken modified from https://github.com/nmeylan/elapsed-time
 */

// We don't bother printing time if it took less than this.
const MIN_ELAPSED_MS: u128 = 1;

#[cfg(not(debug_assertions))]
#[proc_macro_attribute]
pub fn elapsed(_args: TokenStream, function_def: TokenStream) -> TokenStream {
  function_def
}

#[cfg(debug_assertions)]
#[proc_macro_attribute]
pub fn elapsed(_args: TokenStream, function_def: TokenStream) -> TokenStream {
  let mut item = syn::parse(function_def).unwrap();

  let fn_item = match &mut item {
    Item::Fn(fn_item) => fn_item,
    _ => panic!("elapsed proc macro expected a function"),
  };

  let ItemFn {
    attrs,
    vis,
    sig,
    block,
  } = fn_item;

  let function_body = block.clone();
  let fn_name = sig.ident.clone();

  let log_ms = format!("{{}}ms for \"{}\".", fn_name);

  let new_function_def = quote! {
    #(#attrs)* #vis #sig {
      let now = std::time::Instant::now();

      let mut wrapped_func = || #function_body;
      let res = wrapped_func();
      let name = #fn_name;

      let ms = now.elapsed().as_millis();

      if ms > #MIN_ELAPSED_MS {
        println!(#log_ms, now.elapsed().as_millis());
      }

      res
    }
  };

  TokenStream::from(new_function_def)
}
