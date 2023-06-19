pub fn is_armstrong_number(num: u32) -> bool {
    let rems = convert_num_to_digits(num);
    let len = rems.len() as u32;

    rems.iter().map(|&x| x.pow(len)).sum::<u64>() == num as u64
}

fn convert_num_to_digits(mut num: u32) -> Vec<u64> {
    let mut rems = vec![];

    while num > 0 {
        let rem = (num % 10) as u64;
        rems.push(rem);
        num /= 10;
    }

    rems
}
