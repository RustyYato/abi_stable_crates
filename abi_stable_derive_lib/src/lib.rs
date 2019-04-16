/*!
An implementation detail of abi_stable.
*/

#![recursion_limit="192"]
//#![deny(unused_variables)]


extern crate core_extensions;

extern crate proc_macro;

#[macro_use]
mod macros;

mod arenas;
mod attribute_parsing;
mod common_tokens;
mod constants;
mod ignored_wrapper;
mod datastructure;
mod fn_pointer_extractor;

mod lifetimes;
mod stable_abi;
mod to_token_fn;

use proc_macro::TokenStream as TokenStream1;
use proc_macro2::TokenStream as TokenStream2;

use syn::{DeriveInput,ItemFn};

use quote::{quote, ToTokens};

#[allow(unused_imports)]
use core_extensions::prelude::*;

#[allow(unused_imports)]
use crate::{
    arenas::{AllocMethods, Arenas},
    common_tokens::CommonTokens,
};

/// Mangles the name of the function that returns a library's functions/statics,
/// so that one does not accidentally load
/// dynamic libraries that use incompatible versions of abi_stable
#[doc(hidden)]
pub fn mangle_library_getter_ident<S>(s:S)->String
where S: ::std::fmt::Display
{
    use core_extensions::StringExt;

    let major=env!("CARGO_PKG_VERSION_MAJOR").parse::<u32>().unwrap();
    let minor=env!("CARGO_PKG_VERSION_MINOR").parse::<u32>().unwrap();

    let unmangled=if major==0 {
        format!("_as_mod.{}.vn.minor.{}",s,minor)
    }else{
        format!("_as_mod.{}.vn.major.{}",s,major)
    };

    let mut mangled=String::with_capacity(unmangled.len()*3/2);

    for kv in unmangled.split_while(|c| c.is_alphanumeric() ) {
        if kv.key {
            mangled.push_str(kv.str);
            continue
        }
        for c in kv.str.chars() {
            mangled.push_str(match c {
                '.'=>"_0",
                '_'=>"_1",
                '-'=>"_2",
                '<'=>"_3",
                '>'=>"_4",
                '('=>"_5",
                ')'=>"_6",
                '['=>"_7",
                ']'=>"_8",
                '{'=>"_9",
                '}'=>"_a",
                ' '=>"_b",
                ','=>"_c",
                ':'=>"_d",
                ';'=>"_e",
                '!'=>"_f",
                '#'=>"_g",
                '$'=>"_h",
                '%'=>"_i",
                '/'=>"_j",
                '='=>"_k",
                '?'=>"_l",
                '¿'=>"_m",
                '¡'=>"_o",
                '*'=>"_p",
                '+'=>"_q",
                '~'=>"_r",
                '|'=>"_s",
                '°'=>"_t",
                '¬'=>"_u",
                '\''=>"_x",
                '\"'=>"_y",
                '`'=>"_z",
                c=>panic!("cannot currently mangle the '{}' character.", c),
            });
        }
    }

    mangled
}


#[doc(hidden)]
pub fn derive_stable_abi(input: TokenStream1) -> TokenStream1 {
    measure!({
        let input = syn::parse::<DeriveInput>(input).unwrap();
        // println!("deriving StableAbi for {}",input.ident);
        stable_abi::derive(input).into()
    })
}

#[doc(hidden)]
pub fn derive_stable_abi_from_str(s: &str) -> TokenStream2 {
    let input = syn::parse_str::<DeriveInput>(s).unwrap();
    stable_abi::derive(input)
}


#[doc(hidden)]
pub fn mangle_library_getter_attr(_attr: TokenStream1, item: TokenStream1) -> TokenStream1 {
    use syn::Ident;

    use proc_macro2::Span;
    

    measure!({
        let input = syn::parse::<ItemFn>(item).unwrap();
        
        let vis=&input.vis;
        let attrs=&input.attrs;
        let ret_ty=&input.decl.output;
        
        let original_fn_ident=&input.ident;

        let export_name=Ident::new(
            &mangle_library_getter_ident(&original_fn_ident),
            Span::call_site(),
        );

        quote!(
            #input

            #[no_mangle]
            #(#attrs)*
            #vis extern "C" fn #export_name() #ret_ty {
                let _: abi_stable::library::LibraryGetterFn<_> = #original_fn_ident;
                #original_fn_ident()
            }
        ).into()
    })
}

