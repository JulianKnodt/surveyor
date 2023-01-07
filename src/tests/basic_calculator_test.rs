struct CalculatorTest;

pub trait Calculator {
    fn add(&self, a: i32, b: i32) -> i32;
    fn mul(&self, a: i32, b: i32) -> i32;
}

impl<T: Calculator> Test<T> for CalculatorTest {
    type Impl = T;

    type Input = i32;

    fn eval(input: i32, calc: T) -> bool {
        calc.add(input, 3) == input + 3
            && calc.mul(input, 0) == 0
            && calc.mul(input, 1) == 1
            && calc.add(input, 0) == input
            && calc.mul(input, 2) == input * 2
    }
}

type ImplTest<T> = CalculatorTest<T>;
