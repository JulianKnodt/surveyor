macro_rules! document {
  ($t: ty, $( $code_block: tt )*)=> {
    use crate::DocumentedTest;
    impl DocumentedTest for $t {
        fn code_block(&self) -> &'static str {
          stringify!($($code_block)*)
        }
    }

    $( $code_block )*
  }
}

pub(crate) use document;

pub mod calc;
pub use calc::*;

pub mod spatial_query;
pub use spatial_query::*;
