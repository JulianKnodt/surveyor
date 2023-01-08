
### Calculator Test

- Description: Tests that a given type can operate as a basic integer calculator.
- Tags: <button>Demo</button><button>Math</button>

Code:
```
impl < T : Calculator > Test < T > for CalculatorTest
{
    type Input = i32 ; fn eval(input : i32) -> bool
    {
        let calc = T :: new() ; calc.add(input, 3) == input + 3 &&
        calc.mul(input, 0) == 0 && calc.mul(input, 1) == 1 &&
        calc.add(input, 0) == input && calc.mul(input, 2) == input * 2
    }
}
```

### 2D Spatial Query Test

- Description: Tests if a data-structure correctly implements queries in a ball around a point in 2D.
- Tags: <button>Demo</button><button>2D Spatial Query</button>

Code:
```
impl < T : Spatial2DQuery < () >> Test < T > for Spatial2DQueryTest
{
    type Input = (f32, (f32, f32)) ; fn eval((radius, point) : Self :: Input)
    -> bool
    {
        let point = [point.0, point.1] ; let sp = T :: new(radius) ; const N :
        usize = 128 ; let mut gt_hits = vec! [] ; for i in 0 .. N
        {
            let x = i as f32 / N as f32 ; for j in 0 .. N
            {
                let y = j as f32 / N as f32 ; let p = [x, y] ;
                sp.insert(p, ()) ; if dist(p, point) < radius
                { gt_hits.push(p) ; }
            }
        } for near_p in sp.query(point)
        {
            let idx = gt_hits.iter().position(| & p | p == near_p) ; if let
            Some(idx) = idx { gt_hits.remove(idx) ; } else { return false ; }
        } true
    }
}
```
