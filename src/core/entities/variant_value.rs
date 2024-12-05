use crate::core::entities::variant::Variant;

#[derive(Clone)]
pub struct VariantValue {
    variant: Variant,
    values: Vec<Option<String>>,
}

impl VariantValue {
    pub fn new(variant: Variant, values: Vec<Option<String>>) -> VariantValue {
        VariantValue { variant, values }
    }

    pub(crate) fn variant(&self) -> &Variant {
        &self.variant
    }

    pub fn values(&self) -> &Vec<Option<String>> {
        &self.values
    }
}
