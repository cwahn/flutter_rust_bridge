use crate::library::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::library::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::library::utils::namespace::Namespace;

impl WireRustGeneratorMiscTrait for crate::library::codegen::generator::wire::rust::spec_generator::base::GenericWireRustGenerator<'_> {
    fn wrapper_struct_name(&self) -> Option<String> {
        None
    }
    
    fn generate_static_checks(&self) -> Option<String> {
        None
    }
    
    fn generate_imports(&self) -> Option<Vec<Namespace>> {
        None
    }
    
    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc::new(|_| WireRustOutputCode::default())
    }
    
    fn generate_wire_func_call_decode_wrapper(&self) -> Option<String> {
        None
    }
    
    fn generate_wire_func_call_decode_type(&self) -> Option<String> {
        None
    }
}

impl WireRustGeneratorMiscTrait for crate::library::codegen::generator::wire::rust::spec_generator::base::GenericRefWireRustGenerator<'_> {
    fn wrapper_struct_name(&self) -> Option<String> {
        None
    }
    
    fn generate_static_checks(&self) -> Option<String> {
        None
    }
    
    fn generate_imports(&self) -> Option<Vec<Namespace>> {
        None
    }
    
    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        Acc::new(|_| WireRustOutputCode::default())
    }
    
    fn generate_wire_func_call_decode_wrapper(&self) -> Option<String> {
        None
    }
    
    fn generate_wire_func_call_decode_type(&self) -> Option<String> {
        None
    }
}
