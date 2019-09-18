extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;
extern crate postgres;

use rust_orm_lib::{Column, Table, Relation};
use proc_macro::TokenStream;
use::syn::{parse_macro_input, DeriveInput, Type};
use::syn::Data::Struct;
use syn::export::TokenStream2;


#[proc_macro_derive(Relation)]
pub fn impl_get_table(input: TokenStream) -> TokenStream {
    // input: item deriving the trait (struct, enum, or union)
    // TokenStream: format in which the proc_macro library exposes the input
    // DeriveInput: format which the syn parser library needs

    // Parse the TokenStream into a DeriveInput
    let input = parse_macro_input!(input as DeriveInput);

    // Make sure that the macro input is a struct
    let fields = match input.data {
        Struct(s) => s.fields,
        _ => panic!()
    };

    // let mut stdout = stdout();

    /* Generate code to make a Table struct */
    let struct_name = input.ident;
    let mut construct_table_code: Vec<TokenStream2> = Vec::new();

    // Generate code to initialize table
    construct_table_code.push(quote! {
        let mut table = Table {
            name: #struct_name
            columns: Vec::new()
        }
    });

    // Generate code to add columns
    for field in fields.into_iter() {
        // extract the field name and type from the struct
        let field_name = field.ident.expect("field must have name").to_string();
        let field_type = field.ty;

        // use them to create the column
        let add_column_code = quote! {
            table.columns.push(Column {
                name: #field_name,
                typ: #field_type,
            })
        };
        construct_table_code.push(add_column_code);
        // writeln!(&mut stdout, "{}", field_name);
    }

    construct_table_code.push(quote! {
        return table;
    });


    let impl_relation = quote! {
        impl Relation for #struct_name {
            fn get_table(&self) -> Table {
                #(#construct_table_code) *
            }
        }
    };

    proc_macro::TokenStream::from(impl_relation)
}