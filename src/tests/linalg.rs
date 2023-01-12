use crate::{Metadata, Tag, Tags, Test, TestMetadata};
use std::ops::{Add, Div, Mul, Neg, Sub};

/// Indicates whether a type supports basic linear algebra operations
pub trait Linalg:
    Mul<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Div<Output = Self>
    + Neg<Output = Self>
    + Sized
    + PartialEq
    + Copy
{
    /// Create an instance of a tensor with a given shape
    fn new(value: &[f32]) -> Self;

    /// Compute dot product between two vectors
    fn dot(&self, o: &Self) -> f32;

    /// Checks that you can get each element of a tensor
    fn get(&self, i: usize) -> Option<f32>;

    /// Computes the length of a vector
    fn magnitude(&self) -> f32 {
        self.dot(self)
    }
}

pub struct LinalgTest;

impl TestMetadata for LinalgTest {
    fn metadata(&self) -> Metadata {
        Metadata {
            title: "Basic Linear Algebra Test",
            tags: Tags(&[Tag::Demo, Tag::LinearAlgebra]),
            description: "Tests for a basic API for tensor math with linear algebra. It checks that
        a type implements basic vectorized math operations, and can be indexed to get each
        element.",
        }
    }
}

super::document!(
    0:1:0,
    "basic_linalg_test.rs",
    LinalgTest,

    impl<T: Linalg> Test<T> for LinalgTest {
        super::subtests!(
            "elem-wise addition", fn((f32, f32, f32), (f32, f32, f32)) -> bool,
            |(x,y,z), (i,j,k)| {
                T::new(&[x,y,z]) + T::new(&[i,j,k]) == T::new(&[x + i, y + j, z + k])
            },

            "elem-wise subtraction", fn((f32, f32, f32), (f32, f32, f32)) -> bool,
            |(x,y,z), (i,j,k)| {
                T::new(&[x,y,z]) - T::new(&[i,j,k]) == T::new(&[x - i, y - j, z - k])
            },

            "elem-wise multiplication", fn((f32, f32, f32), (f32, f32, f32)) -> bool,
            |(x,y,z), (i,j,k)| {
                T::new(&[x,y,z]) * T::new(&[i,j,k]) == T::new(&[x * i, y * j, z * k])
            },

            "elem-wise division", fn((f32, f32, f32), (f32, f32, f32)) -> bool,
            |(x,y,z), (i,j,k)| {
                T::new(&[x,y,z]) / T::new(&[i,j,k]) == T::new(&[x / i, y / j, z / k])
            },

            "element-access", fn(Vec<f32>, usize) -> bool,
            |v, idx| T::new(&v).get(idx) == v.get(idx).copied(),

            "positive norm", fn (Vec<f32>) -> bool, |v| T::new(&v).magnitude() >= 0.0,
         );
    }
);
