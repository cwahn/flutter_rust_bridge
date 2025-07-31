use crate::library::codegen::generator::codec::sse::ty::*;

impl CodecSseTyTrait for GenericCodecSseTy<'_> {
    fn generate_encode(&self, _lang: &Lang) -> Option<String> {
        // For now, generic types don't support SSE encoding
        // This would need to be implemented properly later
        None
    }

    fn generate_decode(&self, _lang: &Lang) -> Option<String> {
        // For now, generic types don't support SSE decoding
        // This would need to be implemented properly later
        None
    }
}
