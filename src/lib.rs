#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use quickcheck::{Arbitrary, Gen};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub mod tests;
pub use tests::*;

pub enum Tag {
    Demo,
    Math,
    SpatialQuery2D,
    Regex,
}

impl Tag {
    fn to_string(&self) -> &'static str {
        use Tag::*;
        match self {
            Demo => "Demo",
            Math => "Math",
            SpatialQuery2D => "2D Spatial Query",
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
    type Input: Arbitrary + Send + 'static;

    fn eval(input: Self::Input) -> bool;
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestResult {
    Pass,
    Fail,
    Timeout,
}

pub fn run_test<S: Default, T: Test<S>>(timeout: Duration) -> TestResult {
    const NUM_RUNS: usize = 10;
    let (send, rcv) = mpsc::channel();

    let mut g = Gen::new(10);
    for _ in 0..NUM_RUNS {
        let input = T::Input::arbitrary(&mut g);
        let send_clone = send.clone();
        thread::spawn(move || {
            send_clone.send(T::eval(input)).unwrap();
        });

        match rcv.recv_timeout(timeout) {
            Ok(true) => {}
            Ok(false) => return TestResult::Fail,
            Err(_) => return TestResult::Timeout,
        }
    }

    TestResult::Pass
}

#[macro_export]
macro_rules! generate_test {
    ($test: tt, $imp: tt) => {
        fn main() {
            use surveyor::{run_test, TestResult};
            let dur = std::time::Duration::from_secs(10);
            let exit_code = match run_test::<$imp, $test>(dur) {
                TestResult::Pass => 0,
                TestResult::Fail => 1,
                TestResult::Timeout => 2,
            };
            std::process::exit(exit_code);
        }
    };
}

pub fn all_metadata() -> Vec<Box<dyn TestMetadata>> {
    vec![Box::new(CalculatorTest), Box::new(Spatial2DQueryTest)]
}
