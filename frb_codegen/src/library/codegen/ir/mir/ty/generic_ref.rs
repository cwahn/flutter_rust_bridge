use crate::codegen::ir::mir::ty::generic::MirTypeGeneric;
use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

crate::mir! {
pub struct MirTypeGenericRef {
    pub generic_type: Box<MirTypeGeneric>,  // Reference to the generic type definition
    pub type_arguments: Vec<MirType>,       // [MirType::Primitive(String), MirType::I32]
}
}

impl MirTypeTrait for MirTypeGenericRef {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.generic_type.base_type.visit_types(f, mir_context);
        for arg in &self.type_arguments {
            arg.visit_types(f, mir_context);
        }
    }

    fn safe_ident(&self) -> String {
        format!(
            "{}_{}", 
            self.generic_type.safe_ident(),
            self.type_arguments.iter()
                .map(|arg| arg.safe_ident())
                .collect::<Vec<_>>()
                .join("_")
        )
    }

    fn rust_api_type(&self) -> String {
        format!(
            "{}<{}>",
            self.generic_type.base_type.rust_api_type(),
            self.type_arguments.iter()
                .map(|arg| arg.rust_api_type())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}
