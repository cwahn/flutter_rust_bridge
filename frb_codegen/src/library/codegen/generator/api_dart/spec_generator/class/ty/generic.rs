use crate::codegen::generator::api_dart::spec_generator::base::*;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_dart_metadata,
};
use crate::codegen::ir::mir::ty::MirType;

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

                // Generate simple fields for now - we can enhance this later
                let fields: Vec<String> = src.fields.iter()
                    .map(|field| {
                        let field_name = &field.name.dart_style();
                        let field_type = ApiDartGenerator::new(field.ty.clone(), self.context).dart_api_type();
                        format!("  final {} {};", field_type, field_name)
                    })
                    .collect();

                let constructor_params: Vec<String> = src.fields.iter()
                    .map(|field| {
                        let field_name = &field.name.dart_style();
                        format!("required this.{}", field_name)
                    })
                    .collect();

                let class_body = format!(
                    r#"{comments}{metadata}
class {}<{}> {{
{}

  const {}({{{}}});
}}"#,
                    class_name,
                    type_params,
                    fields.join("\n"),
                    class_name,
                    constructor_params.join(", ")
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
            MirType::EnumRef(_enum_ref) => {
                // For now, we don't support generic enums - they're more complex
                // This could be implemented later when needed
                None
            }
            _ => None,
        }
    }
}

impl ApiDartGeneratorClassTrait for GenericRefApiDartGenerator<'_> {
    // GenericRef doesn't generate its own class - it references an existing generic class
}
