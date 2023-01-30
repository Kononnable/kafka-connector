mod metadata;

mod prelude {
    pub use crate::api::generator::ApiStruct;
    pub use crate::api::generator::Field;
    pub use crate::api::generator::FieldType;
}
#[test]
fn generate_api_structures() {
    let api_definitions = [metadata::get_api_call()];
    super::generator::api_codegen(&api_definitions);
}
