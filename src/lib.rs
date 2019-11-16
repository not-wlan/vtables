pub trait VTable {
    /// # Safety
    ///
    /// This calls function pointers from memory with arbitrary parameters.
    /// This is just about as unsafe as it gets.
    unsafe fn get_virtual<T: Sized>(&self, index: usize) -> T;
}
