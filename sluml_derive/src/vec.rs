use super::*;

pub fn parse_derive_input(input: DeriveInput) -> (Ident, DataStruct, Type, Generics) {
    let Data::Struct(data) = input.data else {
        panic!()
    };

    let mut inner_type: Option<Type> = None;

    for field in data.fields.iter() {
        if inner_type.is_none() {
            inner_type = Some(field.ty.clone())
        } else if *inner_type.as_ref().unwrap() != field.ty {
            panic!("All members must be the same type.")
        }
    }

    (input.ident, data, inner_type.unwrap(), input.generics)
}

pub fn gen_index_mut_derive(name: Ident, data: DataStruct, inner_type: Type, generics: Generics) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut match_arms = TokenStream2::new();
    let mut mut_match_arms = TokenStream2::new();
    for (i, field) in data.fields.iter().enumerate() {
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
        impl #impl_generics ::std::ops::Index<usize> for #name #ty_generics #where_clause {
            type Output = #inner_type;

            fn index(&self, index: usize) -> &Self::Output {
                match index {
                    #match_arms
                    _ => panic!("Tried to index non-existent vector component.")
                }
            }
        }

        impl #impl_generics ::std::ops::IndexMut<usize> for #name #ty_generics #where_clause  {
            fn index_mut(&mut self, index: usize) -> &mut Self::Output {
                match index {
                    #mut_match_arms
                    _ => panic!("Tried to index non-existent vector component.")
                }
            }
        }
    )
}

pub fn gen_add_derive(name: Ident, data: DataStruct, generics: Generics) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut add_params = TokenStream2::new();
    for field in data.fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();

        quote!(
            #field_ident: self.#field_ident + rhs.#field_ident,
        )
        .to_tokens(&mut add_params);
    }

    quote!(
        impl #impl_generics ::std::ops::Add for #name #ty_generics #where_clause {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                Self {
                    #add_params
                }
            }
        }

        impl #impl_generics ::std::ops::AddAssign for #name #ty_generics #where_clause {
            fn add_assign(&mut self, rhs: Self) {
                *self = self.add(rhs)
            }
        }
    )
}

pub fn gen_sub_derive(name: Ident, data: DataStruct, generics: Generics) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut sub_params = TokenStream2::new();
    for field in data.fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();

        quote!(
            #field_ident: self.#field_ident - rhs.#field_ident,
        )
        .to_tokens(&mut sub_params);
    }

    quote!(
        impl #impl_generics ::std::ops::Sub for #name #ty_generics #where_clause {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                Self {
                    #sub_params
                }
            }
        }

        impl #impl_generics ::std::ops::SubAssign for #name #ty_generics #where_clause {
            fn sub_assign(&mut self, rhs: Self) {
                *self = self.sub(rhs)
            }
        }
    )
}

pub fn gen_mul_derive(name: Ident, data: DataStruct, inner_type: Type, generics: Generics) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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
        impl #impl_generics ::std::ops::Mul for #name #ty_generics #where_clause {
            type Output = Self;

            fn mul(self, rhs: Self) -> Self {
                Self {
                    #mul_params
                }
            }
        }

        impl #impl_generics ::std::ops::MulAssign for #name #ty_generics #where_clause {
            fn mul_assign(&mut self, rhs: Self) {
                *self = self.mul(rhs)
            }
        }

        impl #impl_generics ::std::ops::Mul<#inner_type> for #name #ty_generics #where_clause {
            type Output = Self;

            fn mul(self, rhs: #inner_type) -> Self {
                Self {
                    #mul_inner_params
                }
            }
        }

        impl #impl_generics ::std::ops::MulAssign<#inner_type> for #name #ty_generics #where_clause {
            fn mul_assign(&mut self, rhs: #inner_type) {
                *self = self.mul(rhs)
            }
        }
    )
}

pub fn gen_div_derive(name: Ident, data: DataStruct, inner_type: Type, generics: Generics) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

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
        impl #impl_generics ::std::ops::Div for #name #ty_generics #where_clause {
            type Output = Self;

            fn div(self, rhs: Self) -> Self {
                Self {
                    #div_params
                }
            }
        }

        impl #impl_generics ::std::ops::DivAssign for #name #ty_generics #where_clause {
            fn div_assign(&mut self, rhs: Self) {
                *self = self.div(rhs)
            }
        }

        impl #impl_generics ::std::ops::Div<#inner_type> for #name #ty_generics #where_clause {
            type Output = Self;

            fn div(self, rhs: #inner_type) -> Self {
                Self {
                    #div_inner_params
                }
            }
        }

        impl #impl_generics ::std::ops::DivAssign<#inner_type> for #name #ty_generics #where_clause {
            fn div_assign(&mut self, rhs: #inner_type) {
                *self = self.div(rhs)
            }
        }
    )
}

pub fn gen_neg_derive(name: Ident, data: DataStruct, generics: Generics) -> TokenStream2 {
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let mut neg_params = TokenStream2::new();
    for field in data.fields.iter() {
        let field_ident = field.ident.as_ref().unwrap();

        quote!(
            #field_ident: -self.#field_ident,
        )
        .to_tokens(&mut neg_params);
    }

    quote!(
        impl #impl_generics ::std::ops::Neg for #name #ty_generics #where_clause {
            type Output = Self;

            fn neg(self) -> Self {
                Self {
                    #neg_params
                }
            }
        }
    )
}
