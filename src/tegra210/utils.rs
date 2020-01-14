#[repr(align(256))]
pub struct AlignedData256<T> where T: Sized {
    pub value: T,
}

impl<T: Sized> AlignedData256<T> {
    pub const fn new(value: T) -> AlignedData256<T> {
        AlignedData256 { value }
    }
}
