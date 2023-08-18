const CHAR_MAPS: [char; 26] = [
    'z', 'y', 'x', 'w', 'v', 'u', 't', 's', 'r', 'q', 'p', 'o', 'n', 'm', 'l', 'k', 'j', 'i', 'h',
    'g', 'f', 'e', 'd', 'c', 'b', 'a',
];

/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    let mut res = "".to_string();

    let mut count = 0;
    for mut c in plain.chars() {
        if c.is_ascii_alphanumeric() {
            count += 1;

            if count > 1 && count % 5 == 1 {
                res.push(' ');
            }

            if c.is_ascii_alphabetic() {
                if c.is_ascii_uppercase() {
                    c = c.to_ascii_lowercase();
                }
                res.push(CHAR_MAPS[(c as u8 - 97) as usize]);
            } else {
                res.push(c);
            }
        }
    }

    res
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    cipher
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .map(|c| {
            if c.is_ascii_alphabetic() {
                CHAR_MAPS[(c as u8 - 97) as usize]
            } else {
                c
            }
        })
        .collect()
}
