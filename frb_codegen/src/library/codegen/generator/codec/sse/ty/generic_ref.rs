use crate::library::codegen::generator::codec::sse::ty::*;

impl CodecSseTyTrait for GenericRefCodecSseTy<'_> {
    fn generate_encode(&self, _lang: &Lang) -> Option<String> {
        // GenericRef delegates to the underlying generic type
        None
    }

    fn generate_decode(&self, _lang: &Lang) -> Option<String> {
        // GenericRef delegates to the underlying generic type
        None
    }
}
