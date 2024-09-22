use bigdecimal::{BigDecimal, FromPrimitive};
use std::f64::consts::PI;
use std::ops::{Add, Mul, Sub};

fn main() {
    const N: f64 = (PI * PI) / 6f64;

    let mut sum = BigDecimal::from_f64(0f64).unwrap();
    let mut count = BigDecimal::from(1);
    loop {
        let i = BigDecimal::from(1) / count.clone().mul(count.clone());
        sum = sum.clone().add(i);

        let abs = BigDecimal::from_f64(N).unwrap().sub(sum.clone());
        println!("sum = {sum} | abs = {abs} | count = {count}",);
        count += 1;
    }
}
