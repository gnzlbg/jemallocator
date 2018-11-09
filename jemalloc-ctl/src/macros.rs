//! Utility macros

macro_rules! types {
    ($id:ident[ str: $byte_string:expr, $mib:ty, $name_to_mib:ident ]  |
     docs: $(#[$doc:meta])*
     mib_docs: $(#[$doc_mib:meta])*
    ) => {
        paste::item! {
            $(#[$doc])*
            #[allow(non_camel_case_types)]
            pub struct $id;

            impl $id {
                const NAME: &'static ::keys::Name = {
                    union U<'a> {
                        bytes: &'a [u8],
                        name: &'a ::keys::Name
                    }

                    unsafe { U { bytes: $byte_string }.name }
                };
                /// Returns Management Information Base (MIB)
                ///
                /// This value can be used to access the key without doing string lookup.
                pub fn mib() -> ::error::Result<[<$id _mib>]> {
                    Ok([<$id _mib>](Self::NAME.$name_to_mib()?))
                }

                /// Key [`::keys::Name`].
                pub fn name() -> &'static ::keys::Name {
                    Self::NAME
                }
            }

            $(#[$doc_mib])*
            #[doc(hidden)]
            #[repr(transparent)]
            #[derive(Copy, Clone)]
            #[allow(non_camel_case_types)]
            pub struct [<$id _mib>](pub ::keys::$mib);
        }
    };
}

/// Read
macro_rules! r {
    ($id:ident => $ret_ty:ty) => {
        paste::item! {
            impl $id {
                /// Reads value using string API.
                pub fn read() -> ::error::Result<$ret_ty> {
                    use ::keys::Access;
                    Self::NAME.read()
                }
            }

            impl [<$id _mib>] {
                /// Reads value using MIB API.
                pub fn read(self) -> ::error::Result<$ret_ty> {
                    use ::keys::Access;
                    self.0.read()
                }
            }

            #[cfg(test)]
            #[test]
            fn [<$id _read_test>]() {
                match stringify!($id) {
                    "background_thread" |
                    "max_background_threads"
                    if cfg!(target_os = "macos") => return,
                    _ => (),
                }

                let _ = $id::read().unwrap();

                let mib = $id::mib().unwrap();
                let _ = mib.read().unwrap();
            }
        }
    };
}

/// Write
macro_rules! w {
    ($id:ident => $ret_ty:ty) => {
        paste::item! {
            impl $id {
                /// Writes `value` using string API.
                pub fn write(value: $ret_ty) -> ::error::Result<()> {
                    use ::keys::Access;
                    Self::NAME.write(value)
                }
            }

            impl [<$id _mib>] {
                /// Writes `value` using MIB API.
                pub fn write(self, value: $ret_ty) -> ::error::Result<()> {
                    use ::keys::Access;
                    self.0.write(value)
                }
            }

            #[cfg(test)]
            #[test]
            fn [<$id _write_test>]() {
                match stringify!($id) {
                    "background_thread" |
                    "max_background_threads"
                        if cfg!(target_os = "macos") => return,
                    _ => (),
                }

                let _ = $id::write($ret_ty::default()).unwrap();

                let mib = $id::mib().unwrap();
                let _ = mib.write($ret_ty::default()).unwrap();
            }
        }
    };
}

/// Update
macro_rules! u {
    ($id:ident  => $ret_ty:ty) => {
        paste::item! {
            impl $id {
                /// Updates key to `value` returning its old value using string API.
                pub fn update(value: $ret_ty) -> ::error::Result<$ret_ty> {
                    use ::keys::Access;
                    Self::NAME.update(value)
                }
            }

            impl [<$id _mib>] {
                /// Updates key to `value` returning its old value using MIB API.
                pub fn update(self, value: $ret_ty) -> ::error::Result<$ret_ty> {
                    use ::keys::Access;
                    self.0.update(value)
                }
            }

            #[cfg(test)]
            #[test]
            fn [<$id _update_test>]() {
                match stringify!($id) {
                    "background_thread" |
                    "max_background_threads"
                        if cfg!(target_os = "macos") => return,
                    _ => (),
                }

                let _ = $id::update($ret_ty::default()).unwrap();

                let mib = $id::mib().unwrap();
                let _ = mib.update($ret_ty::default()).unwrap();
            }
        }
    };
}

/// Creates a new option
macro_rules! option {
    ($id:ident[ str: $byte_string:expr, $mib:ty, $name_to_mib:ident ] => $ret_ty:ty |
     ops: $($ops:ident),* |
     docs:
     $(#[$doc:meta])*
     mib_docs:
     $(#[$doc_mib:meta])*
    ) => {
        types! {
            $id[ str: $byte_string, $mib, $name_to_mib ] |
            docs: $(#[$doc])*
            mib_docs: $(#[$doc_mib])*
        }
        $(
            $ops!($id => $ret_ty);
        )*
    };
    // Non-string option:
    ($id:ident[ str: $byte_string:expr, non_str: $mib_len:expr ] => $ret_ty:ty |
     ops: $($ops:ident),* |
     docs:
     $(#[$doc:meta])*
     mib_docs:
     $(#[$doc_mib:meta])*
    ) => {
        option! {
            $id[ str: $byte_string, Mib<[usize; $mib_len]>, mib ] => $ret_ty |
            ops: $($ops),* |
            docs: $(#[$doc])*
            mib_docs: $(#[$doc_mib])*
        }
    };
    // String option:
    ($id:ident[ str: $byte_string:expr, str: $mib_len:expr ] => $ret_ty:ty |
     ops: $($ops:ident),* |
     docs:
     $(#[$doc:meta])*
     mib_docs:
     $(#[$doc_mib:meta])*
    ) => {
        option! {
            $id[ str: $byte_string, MibStr<[usize; $mib_len]>, mib_str ] => $ret_ty |
            ops: $($ops),* |
            docs: $(#[$doc])*
            mib_docs: $(#[$doc_mib])*
        }
    };
}
