use derive_new::new;

#[derive(new)]
pub struct InputUnit {
    pub value: char,
}

impl Default for InputUnit {
    fn default() -> Self {
        InputUnit::new('.')
    }
}
