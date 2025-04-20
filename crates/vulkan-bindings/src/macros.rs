#[macro_export]
macro_rules! create_info_builder {
    (
        $name:ident,
        s_type: $s_type_value:expr,
        $($method:ident => $field:ident : $type:ty $(=> $is_ptr:ident)?),+ $(,)?
    ) => {
        impl Default for $name {
            fn default() -> Self {
                let mut info: $name = unsafe { std::mem::zeroed() };
                info.sType = $s_type_value;

                $(
                    create_info_builder!(@maybe_null info, $field, $type $(, $is_ptr)?);
                )*

                info
            }
        }

        impl $name {
            $(
                pub fn $method(&mut self, value: $type) -> &mut Self {
                    self.$field = value;
                    self
                }
            )*
        }
    };

    // Optional null initializer
    (@maybe_null $info:ident, $field:ident, $type:ty, null) => {
        $info.$field = std::ptr::null::<_>();
    };
    (@maybe_null $info:ident, $field:ident, $type:ty, null_mut) => {
        $info.$field = std::ptr::null_mut::<_>();
    };
    // Skip non-pointers
    (@maybe_null $info:ident, $field:ident, $type:ty) => {};
}

#[macro_export]
macro_rules! state_struct_builder {
    (
        $name:ident,
        $($method:ident => $field:ident : $type:ty $(=> $is_ptr:ident)?),+ $(,)?
    ) => {
        impl Default for $name {
            fn default() -> Self {
                unsafe { std::mem::zeroed() }
            }
        }

        impl $name {
            $(
                pub fn $method(&mut self, value: $type) -> &mut Self {
                    self.$field = value;
                    self
                }
            )*
        }
    };

    // Optional null initializer
    (@maybe_null $info:ident, $field:ident, $type:ty, null) => {
        $info.$field = std::ptr::null::<_>();
    };
    (@maybe_null $info:ident, $field:ident, $type:ty, null_mut) => {
        $info.$field = std::ptr::null_mut::<_>();
    };
    // Skip non-pointers
    (@maybe_null $info:ident, $field:ident, $type:ty) => {};
}
