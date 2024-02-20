#[macro_export]
macro_rules! DeclareWrappedType {
    ($struct_name:ident, $field_name:ident, $field_type:ty) => {
        #[derive(PartialEq, Copy, Clone)]
        pub struct $struct_name {
            $field_name: $field_type,
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

        impl std::fmt::Display for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_fmt(format_args!("{}", self.$field_name))
            }
        }
    };
}
