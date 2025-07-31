use crate::codegen::generator::wire::rust::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::rust::spec_generator::codec::dco::encoder::ty::WireRustCodecDcoGeneratorEncoderTrait;

impl WireRustCodecDcoGeneratorEncoderTrait for GenericWireRustCodecDcoGenerator<'_> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        match &*self.mir.base_type {
            crate::codegen::ir::mir::ty::MirType::StructRef(struct_ref) => {
                Some(generate_impl_into_dart_struct_generic(
                    &self.mir,
                    struct_ref,
                    self.context.mir_pack,
                ))
            }
            crate::codegen::ir::mir::ty::MirType::EnumRef(enum_ref) => {
                Some(generate_impl_into_dart_enum_generic(
                    &self.mir,
                    enum_ref,
                    self.context.mir_pack,
                ))
            }
            _ => None,
        }
    }
}

fn generate_impl_into_dart_struct_generic(
    mir_generic: &crate::codegen::ir::mir::ty::generic::MirTypeGeneric,
    struct_ref: &crate::codegen::ir::mir::ty::structure::MirTypeStructRef,
    mir_pack: &crate::codegen::ir::mir::pack::MirPack,
) -> String {
    let type_params = mir_generic.type_parameters
        .iter()
        .map(|param| format!("{}: flutter_rust_bridge::IntoDart", param))
        .collect::<Vec<_>>()
        .join(", ");
    
    let struct_data = struct_ref.get(mir_pack);
    let struct_name = &struct_data.name.rust_style();
    let generic_params = if mir_generic.type_parameters.is_empty() {
        String::new()
    } else {
        format!("<{}>", mir_generic.type_parameters.join(", "))
    };

    let field_conversions = struct_data.fields
        .iter()
        .map(|field| {
            let field_name = &field.name.rust_style(false);
            format!("{}: self.{}.into_dart(),", field_name, field_name)
        })
        .collect::<Vec<_>>()
        .join("\n            ");

    format!(
        r#"impl<{}> flutter_rust_bridge::IntoDart for {}{} {{
    fn into_dart(self) -> flutter_rust_bridge::DartAbi {{
        [{}].into_dart()
    }}
}}"#,
        type_params,
        struct_name,
        generic_params,
        field_conversions
    )
}

fn generate_impl_into_dart_enum_generic(
    mir_generic: &crate::codegen::ir::mir::ty::generic::MirTypeGeneric,
    enum_ref: &crate::codegen::ir::mir::ty::enumeration::MirTypeEnumRef,
    mir_pack: &crate::codegen::ir::mir::pack::MirPack,
) -> String {
    let type_params = mir_generic.type_parameters
        .iter()
        .map(|param| format!("{}: flutter_rust_bridge::IntoDart", param))
        .collect::<Vec<_>>()
        .join(", ");
    
    let enum_data = enum_ref.get(mir_pack);
    let enum_name = &enum_data.name.rust_style();
    let generic_params = if mir_generic.type_parameters.is_empty() {
        String::new()
    } else {
        format!("<{}>", mir_generic.type_parameters.join(", "))
    };

    // Basic enum implementation - could be expanded based on variant types
    format!(
        r#"impl<{}> flutter_rust_bridge::IntoDart for {}{} {{
    fn into_dart(self) -> flutter_rust_bridge::DartAbi {{
        match self {{
            // Placeholder - would need to match specific variants
            _ => todo!("Generic enum into_dart conversion")
        }}
    }}
}}"#,
        type_params,
        enum_name,
        generic_params,
    )
}

impl WireRustCodecDcoGeneratorEncoderTrait for GenericRefWireRustCodecDcoGenerator<'_> {
    fn generate_impl_into_dart(&self) -> Option<String> {
        let base_generator = GenericWireRustCodecDcoGenerator::new(
            self.mir.generic_type.as_ref().clone(),
            self.context,
        );
        base_generator.generate_impl_into_dart()
    }
}
