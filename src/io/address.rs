pub use paste::paste;

type Value = u32;
type Pointer = *mut Value;

/* This originally used the newtype idiom, but this quickly ran against the
 * the compiler's constant checking when attempting to calculate offsets */
pub struct Address { pub address: Value }

impl Address {
    #[inline(always)]
    fn to_ptr(&self) -> Pointer {
        self.address as Pointer
    }

    #[inline(always)]
    fn offset(&self, offset: Value) -> Address {
        Address { address: self.address + offset }
    }

    #[inline(always)]
    pub fn read(&self) -> Value {
        unsafe {
            core::ptr::read_volatile(self.address as Pointer)
        }
    }

    #[inline(always)]
    pub fn write(&self, value: Value) {
        unsafe {
            core::ptr::write_volatile(self.address as Pointer, value)
        }
    }
}

macro_rules! addr {
    ($name:ident, $address:expr) => {
        pub(crate) const $name: Address = Address { address: $address };
    };

    ($name:ident, relative $base:ident, $offset:expr) => {
        addr!($name, $base.address + $offset);
    };

    ($name:ident, $address:expr; $($tail:tt)+) => {
        addr!($name, $address)
        addr!{$($tail)+}
    };

    ($name:ident, relative $base:ident, $offset:expr; $($tail:tt)+) => {
        addr!($name, relative $base, $offset)
        addr!{$($tail)+}
    };
}

pub(crate) use addr;

// Macro for declaring ranges of 
macro_rules! registers {
    (decl_base $prefix:ident, $base:ident, $base_offset:expr) => {
        paste! {
            addr!([<$prefix _BASE>], relative $base, $base_offset);
        }
    };

    // Declare registers and add a prefix
    (   prefix $prefix:ident,
        base $base:ident,
        offset $base_offset:expr;
        $($name:ident, $offset:expr);+
    ) => {
        registers!(decl_base $prefix, $base, $base_offset);
        $(paste! {
            addr!([<$prefix _ $name>], relative [<$prefix _BASE>], $offset);
        })+
    };

    // Declare registers without adding a prefix;
    (   name $prefix:ident,
        base $base:ident,
        offset $base_offset:expr;
        $($name:ident, $offset:expr);+
    ) => {
        registers!(decl_base $prefix, $base, $base_offset);
        $(paste!{addr!($name, relative [<$prefix _BASE>], $offset);})+
    };

    // Declare registers relative to a base without naming the base address
    (   base $base:ident,
        offset $base_offset:expr;
        $($name:ident, $offset:expr);+
    ) => {
        $( addr!($name, relative $base, $offset) )+
    };

}

pub(crate) use registers;
