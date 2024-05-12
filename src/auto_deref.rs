use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident, Type};

#[derive(Debug, FromDeriveInput)]
// #[darling(attributes(deref))] 这句话是重点，结构体上调用的使用 deref(mutable=true, field="inner") 这样传参数
#[darling(attributes(deref))]
struct AutoDerefInfo {
    ident: Ident,
    generics: Generics,
    data: Data<(), AutoDerefFieldsInfo>,
    #[darling(default)]
    mutable: bool,
    #[darling(default)]
    field: Option<Ident>,
}

#[derive(Debug, FromField)]
struct AutoDerefFieldsInfo {
    ident: Option<Ident>,
    ty: Type,
}

pub(crate) fn process_auto_deref(input: DeriveInput) -> TokenStream {
    let AutoDerefInfo {
        ident,
        generics,
        data: Data::Struct(fields),
        mutable,
        field,
    } = AutoDerefInfo::from_derive_input(&input).expect("can not parse input")
    else {
        panic!("AutoDeref only works on structs");
    };
    let (fd, ty) = match field {
        Some(field) => match fields.iter().find(|f| f.ident.as_ref().unwrap() == &field) {
            Some(f) => (f.ident.as_ref().unwrap().clone(), &f.ty),
            None => panic!("field {:?} not found in the data structure", field),
        },
        None => {
            // if noly 1 field, use that field
            if fields.len() != 1 {
                panic!("AutoDeref only works on structs with 1 field or with field attribute")
            }
            let f = fields.iter().next().unwrap();
            (f.ident.as_ref().unwrap().clone(), &f.ty)
        }
    };

    let mut code = vec![quote! {
        impl #generics std::ops::Deref for #ident #generics {
            type Target = #ty;

            fn deref(&self) -> &Self::Target {
                &self.#fd
            }
        }
    }];

    if mutable {
        code.push(quote! {
            impl #generics std::ops::DerefMut for #ident #generics {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.#fd
                }
            }
        });
    }

    quote! {
        #(#code)*
    }
    .into()
}
