use std::cmp::Ordering;

/// The unique Specta ID for the type.
///
/// Be aware type aliases don't exist as far as Specta is concerned as they are flattened into their inner type by Rust's trait system.
/// The Specta Type ID holds for the given properties:
///  - `T::SID == T::SID`
///  - `T::SID != S::SID`
///  - `Type<T>::SID == Type<S>::SID` (unlike std::any::TypeId)
///  - `&'a T::SID == &'b T::SID` (unlike std::any::TypeId which forces a static lifetime)
///  - `Box<T> == Arc<T> == Rc<T>` (unlike std::any::TypeId)
///
#[derive(Debug, Clone, Copy)]
pub struct SpectaID {
    pub(crate) type_name: &'static str,
    pub(crate) hash: u64,
}

// We do custom impls so the order prefers type_name over hash.
impl Ord for SpectaID {
    fn cmp(&self, other: &Self) -> Ordering {
        self.type_name
            .cmp(other.type_name)
            .then(self.hash.cmp(&other.hash))
    }
}

// We do custom impls so the order prefers type_name over hash.
impl PartialOrd<Self> for SpectaID {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// We do custom impls so equals is by SID exclusively.
impl Eq for SpectaID {}

// We do custom impls so equals is by SID exclusively.
impl PartialEq<Self> for SpectaID {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

/// The location of the impl block for a given type. This is used for error reporting.
/// The content of it is transparent and is generated by the macros.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImplLocation(pub(crate) &'static str);

impl ImplLocation {
    /// Get the location as a string
    pub const fn as_str(&self) -> &'static str {
        self.0
    }
}
