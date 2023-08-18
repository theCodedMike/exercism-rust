const CHAR_MAPS: [u8; 26] = [
    b'z', b'y', b'x', b'w', b'v', b'u', b't', b's', b'r', b'q', b'p', b'o', b'n', b'm', b'l', b'k',
    b'j', b'i', b'h', b'g', b'f', b'e', b'd', b'c', b'b', b'a',
];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    plain
        .bytes()
        .filter_map(|mut c| {
            if c.is_ascii_alphanumeric() {
                if c.is_ascii_alphabetic() {
                    if c.is_ascii_uppercase() {
                        // uppercase
                        c = c.to_ascii_lowercase();
                    }
                    Some(CHAR_MAPS[(c - 97_u8) as usize])
                } else {
                    // digit
                    Some(c)
                }
            } else {
                // non-alphanumeric
                None
            }
        })
        .collect::<Vec<_>>()
        .chunks(5)
        .map(|c| String::from_utf8_lossy(c).to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .bytes()
        .filter_map(|c| {
            if c.is_ascii_alphanumeric() {
                if c.is_ascii_alphabetic() {
                    Some(CHAR_MAPS[(c - 97_u8) as usize] as char)
                } else {
                    // digit
                    Some(c as char)
                }
            } else {
                // non-alphanumeric
                None
            }
        })
        .collect()
}
