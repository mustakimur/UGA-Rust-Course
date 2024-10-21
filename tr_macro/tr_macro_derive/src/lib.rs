extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

use syn::{parse_macro_input, ItemFn, LitStr};

#[proc_macro_attribute]
pub fn do_copy(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input as a function
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;  // Get the function name
    let data = &input.block;    // Get the function's input data

    // Generate new code with the original function + additional logic
    let output = quote! {
        fn #fn_name() {
            println!("Calling function: {}", stringify!(#fn_name));
            // Call the original function's body
            #data
        }
    };

    // Convert the generated code back into a TokenStream
    output.into()
}

#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
    // Parse the input as a string literal
    let query = parse_macro_input!(input as LitStr);

    // Generate Rust code that prints the SQL query
    let expanded = quote! {
        {
            let query = #query;
            println!("Executing SQL Query: {}", query);
            query  // Return the query string
        }
    };

    // Convert the generated code into a TokenStream
    expanded.into()
}
