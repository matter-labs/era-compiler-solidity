//!
//! Proxy trait for `era_compiler_llvm_context::EraVMWriteLLVM`
//!

///
/// Allows wrapping some type in the type `Self::Wrapper`
///
pub trait Wrap {
    type Wrapper;
    fn wrap(self) -> Self::Wrapper;
}

///
/// Creates a wrapper structure for a YUL syntax tree node type
/// [`unwrapped_type`] and implements [`Wrap`] trait for it.
///
#[macro_export]
macro_rules! create_wrapper {
    ($unwrapped_type:ty, $wrapped_type:ident) => {
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        pub struct $wrapped_type(pub $unwrapped_type);

        impl $crate::yul::parser::wrapper::Wrap for $unwrapped_type {
            type Wrapper = $wrapped_type;

            fn wrap(self) -> Self::Wrapper {
                $wrapped_type(self)
            }
        }
    };
}
