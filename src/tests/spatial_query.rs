use crate::{Metadata, Tag, Tags, Test, TestMetadata};

pub struct Spatial2DQueryTest;

pub trait Spatial2DQuery<T> {
    /// Create a new instance of this query with a fixed radius
    fn new(r: f32) -> Self;

    /// Query a fixed radius near a point.
    fn query(&self, point: [f32; 2]) -> impl Iterator<Item = [f32; 2]>;

    /// Insert a value into this data-structure
    fn insert(&self, point: [f32; 2], val: T);
}

impl TestMetadata for Spatial2DQueryTest {
    fn metadata(&self) -> Metadata {
        Metadata {
        title: "2D Spatial Query Test",
        tags: Tags(&[Tag::Demo, Tag::SpatialQuery2D]),
        description: "Tests if a data-structure correctly implements queries in a ball around a point in 2D."
      }
    }
}

fn dist([a, b]: [f32; 2], [i, j]: [f32; 2]) -> f32 {
    fn sqr(v: f32) -> f32 {
        v * v
    }
    (sqr(a - i) + sqr(b - j)).sqrt()
}

super::document!(
    Spatial2DQueryTest,
    impl<T: Spatial2DQuery<()>> Test<T> for Spatial2DQueryTest {
        type Input = (f32, (f32, f32));

        fn eval((radius, point): Self::Input) -> bool {
            let point = [point.0, point.1];
            let sp = T::new(radius);
            const N: usize = 128;
            let mut gt_hits = vec![];

            for i in 0..N {
                let x = i as f32 / N as f32;
                for j in 0..N {
                    let y = j as f32 / N as f32;
                    let p = [x, y];
                    sp.insert(p, ());

                    if dist(p, point) < radius {
                        gt_hits.push(p);
                    }
                }
            }

            for near_p in sp.query(point) {
                let idx = gt_hits.iter().position(|&p| p == near_p);
                if let Some(idx) = idx {
                    gt_hits.remove(idx);
                } else {
                    return false;
                }
            }
            true
        }
    }
);
