use crate::{Metadata, Tag, Tags, Test, TestMetadata};

pub struct CalculatorTest;

pub trait Calculator {
    fn new() -> Self;

    fn add(&self, a: i32, b: i32) -> i32;
    fn mul(&self, a: i32, b: i32) -> i32;
}

impl TestMetadata for CalculatorTest {
    fn metadata(&self) -> Metadata {
        Metadata {
            title: "Calculator Test",
            tags: Tags(&[Tag::Demo, Tag::Math]),
            description: "Tests that a given type can operate as a basic integer calculator. \
            This is a demonstration test, and thus is not difficult to implement. In practice, \
            this could be useful to demonstrate that an abstract machine or an implemented \
            languaage  can be used to implement fundamental mathematic operations.",
        }
    }
}

super::document!(
    0:1:0,
    "calc_test.rs",
    CalculatorTest,
    impl<T: Calculator> Test<T> for CalculatorTest {
        /*
        fn eval(input: i32) -> bool {
            let calc = T::new();
            calc.add(input, 3) == input + 3
                && calc.mul(input, 0) == 0
                && calc.mul(input, 1) == 1
                && calc.add(input, 0) == input
                && calc.mul(input, 2) == input * 2
        }
        */
    }
);
