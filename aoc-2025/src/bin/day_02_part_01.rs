use std::{fs::File, io::Read};

fn invalid_id_sum(lo: u64, hi: u64) -> u64 {
    let half_digit_pow_10 = (f32::log10(hi as f32).floor() as u32).div_ceil(2);

    let pow_10_val = 10_u64.pow(half_digit_pow_10);

    let top_half = lo / pow_10_val;

    let mut x = top_half;
    let mut p = x * pow_10_val + x;

    let mut sum = 0;

    while p <= hi {
        if p >= lo && f64::log10(p as f64).floor() as u64 % 2 != 0 {
            sum += p;
        }

        x += 1;

        p = x * pow_10_val + x;
    }

    sum
}

fn main() {
    let input_file_path = std::env::args()
        .nth(1)
        .expect("Input file path not specified!");

    println!("{}", input_file_path);

    let mut file = File::open(input_file_path).expect("unable to open file");

    let mut buffer = [0u8; 1024];
    let mut buffer_size: usize = 0;

    let mut sum = 0;

    loop {
        let old_buffer_size = buffer_size;

        let bytes_read = file.read(&mut buffer[old_buffer_size..]).unwrap_or(0);

        if old_buffer_size == 0 && bytes_read == 0 {
            break;
        }

        buffer_size += bytes_read;

        let raw_read_buffer = &buffer[..buffer_size];

        let end = match raw_read_buffer.iter().rposition(|c| *c == b',') {
            Some(x) => x,
            None => buffer_size,
        };

        let mut read_buffer = &buffer[..end];

        while !read_buffer.is_empty() {
            let mut pair_rem_iter = read_buffer.splitn(2, |x| *x == b',');

            match (pair_rem_iter.next(), pair_rem_iter.next()) {
                (Some(pair), rem) => {
                    if let Some((lo, hi)) = str::from_utf8(pair)
                        .expect("invalid utf string")
                        .split_once("-")
                    {
                        let lo = lo.trim().parse().expect("not an integer!");
                        let hi = hi.trim().parse().expect("not an integer!");

                        sum += invalid_id_sum(lo, hi);
                    }

                    read_buffer = rem.unwrap_or(&[]);
                }
                (None, Some(rem)) => read_buffer = rem,
                (None, None) => read_buffer = &[],
            }
        }

        if end < buffer_size {
            let remainder = &buffer[end + 1..buffer_size];
            let remainder_len = remainder.len();
            buffer.copy_within(end + 1..buffer_size, 0);
            buffer_size = remainder_len;
        } else {
            buffer_size = 0;
        }
    }

    println!("{:?}", sum);
}
