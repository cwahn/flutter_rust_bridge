use crate::codegen::generator::api_dart::spec_generator::base::*;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_dart_metadata,
};
use crate::codegen::ir::mir::ty::MirType;
use crate::codegen::ir::mir::ty::enumeration::MirVariantKind;

impl ApiDartGeneratorClassTrait for GenericApiDartGenerator<'_> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let base_type = &*self.mir.base_type;
        
        match base_type {
            MirType::StructRef(struct_ref) => {
                let src = struct_ref.get(self.context.mir_pack);
                let comments = generate_dart_comments(&src.comments);
                let metadata = generate_dart_metadata(&src.effective_dart_metadata());

                let type_params = self.mir.type_parameters.join(", ");
                let class_name = &struct_ref.ident.0.name;

                // Generate fields with proper generic type handling
                let fields: Vec<String> = src.fields.iter()
                    .map(|field| {
                        let field_name = &field.name.dart_style();
                        let field_type = self.generate_field_type(&field.ty, &self.mir.type_parameters);
                        format!("  final {} {};", field_type, field_name)
                    })
                    .collect();

                let constructor_params: Vec<String> = src.fields.iter()
                    .map(|field| {
                        let field_name = &field.name.dart_style();
                        format!("required this.{}", field_name)
                    })
                    .collect();

                // Create complete class with generic constraints if any
                let constraints = if !self.mir.constraints.is_empty() {
                    // For now, we don't generate Dart constraints since they're different
                    // In the future, this could be enhanced to generate proper Dart bounds
                    ""
                } else {
                    ""
                };

                let class_body = format!(
                    r#"{comments}{metadata}
class {}<{}>{} {{
{}

  const {}({{{}}});
  
  @override
  String toString() => '{}<{}>(${})';
}}"#,
                    class_name,
                    type_params,
                    constraints,
                    fields.join("\n"),
                    class_name,
                    constructor_params.join(", "),
                    class_name,
                    type_params,
                    src.fields.iter()
                        .map(|f| format!("${{{}}}", f.name.dart_style()))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

                Some(ApiDartGeneratedClass {
                    header: Default::default(),
                    namespace: src.name.namespace.clone(),
                    class_name: class_name.clone(),
                    code: class_body,
                    needs_freezed: false,
                    needs_json_serializable: false,
                })
            }
            MirType::EnumRef(enum_ref) => {
                let src = enum_ref.get(self.context.mir_pack);
                let comments = generate_dart_comments(&src.comments);
                // Note: MirEnum doesn't have effective_dart_metadata, use empty metadata for now
                let metadata = generate_dart_metadata(&vec![]);

                let type_params = self.mir.type_parameters.join(", ");
                let enum_name = &enum_ref.ident.0.name;

                // Generate enum variants with generic type parameters
                let variants: Vec<String> = src.variants.iter()
                    .map(|variant| {
                        let variant_name = &variant.name.dart_style();
                        match &variant.kind {
                            MirVariantKind::Value => {
                                format!("  {};", variant_name)
                            }
                            MirVariantKind::Struct(struct_variant) => {
                                let params: Vec<String> = struct_variant.fields.iter()
                                    .map(|field| {
                                        let field_type = self.generate_field_type(&field.ty, &self.mir.type_parameters);
                                        format!("{} {}", field_type, field.name.dart_style())
                                    })
                                    .collect();
                                
                                if params.is_empty() {
                                    format!("  {};", variant_name)
                                } else {
                                    format!("  {}({});", variant_name, params.join(", "))
                                }
                            }
                        }
                    })
                    .collect();

                let enum_body = format!(
                    r#"{comments}{metadata}
enum {}<{}> {{
{}
}}"#,
                    enum_name,
                    type_params,
                    variants.join("\n")
                );

                Some(ApiDartGeneratedClass {
                    header: Default::default(),
                    namespace: src.name.namespace.clone(),
                    class_name: enum_name.clone(),
                    code: enum_body,
                    needs_freezed: false,
                    needs_json_serializable: false,
                })
            }
            _ => None,
        }
    }
}

impl GenericApiDartGenerator<'_> {
    /// Generate proper Dart type for fields, replacing generic parameters with actual type parameters
    fn generate_field_type(&self, mir_type: &MirType, type_params: &[String]) -> String {
        match mir_type {
            // If it's a generic type parameter, use it directly
            MirType::RustAutoOpaqueImplicit(rust_opaque) => {
                let type_str = rust_opaque.raw.string.with_original_lifetime();
                
                // Check if this is one of our generic parameters
                if type_params.iter().any(|param| param == type_str) {
                    type_str.to_string()
                } else {
                    // Fallback to regular type generation
                    ApiDartGenerator::new(mir_type.clone(), self.context).dart_api_type()
                }
            }
            _ => {
                // For all other types, use standard generation
                ApiDartGenerator::new(mir_type.clone(), self.context).dart_api_type()
            }
        }
    }
}

impl ApiDartGeneratorClassTrait for GenericRefApiDartGenerator<'_> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        // GenericRef doesn't generate its own class - it references an existing generic class
        // The actual instantiation happens at the usage site
        None
    }
}
