pub fn is_armstrong_number(num: u32) -> bool {
    let digits_count = (num as f64).log10() as u32 + 1;

    convert_num_to_digits_and_calc(num, digits_count) == num as u64
}

fn convert_num_to_digits_and_calc(mut num: u32, digits_count: u32) -> u64 {
    let mut sum = 0_u64;

    while num > 0 {
        let rem = (num % 10) as u64;
        sum += rem.pow(digits_count);
        num /= 10;
    }

    sum
}
