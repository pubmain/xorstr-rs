extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use rand::{Rng, thread_rng};
use syn::{LitStr, parse::Parse, parse::ParseStream, parse_macro_input};

struct Args {
    input: LitStr,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Args {
            input: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn xorstr(input: TokenStream) -> TokenStream {
    let Args { input } = parse_macro_input!(input as Args);
    let value = input.value();

    let mut rng = rand::rng();
    let key_len = rng.random_range(5..10);
    let key: Vec<u8> = (0..key_len).map(|_| rng.random::<u8>()).collect();

    let encrypted: Vec<u8> = value
        .bytes()
        .enumerate()
        .map(|(i, b)| b ^ key[i % key_len])
        .collect();

    let encrypted_len = encrypted.len();

    let encrypted_lits = encrypted.iter().map(|b| quote! { #b });
    let key_lits = key.iter().map(|b| quote! { #b });

    quote! {
        {
            const fn xor(data: &[u8], key: &[u8]) -> [u8; #encrypted_len] {
                let mut out = [0u8; #encrypted_len];
                let mut i = 0;
                while i < data.len() {
                    out[i] = data[i] ^ key[i % #key_len];
                    i += 1;
                }
                out
            }

            const DATA: [u8; #encrypted_len] = [#(#encrypted_lits),*];
            const KEY: [u8; #key_len] = [#(#key_lits),*];
            const DECRYPTED: [u8; #encrypted_len] = xor(&DATA, &KEY);

            unsafe { ::core::str::from_utf8_unchecked(&DECRYPTED) }
        }
    }
    .into()
}
