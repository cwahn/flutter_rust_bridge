use crate::codegen::generator::api_dart::spec_generator::class::method::dart_constructor_postfix;
use crate::codegen::ir::mir::ty::enumeration::{MirEnumMode, MirVariantKind};
use crate::library::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::library::codegen::ir::mir::ty::MirTypeTrait;
use itertools::Itertools;

impl WireDartCodecDcoGeneratorDecoderTrait for crate::library::codegen::generator::wire::dart::spec_generator::codec::dco::base::GenericWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        let base_type = &*self.mir.base_type;
        
        match base_type {
            crate::codegen::ir::mir::ty::MirType::StructRef(struct_ref) => {
                let s = struct_ref.get(self.context.mir_pack);
                
                let inner = s
                    .fields
                    .iter()
                    .enumerate()
                    .map(|(idx, field)| {
                        format!(
                            "{}: dco_decode_{}(arr[{}]),",
                            field.name.dart_style(),
                            field.ty.safe_ident(),
                            idx
                        )
                    })
                    .collect_vec();

                let inner = inner.join("\n");
                let cast = "final arr = raw as List<dynamic>;".to_string();
                let safe_check = format!("if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');", s.fields.len(), s.fields.len());
                
                // Generate generic class name with type parameters
                let type_params = self.mir.type_parameters.join(", ");
                let class_name = format!("{}<{}>", s.name.name, type_params);
                
                let ctor_postfix = dart_constructor_postfix(
                    &s.name.name,
                    &self.context.mir_pack.funcs_with_impl(),
                    self.context.as_api_dart_context(),
                );
                
                format!(
                    "{cast}
                {safe_check}
                return {class_name}{ctor_postfix}({inner});",
                )
            }
            crate::codegen::ir::mir::ty::MirType::EnumRef(enum_ref) => {
                let enu = enum_ref.get(self.context.mir_pack);
                assert_eq!(enu.mode, MirEnumMode::Complex);

                let variants = enu
                    .variants()
                    .iter()
                    .enumerate()
                    .map(|(idx, variant)| {
                        let args = match &variant.kind {
                            MirVariantKind::Value => "".to_owned(),
                            MirVariantKind::Struct(st) => st
                                .fields
                                .iter()
                                .enumerate()
                                .map(|(idx, field)| {
                                    let val = format!("dco_decode_{}(raw[{}]),", field.ty.safe_ident(), idx + 1);
                                    if st.is_fields_named {
                                        format!("{}: {}", field.name.dart_style(), val)
                                    } else {
                                        val
                                    }
                                })
                                .collect_vec()
                                .join(""),
                        };
                        
                        // Generate generic variant name with type parameters
                        let type_params = self.mir.type_parameters.join(", ");
                        let variant_name = if type_params.is_empty() {
                            variant.wrapper_name.to_string()
                        } else {
                            format!("{}<{}>", variant.wrapper_name, type_params)
                        };
                        
                        format!("case {}: return {}({});", idx, variant_name, args)
                    })
                    .collect_vec();
                    
                format!(
                    "switch (raw[0]) {{
                        {}
                        default: throw Exception(\"unreachable\");
                    }}",
                    variants.join("\n"),
                )
            }
            _ => {
                // Fallback for unsupported base types
                "throw UnimplementedError('Generic DCO decoding for this base type not yet supported');".to_string()
            }
        }
    }
}

impl WireDartCodecDcoGeneratorDecoderTrait for crate::library::codegen::generator::wire::dart::spec_generator::codec::dco::base::GenericRefWireDartCodecDcoGenerator<'_> {
    fn generate_impl_decode_body(&self) -> String {
        // GenericRef should delegate to the underlying generic type
        let base_generator = crate::library::codegen::generator::wire::dart::spec_generator::codec::dco::base::GenericWireDartCodecDcoGenerator::new(
            (*self.mir.generic_type).clone(),
            self.context
        );
        base_generator.generate_impl_decode_body()
    }
}
