
extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;


#[macro_use(c)]
extern crate cute;

#[proc_macro_derive(EnumTools)]
pub fn enum_tools(input: TokenStream) -> TokenStream {
    let source = input.to_string();

    let ast = syn::parse_derive_input(&source).unwrap();
    let expanded = expand_enum_tools(&ast);

    expanded.parse().unwrap()
}


fn expand_enum_tools(ast: &syn::DeriveInput) -> quote::Tokens {
    let variants = match ast.body {
        syn::Body::Enum(ref variants) => variants,
        syn::Body::Struct(_) => panic!("#[derive(EnumTools)] can only be used with enums"),
    };

    let enum_name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let vis = &ast.vis;

    let mut fns = Vec::new();

    for variant in variants {
        let variant_name = &variant.ident;
        let variant_path = quote!{ #enum_name :: #variant_name }; 

        let is_variant = quote::Ident::new(format!("is_{}", variant_name));

        match variant.data {
            syn::VariantData::Tuple(ref v) => {
                let syms = &c![quote::Ident::new(format!("v{}", i)), for i in 0..v.len()];

                fns.push(quote! {
                    #[allow(unreachable_patterns)]
                    #vis fn #variant_name (self) -> ( #(#v),* ) {
                        match self {
                            #variant_path  ( #(#syms),* ) => ( #(#syms),* ),
                            _ => panic!(),
                        }
                    }
                });

                fns.push(quote!{
                    #[allow(unreachable_patterns)]
                    #vis fn #is_variant (&self) -> bool {
                        match *self { #variant_path (..) => true, _ => false, }
                    }
                });

            },
            syn::VariantData::Struct(_) => unimplemented!(),
            syn::VariantData::Unit => {
                fns.push(quote!{
                    #[allow(unreachable_patterns)]
                    #vis fn #is_variant (&self) -> bool {
                        match *self { #variant_path => true, _ => false, }
                    }
                });
            },
        };
    }

    quote! {
        #[allow(non_snake_case, dead_code)]
        impl #impl_generics #enum_name  #ty_generics #where_clause {
            #(#fns)* 
        }
    }
}






