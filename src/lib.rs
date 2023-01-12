#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use quickcheck::{Gen, Testable};
use std::time::Duration;

pub mod tests;
pub use tests::*;

pub enum Tag {
    Demo,
    Math,
    SpatialQuery2D,
    LinearAlgebra,
    Regex,
}

impl Tag {
    fn to_string(&self) -> &'static str {
        use Tag::*;
        match self {
            Demo => "Demo",
            Math => "Math",
            SpatialQuery2D => "2D Spatial Query",
            LinearAlgebra => "Linear Algebra",
            Regex => "Regex",
        }
    }
}

pub struct Tags(&'static [Tag]);

impl std::fmt::Display for Tags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for tag in self.0 {
            write!(f, "<button>{}</button>", tag.to_string())?;
        }
        Ok(())
    }
}

/// A test case for `T`, whether it correctly implements a given function.
pub trait Test<T> {
    fn subtests() -> &'static [(&'static str, &'static dyn Testable)] {
        &[]
    }
}

pub struct Metadata {
    pub title: &'static str,
    pub tags: Tags,
    pub description: &'static str,
}

pub trait TestMetadata: DocumentedTest {
    fn metadata(&self) -> Metadata;
}

/// A test that is documented
pub trait DocumentedTest {
    fn code_block(&self) -> &'static str;
    fn example_file(&self) -> &'static str;

    fn semver(&self) -> semver::Version;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestResult {
    Pass,
    Fail,
    Timeout,
}

pub fn run_test<S, T: Test<S>>(
    _timeout: Duration,
) -> impl Iterator<Item = (&'static str, TestResult)> {
    const NUM_RUNS: usize = 10;
    let mut g = Gen::new(10);

    T::subtests().iter().map(move |&(name, test)| {
        for _ in 0..NUM_RUNS {
            let result = test.result(&mut g);

            if result.is_failure() {
                return (name, TestResult::Fail);
            } else if result.is_error() {
                return (name, TestResult::Timeout);
            }
        }
        (name, TestResult::Pass)
    })
}

#[macro_export]
macro_rules! generate_test {
    ($test: ty, $imp: ty) => {
        fn main() {
            use surveyor::{run_test, TestResult};
            let dur = std::time::Duration::from_secs(10);

            for (name, result) in run_test::<$imp, $test>(dur) {
                let result = match result {
                    TestResult::Pass => "pass",
                    TestResult::Fail => "fail",
                    TestResult::Timeout => "timeout",
                };
                println!("{name}, {result}");
            }
        }
    };
}

pub fn all_metadata() -> Vec<Box<dyn TestMetadata>> {
    vec![
        Box::new(CalculatorTest),
        Box::new(Spatial2DQueryTest),
        Box::new(LinalgTest),
    ]
}
