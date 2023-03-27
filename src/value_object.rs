#[macro_export]
macro_rules! define_value_object {
    ($vis:vis $name:ident, $ty:ty, $default:expr) => {
        define_value_object!($vis $name, $ty, $default, |_v| { true });
    };
    ($vis:vis $name:ident, $ty:ty, $default:expr, |$value:ident| $validation_body:block) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        $vis struct $name($ty);

        impl $name {
            pub fn value(&self) -> $ty {
                self.0
            }

            pub fn new(value: $ty) -> Option<Self> {
                #[allow(clippy::redundant_closure_call)]
                let valid = (|$value: $ty| $validation_body)(value);
                if valid { Some($name(value)) } else { None }
            }
        }

        impl Default for $name {
            fn default() -> Self {
                ($default).try_into().expect("Invalid default value {}")
            }
        }

        impl TryFrom<$ty> for $name {
            type Error = anyhow::Error;

            fn try_from(value: f32) -> Result<Self, Self::Error> {
                $name::new(value).ok_or(anyhow::anyhow!("Invalid value: {}", value))
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
