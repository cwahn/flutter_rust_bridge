use crate::library::codegen::generator::wire::dart::spec_generator::misc::ty::WireDartGeneratorMiscTrait;
use crate::library::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::spec_generator::WireDartOutputCode;

impl WireDartGeneratorMiscTrait for crate::library::codegen::generator::wire::dart::spec_generator::base::GenericWireDartGenerator<'_> {
    fn generate_extra_functions(&self) -> Option<Acc<WireDartOutputCode>> {
        None
    }
}

impl WireDartGeneratorMiscTrait for crate::library::codegen::generator::wire::dart::spec_generator::base::GenericRefWireDartGenerator<'_> {
    fn generate_extra_functions(&self) -> Option<Acc<WireDartOutputCode>> {
        None
    }
}
