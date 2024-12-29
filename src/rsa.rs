// https://de.wikipedia.org/wiki/RSA-Kryptosystem
pub fn run() {
    // two primes. p must not be equal to q. also, it's recommended for them to have the same order
    // of magnitude, but they shouldn't be to close to each other. they should fall somewhere in
    // the following range:
    // 0.1 < abs(log2(p)-log2(1)) < 30
    let p = 17;
    let q = 19;

    let abs = f32::abs(f32::log2(p as f32) - f32::log2(q as f32));
    println!("abs = {}", abs);

    // RSA-Modulus
    let n = p * q;
    println!("N = {}", n);

    // Euler's totient function phi(N)
    let phi_n = (p - 1) * (q - 1);
    println!("phi(N) = {}", phi_n);

    // find a coprime e to phi(N) for 1 < e < phi(N) - 1
    let e = 31;
    println!("e = {}", e);

    // find decryption exponent d
    let mut d = 0;
    loop {
        if (e * d) % phi_n == 1 {
            break;
        }

        d += 1;
    }
    println!("d = {}", d);

    // p, q and phi(N) are not needed anymore and can be discarded. since they can be used to
    // easily create p, q and phi(N), they should be kept secret. for higher security, it's
    // recommended to use an exponent e between 2^16 < e < 2^64 and discards all primes p and q
    // where (p - 1) and (q - 1) aren't coprime to e. also e shouldn't be smaller than the fermat
    // number F4 = 2^16 + 1 = 65537
    let public_key = (e, n);
    let private_key = (d, n);
    println!("public key = {:?}", public_key);
    println!("private key = {:?}", private_key);

    // simple example, message must be smaller than N
    let message: u32 = 42;
    let encrypted = mod_pow(message, e, n);
    let decrypted = mod_pow(encrypted, d, n);

    println!("message = {}", message);
    println!("encrypted = {}", encrypted);
    println!("decrypted = {}", decrypted);

    // try with string. currently this encrypts the bytes individually. for an actual real world
    // example, one would use multiple characters as a single number. because the numbers easily
    // become very large, it seems rsa is not suitable for large data streams.
    let message_string = "hello world";
    let message = message_string.as_bytes();
    let mut encrypted = message.to_vec();
    for byte in encrypted.iter_mut() {
        *byte = mod_pow(*byte as u32, e, n) as u8;
    }
    let mut decrypted = encrypted.to_vec();
    for byte in decrypted.iter_mut() {
        *byte = mod_pow(*byte as u32, d, n) as u8;
    }
    let decrypted_string = String::from_utf8(decrypted.clone()).unwrap();

    println!("message string = {}", message_string);
    println!("message = {:?}", message);
    println!("encrypted = {:?}", encrypted);
    println!("decrypted = {:?}", decrypted);
    println!("decrypted_string = {}", decrypted_string);
}

fn mod_pow(value: u32, exp: u32, modulus: u32) -> u32 {
    let mut result = 1;
    for _ in 0..exp {
        result = mod_mul(result, value, modulus);
    }
    result
}

fn mod_mul(lhs: u32, rhs: u32, modulus: u32) -> u32 {
    let mut result = 0;
    for _ in 0..rhs {
        result = mod_add(result, lhs, modulus);
    }
    result
}

fn mod_add(lhs: u32, rhs: u32, modulus: u32) -> u32 {
    let mut result = lhs + rhs;
    while result >= modulus {
        result -= modulus;
    }
    result
}
