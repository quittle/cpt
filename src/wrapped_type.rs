#[macro_export]
macro_rules! DeclareWrappedType {
    ($struct_name:ident, $field_name:ident, $field_type:ty) => {
        #[derive(PartialEq, Copy, Clone, Eq, Hash, Debug)]
        pub struct $struct_name {
            pub $field_name: $field_type,
        }

        impl $struct_name {
            pub fn new($field_name: $field_type) -> Self {
                Self { $field_name }
            }

            pub fn parse($field_name: &str) -> Option<Self> {
                Some(Self {
                    $field_name: $field_name.parse::<$field_type>().ok()?,
                })
            }
        }

        impl serde::Serialize for $struct_name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_newtype_struct(stringify!($struct_name), &self.$field_name)
            }
        }

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{}", self.$field_name))
            }
        }

        impl From<$struct_name> for String {
            fn from(source: $struct_name) -> String {
                source.$field_name.to_string()
            }
        }

        impl std::ops::AddAssign for $struct_name {
            fn add_assign(&mut self, rhs: Self) {
                *self = Self {
                    $field_name: self.$field_name + rhs.$field_name,
                }
            }
        }
    };
}
