use rand::Rng;

const A_U8: u8 = b'a';
const Z_U8: u8 = b'z';

const ENCODE_CONVERT: fn(char, char) -> char = |c: char, k: char| -> char {
    let diff = k as u8 - A_U8;
    let res = c as u8 + diff;
    if res > Z_U8 {
        (res - Z_U8 - 1 + A_U8) as char
    } else {
        res as char
    }
};

const DECODE_CONVERT: fn(char, char) -> char = |c: char, k: char| -> char {
    if c >= k {
        (A_U8 + (c as u8 - k as u8)) as char
    } else {
        (Z_U8 - (k as u8 - c as u8 - 1)) as char
    }
};

///
/// Use {key} to encode {s} using shift cipher
///
pub fn encode(key: &str, s: &str) -> Option<String> {
    convert(s, key, ENCODE_CONVERT)
}

///
/// Use {key} to decode {s} using shift cipher
///
pub fn decode(key: &str, s: &str) -> Option<String> {
    convert(s, key, DECODE_CONVERT)
}

///
/// Generate random key with only a-z chars and encode {s}.
///
/// Return tuple (key, encoded s)
///
pub fn encode_random(s: &str) -> (String, String) {
    let key = (0..100)
        .map(|_| rand::thread_rng().gen_range('a'..='z'))
        .collect::<String>();

    let res = convert(s, &key, ENCODE_CONVERT);

    (key, res.unwrap_or_default())
}

fn convert<CF>(mut s: &str, key: &str, convert_fn: CF) -> Option<String>
where
    CF: Fn(char, char) -> char,
{
    if key.is_empty() {
        return None;
    }
    let k_len = key.len();
    let s_len = s.len();
    let (left, right) = s.split_at(std::cmp::min(k_len, s_len));
    s = left;

    let res = s
        .chars()
        .zip(key.chars())
        .map(|(c, k)| {
            if !k.is_ascii_lowercase() {
                return None;
            }
            Some(convert_fn(c, k))
        })
        .collect::<Option<String>>();

    res.map(|s| s + right)
}
