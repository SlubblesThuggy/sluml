use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Ident, Type};

mod vec;

#[proc_macro_derive(IndexMut)]
pub fn index_mut_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (data_struct, inner_type) = vec::parse_derive_input(&input);

    vec::gen_index_mut_impls(&input.ident, data_struct, inner_type).into()
}

#[proc_macro_derive(Add)]
pub fn add_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (data_struct, inner_type) = vec::parse_derive_input(&input);

    vec::gen_add_impls( &input.ident, data_struct, inner_type).into()
}

#[proc_macro_derive(Sub)]
pub fn sub_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (data_struct, inner_type) = vec::parse_derive_input(&input);

    vec::gen_sub_impls( &input.ident, data_struct, inner_type).into()
}

#[proc_macro_derive(Mul)]
pub fn mul_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (data_struct, inner_type) = vec::parse_derive_input(&input);

    vec::gen_mul_impls( &input.ident, data_struct, inner_type).into()
}

#[proc_macro_derive(Div)]
pub fn div_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (data_struct, inner_type) = vec::parse_derive_input(&input);

    vec::gen_div_impls( &input.ident, data_struct, inner_type).into()
}

#[proc_macro_derive(Neg)]
pub fn neg_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (data_struct, inner_type) = vec::parse_derive_input(&input);

    vec::gen_neg_impls( &input.ident, data_struct, inner_type).into()
}

#[proc_macro_derive(Dot)]
pub fn dot_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let (data_struct, inner_type) = vec::parse_derive_input(&input);

    vec::gen_dot_impls( &input.ident, data_struct, inner_type).into()
}
