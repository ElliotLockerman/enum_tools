
extern crate proc_macro;
use proc_macro::TokenStream;

extern crate syn;

#[macro_use]
extern crate quote;


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
        let getter = &variant.ident;

        let data = match variant.data {
            syn::VariantData::Tuple(ref v) => v,
            syn::VariantData::Struct(_) => unimplemented!(),
            syn::VariantData::Unit => continue,
        };

        // let ref_getter = getter.to_string() + "_ref";
        // let ref_getter_type = "&".to_string() + &getter_type.to_string();

        let variant_name = quote! {#enum_name :: #getter };

        let new = quote! {
            #vis fn #getter (self) -> ( #(#data),* ) {
                if let #variant_name (v) = self { v } else { panic!() }
            }
        };

        fns.push(new);
    }

    quote! {
        #[allow(non_snake_case, dead_code)]
        impl #impl_generics #enum_name  #ty_generics #where_clause {
            #(#fns)* 
        }
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}




