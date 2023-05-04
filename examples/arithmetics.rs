#![cfg_attr(target_arch = "assigner", feature(restricted_std))]
#![no_main]

extern crate zktypes;

use zktypes::PallasBase;

fn pow(a: PallasBase, n: u32) -> PallasBase {
    if n == 0 {
        return 1g.into();
    }
    let mut res = 1g.into();
    for _ in 0..n {
        res *= a;
    }
    res
}

#[circuit]
pub fn field_arithmetic_example(a: PallasBase, b: PallasBase) -> PallasBase {
    let c = (a + b) * a + b * (a + b) * (a + b);
    const CONSTANT: PallasBase = PallasBase(0x12345678901234567890g);
    c * c * c / (b - a) + pow(a, 2) + CONSTANT
}
