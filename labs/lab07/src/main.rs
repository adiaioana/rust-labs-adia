#[derive(Debug, Copy, Clone)]
struct Complex<T1,T2>{
    real:T1,
    imag:T2,
}

impl<T1,T2> Complex<T1,T2> {
    fn new(x:T1, y:T2) -> Self{
        Complex{real:x,imag:y}
    }
}
impl <T1,i32> Complex<T1,i32> {
    fn conjugate(&self) -> Self{
        let x:T1=self.real;
        let y:i32=(self.imag*(-1)) as i32;
        
        Complex{real:x,imag:y}
    }
}
fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
// This is a macro that panics if 2 floats are not equal using an epsilon.
// You are not required to understand it yet, just to use it.
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

impl From<i32> for Complex<i32,i32> {
    fn from(val:i32) -> Self {
        Complex{real:val,imag:0}
    }
}
impl From<f64> for Complex<f64,i32> {
    fn from(val:f64) -> Self {
        Complex{real:val,imag:0}
    }
}
fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d)
}
