use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input,
    ItemStruct,
    Meta,
    Lit
};


fn to_snake_case(text: String) -> String {
    text.replace(' ', "_").to_lowercase()
}


#[proc_macro_attribute]
pub fn table(_attribute: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input
    let input = parse_macro_input!(item as ItemStruct);

    // Get the struct name
    let struct_name = &input.ident;

    // Convert the struct name to snake case
    let table_name = to_snake_case(struct_name.to_string());

    // Get the visibility
    let vis = &input.vis;

    // Test if the table has any generics
    if !input.generics.params.is_empty() {
        panic!("A table may not have any generics!")
    }

    // Keep track of the primary key
    let mut primary_key = quote! {
        sql::Column {
            pk: true,
            unique: true,
            auto_increment: true,
            default: sql::SQLValue::Null,
            null: false,
            fk: false,
            m2m: false,
            name: String::from("id"),
            sql_name: String::from("id"),
            sql_type: sql::SQLType::Integer
        }
    };

    // Interpret the fields
    let mut columns = Vec::new();  // Columns of the table
    let mut fields = Vec::new();  // Fields of the struct
    for field in &input.fields {
        // Field name
        let name =  field.ident
            .clone()
            .expect("Unable to get name of field");

        // SQL type
        let sql_type = &field.ty;

        // Rust representation of SQL type
        let ty = match quote! { #sql_type }.to_string().as_str() {
            "Text" => quote! { String },
            "Blob" => quote! { Vec<u8> },
            "Boolean" => quote! { bool },
            "Bit" => quote! { bool },
            "Integer" => quote! { i32 },
            "Float" => quote! { f64 },
            "TimeStamp" => quote! { u128 },
            "Date" => quote! { u128 },
            "Time" => quote! { u128 },
            _ => panic!("Unknown type")
        };

        // Add the Rust representations of the SQL columns
        fields.push(
            quote! {
                pub #name: #ty
            }
        );

        // Save the attributes
        let mut pk = false;
        let mut unique = false;
        let mut auto_increment = false;
        let mut default = quote! {};
        let mut null = true;
        let mut fk = false;
        let mut m2m = false;
        let mut sql_name = to_snake_case(name.to_string());
        let column_name = name.to_string();

        // Interpret the attributes
        for attr in &field.attrs {
            // Primary key
            if attr.path().is_ident("primary_key") {
                pk = true;
            }

            // Unique
            if attr.path().is_ident("unique") {
                unique = true;
            }

            // Auto-increment
            if attr.path().is_ident("auto_increment") {
                auto_increment = true;
            }

            // Default
            if attr.path().is_ident("default") {
                // Parse the name
                match &attr.meta {
                    Meta::NameValue(nv) => {
                        if let syn::Expr::Lit(expr_lit) = &nv.value {
                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                let value = lit_str.value();

                                // Parse the default value
                                default = match quote! { #sql_type }.to_string().as_str() {
                                    // String
                                    "Text" => quote! { String::from(#value) },

                                    // Bytes
                                    "Blob" => panic!("Can not specify a default value for a blob"),

                                    // Boolean
                                    "Boolean" => {
                                        match value.as_str() {
                                            "0" | "false" | "False" => quote! { false },
                                            "1" | "true" | "True" => quote! { true },
                                            _ => panic!("A boolean value must be either \"0\", \"1\", \"true\", \"false\", \"True\" or \"False\"")
                                        }
                                    },

                                    // Bit
                                    "Bit" => {
                                        match value.as_str() {
                                            "0" => quote! { false },
                                            "1" => quote! { true },
                                            _ => panic!("A bit value must be either \"0\" or \"1\"")
                                        }
                                    },

                                    // Integer
                                    "Integer" => {
                                        match value.parse::<i64>() {
                                            Ok(i) => quote! { #i },
                                            Err(e) => panic!("Unable to parse integer: {e}")
                                        }
                                    },

                                    // Floating-point Integer
                                    "Float" => match value.parse::<f64>() {
                                        Ok(i) => quote! { #i },
                                        Err(e) => panic!("Unable to parse float: {e}")
                                    },

                                    // Timestamp
                                    "TimeStamp" => match value.parse::<u128>() {
                                        Ok(i) => quote! { #i },
                                        Err(e) => panic!("Unable to parse timestamp: {e}")
                                    },

                                    // Date
                                    "Date" => match value.parse::<u128>() {
                                        Ok(i) => quote! { #i },
                                        Err(e) => panic!("Unable to parse date: {e}")
                                    },

                                    // Time
                                    "Time" => match value.parse::<u128>() {
                                        Ok(i) => quote! { #i },
                                        Err(e) => panic!("Unable to parse time: {e}")
                                    },

                                    // Unknown
                                    _ => panic!("Unknown type")
                                };
                            } else {
                                panic!("Expected string literal for default. Even if it is a number, or a boolean, please, provide it in a string literal format, like this: #[default = \"5\"]")
                            }
                        } else {
                            panic!("Expected string literal for default. Even if it is a number, or a boolean, please, provide it in a string literal format, like this: #[default = \"5\"]")
                        }
                    },
                    _ => panic!("The default value has to be given in the following format: #[default = \"value\"]")
                }
            }

            // SQL name
            if attr.path().is_ident("sql_name") {
                // Parse the name
                match &attr.meta {
                    Meta::NameValue(nv) => {
                        if let syn::Expr::Lit(expr_lit) = &nv.value {
                            if let Lit::Str(lit_str) = &expr_lit.lit {
                                sql_name = lit_str.value();
                            } else {
                                panic!("Expected string literal for SQL name")
                            }
                        } else {
                            panic!("Expected string literal for SQL name")
                        }
                    },
                    _ => panic!("The SQL name has to be given in the following format: #[sql_name = \"name\"]")
                }
            }

            // Not null
            if attr.path().is_ident("not_null") {
                null = false;
            }

            // Foreign key
            if attr.path().is_ident("foreign_key") {
                fk = true;
            }

            // Many-to-many field
            if attr.path().is_ident("many_to_many") {
                m2m = true;
            }
        }

        // If the default value is empty, provide pre-defined default values
        if default.is_empty() {
            default = match quote! { #sql_type }.to_string().as_str() {
                "Text" => quote! { "String" },
                "Blob" => quote! { Vec::new() },
                "Boolean" => quote! { false },
                "Bit" => quote! { false },
                "Integer" => quote! { 0 },
                "Float" => quote! { 0.0 },
                "TimeStamp" => quote! { 0 },
                "Date" => quote! { 0 },
                "Time" => quote! { 0 },
                _ => panic!("Unknown type")
            };
        }

        // Create the SQL column
        let column = quote! {
            sql::Column {
                pk: #pk,
                unique: #unique,
                auto_increment: #auto_increment,
                default: sql::SQLValue::#sql_type(#default),
                null: #null,
                fk: #fk,
                m2m: #m2m,
                name: String::from(#column_name),
                sql_name: String::from(#sql_name),
                sql_type: sql::SQLType::#sql_type
            }
        };

        match pk {
            true => primary_key = column,
            false => columns.push(column)
        }
    }

    // Write the struct and it's implementations
    let expanded = quote! {
        #vis struct #struct_name {
            #(#fields),*
        }

        impl #struct_name {
            pub const fn table_name() -> &'static str {
                #table_name
            }

            /*pub fn remove(&self, db: &mut sql::Database) {
                db.execute(
                    format!("DELETE FROM {table_name} WHERE {pk_column} = {pk}",
                        table_name=Self::table_name(),
                        pk_column=Self::pk_column(),
                        pk=self.pk()
                    )
                );
            }*/

            pub fn table() -> sql::Table {
                sql::Table {
                    name: String::from(#table_name ),
                    primary_key: #primary_key,
                    columns: {
                        let mut vector = Vec::new();
                        #(vector.push(#columns));* ;
                        vector
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}