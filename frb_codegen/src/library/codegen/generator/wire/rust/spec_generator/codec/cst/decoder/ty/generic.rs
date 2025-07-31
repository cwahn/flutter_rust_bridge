use crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::decoder::ty::WireRustCodecCstGeneratorDecoderTrait;
use crate::library::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::library::codegen::generator::misc::target::Target;
use std::borrow::Cow;

impl WireRustCodecCstGeneratorDecoderTrait for crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::base::GenericWireRustCodecCstGenerator<'_> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        None
    }
    
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc::new(|_| None)
    }
    
    fn generate_impl_decode_jsvalue_body(&self) -> Option<Cow<str>> {
        None
    }
    
    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        None
    }
    
    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc::new(|_| WireRustOutputCode::default())
    }
    
    fn rust_wire_type(&self, _target: Target) -> String {
        "todo!()".to_string()
    }
}

impl WireRustCodecCstGeneratorDecoderTrait for crate::library::codegen::generator::wire::rust::spec_generator::codec::cst::base::GenericRefWireRustCodecCstGenerator<'_> {
    fn generate_decoder_class(&self) -> Option<WireRustOutputCode> {
        None
    }
    
    fn generate_impl_decode_body(&self) -> Acc<Option<String>> {
        Acc::new(|_| None)
    }
    
    fn generate_impl_decode_jsvalue_body(&self) -> Option<Cow<str>> {
        None
    }
    
    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        None
    }
    
    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc::new(|_| WireRustOutputCode::default())
    }
    
    fn rust_wire_type(&self, _target: Target) -> String {
        "todo!()".to_string()
    }
}
