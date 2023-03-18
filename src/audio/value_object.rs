macro_rules! define_value_object {
    ($vis:vis $name:ident, $ty:ty, $default:expr, |$value:ident| -> bool $validation_body:block) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $name($ty);

        impl $name {
            pub fn value(&self) -> $ty {
                self.0
            }

            pub fn new(value: $ty) -> Option<Self> {
                let valid = (|$value: $ty| $validation_body)(value);
                if valid { Some($name(value)) } else { None }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                ($default).try_into().unwrap()
            }
        }

        impl TryFrom<$ty> for $name {
            type Error = &'static str;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                $name::new(value).ok_or("Provided default value is invalid regarding the validation for value object")
            }
        }

        impl std::ops::Deref for $name {
            type Target = $ty;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };
}

define_value_object!(pub Volume, f32, 1.0, |v| -> bool { v >= 0.0 && v <= 1.0 });


