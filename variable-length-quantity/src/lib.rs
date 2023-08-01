#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

const U32_MAX: u64 = u32::MAX as u64;

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values
        .into_iter()
        .flat_map(|v| process_single(*v))
        .collect()
}

fn process_single(mut ori: u32) -> Vec<u8> {
    if ori == 0 {
        return vec![0];
    }
    // to binary
    let mut len = ori.ilog2() as usize + 1;
    len = 7 * if len % 7 == 0 { len / 7 } else { (len / 7) + 1 };
    let mut bits = vec![0; len];
    for i in (0..len).rev() {
        bits[i] = (ori % 2) as u8;
        ori /= 2;
        if ori == 0 {
            break;
        }
    }

    // group by 7 bits
    let last_idx = len / 7 - 1;
    bits = bits
        .chunks(7)
        .enumerate()
        .flat_map(|(idx, v)| {
            let mut vec = v.to_vec();
            if idx == last_idx {
                vec.insert(0, 0);
            } else {
                vec.insert(0, 1);
            }
            vec
        })
        .collect::<Vec<_>>();

    // to dec
    let res = bits
        .chunks(8)
        .map(|v| {
            let mut dec = 0;

            for (idx, val) in v.iter().enumerate() {
                if *val == 1 {
                    dec += 2_u8.pow((7 - idx) as u32);
                }
            }

            dec
        })
        .collect::<Vec<_>>();

    res
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    // to binary
    let mut bits = bytes
        .iter()
        .map(|v| {
            let mut val = *v;
            let mut vec = vec![0; 8];
            for i in (0..8).rev() {
                vec[i] = val % 2;
                val /= 2;
                if val == 0 {
                    break;
                }
            }
            vec
        })
        .collect::<Vec<_>>();

    // group
    let groups = bits
        .split_inclusive_mut(|v| v[0] == 0)
        .map(|v| {
            let mut group = vec![];

            for i in v {
                group.append(i);
            }

            group
        })
        .collect::<Vec<_>>();

    let mut res = vec![];
    for group in groups {
        let sub_res = handle_single(group)?;
        res.push(sub_res);
    }

    Ok(res)
}

fn handle_single(bits: Vec<u8>) -> Result<u32, Error> {
    // validate
    let bits_len = bits.len();
    if bits[bits_len - 8] == 1 {
        return Err(Error::IncompleteNumber);
    }

    // remove 1 or 0
    let real_bits = bits
        .chunks(8)
        .flat_map(|v| {
            let mut vec = v.to_vec();
            vec.remove(0);
            vec
        })
        .collect::<Vec<_>>();

    let mut dec = 0_u64;
    let len = real_bits.len() - 1;
    for (idx, val) in real_bits.iter().enumerate() {
        if *val == 1 {
            let pow = 2_u64.pow((len - idx) as u32);
            if dec + pow > U32_MAX {
                return Err(Error::Overflow);
            }
            dec += pow;
        }
    }

    Ok(dec as u32)
}
