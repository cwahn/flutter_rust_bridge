use crate::library::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;

impl WireDartCodecDcoGeneratorDecoderTrait for crate::library::codegen::generator::wire::dart::spec_generator::codec::dco::base::GenericWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        // Generic DCO decoding not yet supported
        "throw UnimplementedError('Generic DCO decoding not yet supported');".to_string()
    }
}

impl WireDartCodecDcoGeneratorDecoderTrait for crate::library::codegen::generator::wire::dart::spec_generator::codec::dco::base::GenericRefWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        // GenericRef DCO decoding not yet supported
        "throw UnimplementedError('GenericRef DCO decoding not yet supported');".to_string()
    }
}
