use super::*;

pub fn parse_derive_input(input: &DeriveInput) -> (&DataStruct, &Type) {
    let Data::Struct(ref data) = input.data else {
        panic!("Must be called on struct.")
    };

    let mut inner_type: Option<&Type> = None;

    for field in data.fields.iter() {
        if inner_type.is_none() {
            inner_type = Some(&field.ty)
        } else if *inner_type.unwrap() != field.ty {
            panic!("All members must be the same type.")
        }
    }

    (data, inner_type.unwrap())
}

pub fn gen_index_mut_impls(
    name: &Ident,
    data_struct: &DataStruct,
    inner_type: &Type,
) -> TokenStream2 {
    let mut match_arms = TokenStream2::new();
    let mut mut_match_arms = TokenStream2::new();
    for (i, field) in data_struct.fields.iter().enumerate() {
        let field_ident = field.ident.as_ref().unwrap();

        quote!(
            #i => &self.#field_ident,
        )
        .to_tokens(&mut match_arms);

        quote!(
            #i => &mut self.#field_ident,
        )
        .to_tokens(&mut mut_match_arms);
    }

    quote!(
        impl<#inner_type> ::std::ops::Index<usize> for #name<#inner_type> {
            type Output = #inner_type;

            fn index(&self, index: usize) -> &#inner_type {
                match index {
                    #match_arms
                    _ => panic!("Tried to index a non-existent vector component.")
                }
            }
        }

        impl<#inner_type> ::std::ops::IndexMut<usize> for #name<#inner_type> {
            fn index_mut(&mut self, index: usize) -> &mut #inner_type {
                match index {
                    #mut_match_arms
                    _ => panic!("Tried to index a non-existent vector component.")
                }
            }
        }
    )
}

pub fn gen_add_impls(name: &Ident, data: &DataStruct, inner_type: &Type) -> TokenStream2 {
    let mut add_params = TokenStream2::new();
    for field in data.fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();

        quote!(
            #field_ident: self.#field_ident + rhs.#field_ident,
        )
        .to_tokens(&mut add_params);
    }

    quote!(
        impl<#inner_type: Copy + ::std::ops::Add<Output = #inner_type>> ::std::ops::Add for #name<#inner_type> {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self {
                    #add_params
                }
            }
        }

        impl<#inner_type: Copy + ::std::ops::Add<Output = #inner_type>> ::std::ops::AddAssign for #name<#inner_type> {
            fn add_assign(&mut self, rhs: Self) {
                *self = self.add(rhs)
            }
        }
    )
}

pub fn gen_sub_impls(name: &Ident, data: &DataStruct, inner_type: &Type) -> TokenStream2 {
    let mut sub_params = TokenStream2::new();
    for field in data.fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();

        quote!(
            #field_ident: self.#field_ident - rhs.#field_ident,
        )
        .to_tokens(&mut sub_params);
    }

    quote!(
        impl<#inner_type: Copy + ::std::ops::Sub<Output = #inner_type>> ::std::ops::Sub for #name<#inner_type> {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self {
                    #sub_params
                }
            }
        }

        impl<#inner_type: Copy + ::std::ops::Sub<Output = #inner_type>> ::std::ops::SubAssign for #name<#inner_type> {
            fn sub_assign(&mut self, rhs: Self) {
                *self = self.sub(rhs)
            }
        }
    )
}

pub fn gen_mul_impls(
    name: &Ident,
    data: &DataStruct,
    inner_type: &Type,
) -> TokenStream2 {
    let mut mul_params = TokenStream2::new();
    let mut mul_inner_params = TokenStream2::new();
    for field in data.fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();

        quote!(
            #field_ident: self.#field_ident * rhs.#field_ident,
        )
        .to_tokens(&mut mul_params);

        quote!(
            #field_ident: self.#field_ident * rhs,
        )
        .to_tokens(&mut mul_inner_params);
    }

    quote!(
        impl<#inner_type: Copy + ::std::ops::Mul<Output = #inner_type>> ::std::ops::Mul for #name<#inner_type> {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self {
                    #mul_params
                }
            }
        }

        impl<#inner_type: Copy + ::std::ops::Mul<Output = #inner_type>> ::std::ops::MulAssign for #name<#inner_type> {
            fn mul_assign(&mut self, rhs: Self) {
                *self = self.mul(rhs)
            }
        }

        impl<#inner_type: Copy + ::std::ops::Mul<Output = #inner_type>> ::std::ops::Mul<#inner_type> for #name<#inner_type> {
            type Output = Self;

            fn mul(self, rhs: #inner_type) -> Self {
                Self {
                    #mul_inner_params
                }
            }
        }

        impl<#inner_type: Copy + ::std::ops::Mul<Output = #inner_type>> ::std::ops::MulAssign<#inner_type> for #name<#inner_type> {
            fn mul_assign(&mut self, rhs: #inner_type) {
                *self = self.mul(rhs)
            }
        }
    )
}

pub fn gen_div_impls(
    name: &Ident,
    data: &DataStruct,
    inner_type: &Type,
) -> TokenStream2 {
    let mut div_params = TokenStream2::new();
    let mut div_inner_params = TokenStream2::new();
    for field in data.fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();

        quote!(
            #field_ident: self.#field_ident / rhs.#field_ident,
        )
        .to_tokens(&mut div_params);

        quote!(
            #field_ident: self.#field_ident / rhs,
        )
        .to_tokens(&mut div_inner_params);
    }

    quote!(
        impl<#inner_type: Copy + ::std::ops::Div<Output = #inner_type>> ::std::ops::Div for #name<#inner_type> {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                Self {
                    #div_params
                }
            }
        }

        impl<#inner_type: Copy + ::std::ops::Div<Output = #inner_type>> ::std::ops::DivAssign for #name<#inner_type> {
            fn div_assign(&mut self, rhs: Self) {
                *self = self.div(rhs)
            }
        }

        impl<#inner_type: Copy + ::std::ops::Div<Output = #inner_type>> ::std::ops::Div<#inner_type> for #name<#inner_type> {
            type Output = Self;

            fn div(self, rhs: #inner_type) -> Self {
                Self {
                    #div_inner_params
                }
            }
        }

        impl<#inner_type: Copy + ::std::ops::Div<Output = #inner_type>> ::std::ops::DivAssign<#inner_type> for #name<#inner_type> {
            fn div_assign(&mut self, rhs: #inner_type) {
                *self = self.div(rhs)
            }
        }
    )
}

pub fn gen_neg_impls(name: &Ident, data: &DataStruct, inner_type: &Type) -> TokenStream2 {
    let mut neg_params = TokenStream2::new();
    for field in data.fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();

        quote!(
            #field_ident: -self.#field_ident,
        )
        .to_tokens(&mut neg_params);
    }

    quote!(
        impl<#inner_type: Copy + ::std::ops::Neg<Output = #inner_type>> ::std::ops::Neg for #name<#inner_type> {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    #neg_params
                }
            }
        }
    )
}

pub fn gen_dot_impls(
    name: &Ident,
    data: &DataStruct,
    inner_type: &Type,
) -> TokenStream2 {
    let mut dot_expr = TokenStream2::new();
    for (i, field) in data.fields.iter().enumerate() {
        let field_ident = field.ident.as_ref().unwrap();

        if i != 0 {
            quote!(+).to_tokens(&mut dot_expr);
        }

        quote!(
            self.#field_ident * rhs.#field_ident
        )
        .to_tokens(&mut dot_expr);
    }

    quote!(
        impl<#inner_type: Copy + ::std::ops::Add<Output = #inner_type> + ::std::ops::Mul<Output = #inner_type>> crate::vec::Dot for #name<#inner_type> {
            type Output = #inner_type;

            fn dot(self, rhs: Self) -> #inner_type {
                #dot_expr
            }
        }
    )
}
