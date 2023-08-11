use rand::Rng;

///
/// Pick a private key greater than 1 and less than p
///
pub fn private_key(p: u64) -> u64 {
    rand::thread_rng().gen_range(2..p)
}

///
/// Calculate public key using prime numbers p and g, and private key a
///
/// public_key = (g ^ a) % p = ((g % p) ^ a) % p
///
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    pow_then_mod(g as u128, a as u128, p as u128)
}

///
/// Calculate secret key using prime number p, public key b_pub, and private key a
///
pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    pow_then_mod(b_pub as u128, a as u128, p as u128)
}

fn pow_then_mod(mut base: u128, mut exp: u128, mo: u128) -> u64 {
    let mut res = 1;

    while exp != 0 {
        if exp & 1 != 0 {
            // if exp is odd
            res = res * base % mo;
        }
        base = base * base % mo;
        exp /= 2;
    }

    res as u64
}
