pub extern crate core as __core;

#[macro_export]
macro_rules! offset_of {
    ($container:path, $field:ident) => {{
        // Make sure the field actually exists. This line ensures that a
        // compile-time error is generated if $field is accessed through a
        // Deref impl.
        #[cfg_attr(feature = "cargo-clippy", allow(unneeded_field_pattern))]
        let $container { $field: _, .. };

        // Create an instance of the container and calculate the offset to its
        // field. Although we are creating references to uninitialized data this
        // is fine since we are not dereferencing them.
        #[allow(unused_unsafe)]
        let val: $container = unsafe { $crate::__core::mem::uninitialized() };
        let result = &val.$field as *const _ as usize - &val as *const _ as usize;
        #[cfg_attr(feature = "cargo-clippy", allow(forget_copy))]
        $crate::__core::mem::forget(val);
        result as isize
    }};
}

pub mod apb;
pub mod board;
pub mod clock;
pub mod gpio;
pub mod pinmux;
pub mod timer;
pub mod uart;
pub mod utils;
