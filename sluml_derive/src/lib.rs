use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Generics, Ident, Type};

mod vec;

#[proc_macro_derive(IndexMut)]
pub fn index_mut_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (name, data, inner_type, generics) = vec::parse_derive_input(input);
    
    vec::gen_index_mut_derive(name, data, inner_type, generics).into()
}

#[proc_macro_derive(Add)]
pub fn add_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (name, data, _, generics) = vec::parse_derive_input(input);

    vec::gen_add_derive(name, data, generics).into()
}

#[proc_macro_derive(Sub)]
pub fn sub_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (name, data, _, generics) = vec::parse_derive_input(input);
    
    vec::gen_sub_derive(name, data, generics).into()
}

#[proc_macro_derive(Mul)]
pub fn mul_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (name, data, inner_type, generics) = vec::parse_derive_input(input);
    
    vec::gen_mul_derive(name, data, inner_type, generics).into()
}

#[proc_macro_derive(Div)]
pub fn div_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (name, data, inner_type, generics) = vec::parse_derive_input(input);
    
    vec::gen_div_derive(name, data, inner_type, generics).into()
}
