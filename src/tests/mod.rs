macro_rules! document {
  (
    $semver_major:literal:$semver_minor:literal:$semver_patch:literal,
    $file:literal, $t: ty, $( $code_block: tt )*
  )=> {
    use crate::DocumentedTest;
    impl DocumentedTest for $t {
        fn semver(&self) -> semver::Version {
          semver::Version {
            major: $semver_major,
            minor: $semver_minor,
            patch: $semver_patch,
            pre: semver::Prerelease::EMPTY,
            build: semver::BuildMetadata::EMPTY,
          }
        }
        fn example_file(&self) -> &'static str { $file }
        fn code_block(&self) -> &'static str { stringify!($($code_block)*) }
    }

    $( $code_block )*
  }
}

pub(crate) use document;

pub mod calc;
pub use calc::*;

pub mod spatial_query;
pub use spatial_query::*;
