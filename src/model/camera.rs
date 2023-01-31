use rust_utils_macro::New;

#[derive(New, Clone, Copy)]
pub struct PatternsCamera {
    #[new_default]
    pub column_offset: i32,
    #[new_default]
    pub line_offset: i32,
}
