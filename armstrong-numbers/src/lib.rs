pub fn is_armstrong_number(num: u32) -> bool {
    let mut copy = num;
    let mut rems = vec![];

    while copy > 0 {
        let rem = (copy % 10) as u8;
        rems.push(rem);
        copy /= 10;
    }

    let len = rems.len();
    let mut sum = 0_u64;
    for rem in rems {
        sum += product(len, rem);
    }

    sum == num as u64
}

fn product(len: usize, rem: u8) -> u64 {
    let mut prod = 1_u64;

    for _ in 0..len {
        prod *= rem as u64
    }

    prod
}
