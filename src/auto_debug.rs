use darling::{ast::Data, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Generics, Ident};

#[derive(Debug, FromDeriveInput)]
struct AutoDebugInfo {
    ident: Ident,
    generics: Generics,
    data: Data<(), AutoDebugFieldsInfo>,
}

#[derive(Debug, FromField)]
// #[darling(attributes(debug))] 这句话是重点，结构体字段上调用的使用 debug(skip) 这样传参数
#[darling(attributes(debug))]
struct AutoDebugFieldsInfo {
    ident: Option<Ident>,
    #[darling(default)]
    skip: bool,
}

pub(crate) fn process_auto_debug(input: DeriveInput) -> TokenStream {
    let AutoDebugInfo {
        ident,
        generics,
        data: Data::Struct(fields),
    } = AutoDebugInfo::from_derive_input(&input).expect("can not parse input")
    else {
        panic!("AutoDebug only works on structs");
    };

    // let mut fmt_str = vec![];
    // let mut fmt_args = vec![];
    // let mut has_name = false;

    // for (idx, field) in fields.iter().enumerate() {
    //     if field.skip {
    //         continue;
    //     }
    //     let ident = field.ident.as_ref();
    //     let name = match ident {
    //         Some(ident) => {
    //             has_name = true;
    //             ident.to_string()
    //         }
    //         None => idx.to_string(),
    //     };
    //     fmt_str.push(format!("{}: {{:?}}", name));
    //     fmt_args.push(quote! {
    //         &self.#name,
    //     });
    // }

    // let fmt_str = if has_name {
    //     format!("{} {{ {} }}", ident, fmt_str.join(", "))
    // } else {
    //     format!("{}({})", ident, fmt_str.join(", "))
    // };

    let fields = fields.iter().map(|field| {
        let ident = field.ident.as_ref().unwrap();
        if field.skip {
            quote! {}
        } else {
            quote! {
                .field(stringify!(#ident), &self.#ident)
            }
        }
    });

    quote! {
        #[automatically_derived]
        impl ::core::fmt::Debug for #ident #generics {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                f.debug_struct(stringify!(#ident))
                    #(#fields)*
                    .finish()
            }
        }
    }
    .into()
}

/*
#[automatically_derived]
#[allow(unused)]
impl ::core::fmt::Debug for RespBulkString {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field2_finish(
            f,
            "RespBulkString",
            "inner",
            &self.inner,
            "nothing",
            &&self.nothing,
        )
    }
}
*/
