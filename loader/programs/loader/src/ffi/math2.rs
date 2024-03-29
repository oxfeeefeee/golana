use go_vm::types::*;
use go_vm::*;
use spl_math::approximations::sqrt;

#[derive(Ffi)]
pub struct Math2Ffi;

#[ffi_impl]
impl Math2Ffi {
    // Use u128 to calculate the geometric mean of two u64s.
    fn ffi_u64_geometry_mean(x: u64, y: u64) -> u64 {
        let z = (x as u128) * (y as u128);
        sqrt(z).unwrap() as u64
    }

    // Use u128 to calculate x * m / d
    fn ffi_u64_mul_div(x: u64, m: u64, d: u64) -> u64 {
        let z = (x as u128) * (m as u128);
        (z / (d as u128)) as u64
    }
}
