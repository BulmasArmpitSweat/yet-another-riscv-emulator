pub const trait Sizeof<T: Sized> {
    fn sizeof() -> usize {
        core::mem::size_of::<T>()
    }

    fn sizeof_self(&self) -> usize {
        core::mem::size_of::<T>()
    }
}

const impl<T: Sized> Sizeof<T> for T {}