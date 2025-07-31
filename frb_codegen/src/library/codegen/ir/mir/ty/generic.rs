use crate::codegen::ir::mir::ty::{MirContext, MirType, MirTypeTrait};

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize)]
pub struct TypeConstraint {
    pub param: String,
    pub bounds: Vec<String>,
}

crate::mir! {
pub struct MirTypeGeneric {
    pub base_type: Box<MirType>,         // The underlying struct/enum without generics
    pub type_parameters: Vec<String>,    // ["T", "U", "V"]
    pub constraints: Vec<TypeConstraint>, // where T: Clone + Debug
}
}

impl MirTypeTrait for MirTypeGeneric {
    fn visit_children_types<F: FnMut(&MirType) -> bool>(
        &self,
        f: &mut F,
        mir_context: &impl MirContext,
    ) {
        self.base_type.visit_types(f, mir_context);
    }

    fn safe_ident(&self) -> String {
        format!(
            "generic_{}_{}", 
            self.base_type.safe_ident(),
            self.type_parameters.join("_")
        )
    }

    fn rust_api_type(&self) -> String {
        format!(
            "{}<{}>",
            self.base_type.rust_api_type(),
            self.type_parameters.join(", ")
        )
    }
}
