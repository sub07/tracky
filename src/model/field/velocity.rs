use derive_new::new;

use crate::model::field::input_unit::InputUnit;

#[derive(new, Default)]
pub struct VelocityField {
    pub digit1: InputUnit,
    pub digit2: InputUnit,
}


