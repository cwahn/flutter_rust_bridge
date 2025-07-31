use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::base::*;
use crate::codegen::ir::mir::field::MirField;
use crate::codegen::ir::mir::ty::enumeration::{MirEnumVariant, MirVariantKind};
use crate::codegen::generator::wire::dart::spec_generator::codec::cst::encoder::ty::WireDartCodecCstGeneratorEncoderTrait;
use crate::codegen::ir::mir::ty::generic::MirTypeGeneric;
use crate::codegen::ir::mir::ty::MirType;

impl WireDartCodecCstGeneratorEncoderTrait for GenericWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        match &*self.mir.base_type {
            MirType::StructRef(struct_ref) => {
                let struct_data = struct_ref.get(self.context.mir_pack);
                self.generate_encode_func_body_struct(&struct_data.fields)
            },
            MirType::EnumRef(enum_ref) => {
                let enum_data = enum_ref.get(self.context.mir_pack);
                self.generate_encode_func_body_enum(&enum_data.variants)
            },
            _ => Acc::default(),
        }
    }

    fn dart_wire_type(&self, target: Target) -> String {
        match target {
            Target::Io => format!("ffi.Pointer<wire_{}>", self.context.mir_pack.get_struct_ref_str(&MirType::Generic(self.mir.clone()))),
            Target::Web => "JSAny".to_string(),
        }
    }
}

impl GenericWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body_struct(&self, fields: &[MirField]) -> Acc<Option<String>> {
        let field_encoders = fields
            .iter()
            .enumerate()
            .map(|(index, field)| {
                let field_name = &field.name.rust_style(false);
                format!(
                    "ans.ref.{} = {};",
                    field_name,
                    self.generate_field_encoder(&field.ty, field_name)
                )
            })
            .collect::<Vec<_>>()
            .join("\n  ");

        let struct_name = self.context.mir_pack.get_struct_ref_str(&MirType::Generic(self.mir.clone()));
        Acc {
            io: Some(format!(
                r#"final ans = inner.new_{}();
  {}
  return ans;"#,
                struct_name, field_encoders
            )),
            web: Some(format!(
                r#"return {{ {} }};"#,
                fields
                    .iter()
                    .map(|field| {
                        let field_name = &field.name.rust_style(false);
                        format!("'{}': {}.cstEncode()", field_name, field_name)
                    })
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            ..Default::default()
        }
    }

    fn generate_encode_func_body_enum(&self, variants: &[MirEnumVariant]) -> Acc<Option<String>> {
        let variant_cases = variants
            .iter()
            .enumerate()
            .map(|(index, variant)| {
                let variant_name = &variant.name.rust_style(false);
                match &variant.kind {
                    MirVariantKind::Value => {
                        format!(
                            "{}.{} => inner.new_{}_{}_{}()",
                            self.context.mir_pack.get_struct_ref_str(&MirType::Generic(self.mir.clone())),
                            variant_name,
                            self.context.mir_pack.get_struct_ref_str(&MirType::Generic(self.mir.clone())).to_lowercase(),
                            variant_name.to_lowercase(),
                            index
                        )
                    },
                    MirVariantKind::Struct(st) => {
                        let field_assigns = st.fields.iter()
                            .map(|field| {
                                let field_name = &field.name.rust_style(false);
                                format!(
                                    "ans.ref.field{}.ref.{} = {};",
                                    index,
                                    field_name,
                                    self.generate_field_encoder(&field.ty, field_name)
                                )
                            })
                            .collect::<Vec<_>>()
                            .join("\n      ");
                        
                        format!(
                            r#"{}.{}(obj) => {{
      final ans = inner.new_{}_{}_{}_{}();
      {}
      return ans;
    }}"#,
                            self.context.mir_pack.get_struct_ref_str(&MirType::Generic(self.mir.clone())),
                            variant_name,
                            self.context.mir_pack.get_struct_ref_str(&MirType::Generic(self.mir.clone())),
                            self.context.mir_pack.get_struct_ref_str(&MirType::Generic(self.mir.clone())).to_lowercase(),
                            variant_name,
                            index,
                            field_assigns
                        )
                    },
                }
            })
            .collect::<Vec<_>>()
            .join(",\n    ");

        Acc {
            io: Some(format!(
                r#"return raw.when(
    {}
  );"#,
                variant_cases
            )),
            web: Some(format!(
                r#"return {{ 'tag': raw.runtimeType.toString(), 'value': raw.cstEncode() }};"#
            )),
            ..Default::default()
        }
    }

    fn generate_field_encoder(&self, _field_type: &MirType, field_name: &str) -> String {
        // This should delegate to the appropriate encoder based on field type
        format!("{}.cstEncode()", field_name)
    }
}

impl WireDartCodecCstGeneratorEncoderTrait for GenericRefWireDartCodecCstGenerator<'_> {
    fn generate_encode_func_body(&self) -> Acc<Option<String>> {
        let base_generator = GenericWireDartCodecCstGenerator::new(
            self.mir.generic_type.as_ref().clone(),
            self.context,
        );
        base_generator.generate_encode_func_body()
    }

    fn dart_wire_type(&self, target: Target) -> String {
        let base_generator = GenericWireDartCodecCstGenerator::new(
            self.mir.generic_type.as_ref().clone(),
            self.context,
        );
        base_generator.dart_wire_type(target)
    }
}
