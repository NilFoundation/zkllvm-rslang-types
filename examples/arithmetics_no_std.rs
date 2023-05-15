#![no_main]
#![no_std]

type PallasBase = __zkllvm_field_pallas_base;

fn pow(a: PallasBase, n: u32) -> PallasBase {
    if n == 0 {
        return 1g;
    }
    let mut res = 1g;
    for _ in 0..n {
        res *= a;
    }
    res
}

#[circuit]
pub fn field_arithmetic_example(a: PallasBase, b: PallasBase) -> PallasBase {
    let c = (a + b) * a + b * (a + b) * (a + b);
    const CONSTANT: PallasBase = 0x12345678901234567890g;
    c * c * c / (b - a) + pow(a, 2) + CONSTANT
}
