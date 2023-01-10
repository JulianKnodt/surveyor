
### Calculator Test

- Description: Tests that a given type can operate as a basic integer calculator. This is a demonstration test, and thus is not difficult to implement. In practice, this could be useful to demonstrate that an abstract machine or an implemented languaage  can be used to implement fundamental mathematic operations.
- Version: 0.1.0
- Tags: <button>Demo</button><button>Math</button>

Code:
```rust
impl<T: Calculator> Test<T> for CalculatorTest {}

```

### 2D Spatial Query Test

- Description: Tests if a data-structure correctly implements queries in a ball around a
        point in 2D. A common problem is collision detection between bounded objects which can
        be checked quickly by adding a bounding sphere around each object. This test assesses
        whether or not a library can check for points within a given radius
- Version: 0.1.0
- Tags: <button>Demo</button><button>2D Spatial Query</button>

Code:
```rust
impl<T: Spatial2DQuery<()>> Test<T> for Spatial2DQueryTest {
    super::subtests! {
        "fixed-radius query", fn(f32, (f32, f32)) -> bool = | radius, point |
        {
            let point = [point.0, point.1] ; let mut sp = T :: new(radius) ;
            const N : usize = 128 ; let mut gt_hits = vec! [] ; for i in 0 ..
            N
            {
                let x = i as f32 / N as f32 ; for j in 0 .. N
                {
                    let y = j as f32 / N as f32 ; let p = [x, y] ;
                    sp.insert(p, ()) ; if dist(p, point) < radius
                    { gt_hits.push(p) ; }
                }
            } for near_p in sp.query(point)
            {
                let idx = gt_hits.iter().position(| & p | p == near_p) ; if
                let Some(idx) = idx { gt_hits.remove(idx) ; } else
                { return false ; }
            } true
        }
    }
}

```

### Basic Linear Algebra Test

- Description: Tests for a basic API for tensor math with linear algebra. It checks that
        a type implements basic vectorized math operations, and can be indexed to get each
        element.
- Version: 0.1.0
- Tags: <button>Demo</button><button>Linear Algebra</button>

Code:
```rust
impl<T: Linalg> Test<T> for LinalgTest {
    super::subtests! {
        "elem-wise addition", fn((f32, f32, f32), (f32, f32, f32)) -> bool = |
        (x, y, z), (i, j, k) |
        {
            T :: new(& [x, y, z]) + T :: new(& [i, j, k]) == T ::
            new(& [x + i, y + j, z + k])
        },
    }
}

```
