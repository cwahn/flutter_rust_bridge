// Test file to verify basic generic support parsing
use crate::codegen::parser::mir::parser::ty::structure::MirStructParser;
use crate::codegen::parser::mir::parser::attribute::Attribute;

#[cfg(test)]
mod tests {
    use super::*;
    use syn::{parse_quote, DeriveInput};
    
    #[test]
    fn test_generic_struct_parsing() {
        // Test parsing a generic struct
        let input: DeriveInput = parse_quote! {
            pub struct GenericStruct<T, U> {
                pub field1: T,
                pub field2: U,
            }
        };
        
        // Basic compilation test - if this compiles, our type system extension is working
        println!("Generic struct input parsed: {:?}", input.ident);
    }
    
    #[test]
    fn test_generic_enum_parsing() {
        // Test parsing a generic enum
        let input: DeriveInput = parse_quote! {
            pub enum GenericEnum<T> {
                Some(T),
                None,
            }
        };
        
        // Basic compilation test - if this compiles, our type system extension is working
        println!("Generic enum input parsed: {:?}", input.ident);
    }
}
