use crate::library::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::*;

impl WireDartCodecCstGeneratorEncoderTrait for GenericWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        // Generic types not yet supported for CST encoding
        // Would need proper implementation
        Acc::new(|_| None)
    }
    
    fn dart_wire_type(&self, _target: Target) -> String {
        "Object".to_string()
    }
}

impl WireDartCodecCstGeneratorEncoderTrait for GenericRefWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        // GenericRef types not yet supported for CST encoding
        // Would need proper implementation
        Acc::new(|_| None)
    }
    
    fn dart_wire_type(&self, _target: Target) -> String {
        "Object".to_string()
    }
}
