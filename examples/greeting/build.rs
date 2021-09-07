// build.rs

use marine_test_macro_impl::marine_test_impl2;

use proc_macro2::TokenStream;
use quote::quote;
use quote::ToTokens;

use std::env;
use std::fs;
use std::path::Path;
use std::string::ToString;

pub fn test_marine_test_token_streams(
    config_path: &str,
    modules_dir: &str,
) -> Option<TokenStream> {
    //let marine_item = stream_from_file(&marine_path);
    //let test_token_stream = quote::quote! { #marine_item };
    //let buf = marine_path.as_ref().to_path_buf();
    let attrs = quote::quote! {config_path = #config_path, modules_dir = #modules_dir};
    let marine_token_streams = marine_test_impl2(attrs)
        .unwrap_or_else(|e| panic!("failed to apply the marine macro due {}", e));

    //let expanded_item = items_from_file(&expanded_path);
    Some(marine_token_streams)
}

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("marine_test_env.rs");
    let tokens = test_marine_test_token_streams("Config.toml", "artifacts").unwrap();
    let tokens=tokens.to_string();
    if cfg!(test) {
        fs::write(
            &dest_path,&tokens
        ).unwrap();
    }


    println!("out_dir: {}", out_dir.to_str().unwrap());
    println!("code: {}", tokens);
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/main.rs");
}