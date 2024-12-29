pub fn mod_pow(value: u32, exp: u32, modulus: u32) -> u32 {
    let mut result = 1;
    for _ in 0..exp {
        result = mod_mul(result, value, modulus);
    }
    result
}

pub fn mod_mul(lhs: u32, rhs: u32, modulus: u32) -> u32 {
    let mut result = 0;
    for _ in 0..rhs {
        result = mod_add(result, lhs, modulus);
    }
    result
}

pub fn mod_add(lhs: u32, rhs: u32, modulus: u32) -> u32 {
    let mut result = lhs + rhs;
    while result >= modulus {
        result -= modulus;
    }
    result
}
