extern crate proc_macro;
use quote::quote;
use syn::{LitStr, parse::Parse, parse_macro_input};

struct Args {
    input: LitStr,
}

impl Parse for Args {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Args {
            input: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn xorstr(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let args = parse_macro_input!(input as Args);
    let key_len = rand::random_range(5..10);
    let mut key_bytes = vec![];
    for _ in 0..key_len {
        key_bytes.push(rand::random::<u8>());
    }
    let obfuscated_bytes = args
        .input
        .value()
        .bytes()
        .enumerate()
        .map(|(i, b)| b ^ key_bytes[i % key_bytes.len()])
        .collect::<Vec<_>>();

    quote! {
        {
            let data = [#(#obfuscated_bytes),*];
            let key = [#(#key_bytes),*];
            let decrypted = data.iter().enumerate()
                .map(|(i, b)| (b ^ key[i % key.len()]))
                .collect::<Vec<u8>>();
            String::from_utf8(decrypted).unwrap()
        }
    }
    .into()
}
