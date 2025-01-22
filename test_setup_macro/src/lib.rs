use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

#[proc_macro_attribute]
pub fn with_temp_db(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut ast = parse_macro_input!(item as ItemFn);

    // Insert the setup_db() call at the beginning of the function body
    let setup_db_call = quote! {
        let temp_db = ::tempfile::Builder::new()
            .suffix(".db")
            .prefix("test")
            .tempfile_in("dbtests")
            .unwrap();
    };
    ast.block
        .stmts
        .insert(0, syn::parse2::<syn::Stmt>(setup_db_call).unwrap());

    // Generate the modified function
    let gen = quote! {
        #[tokio::test]
        #ast
    };
    gen.into()
}
