use broken_app::{algo, count_nonzero_bytes, normalize, sum_even};

fn main() {
    let nums = [1, 2, 3, 4];
    println!("sum_even: {}", sum_even(&nums));

    let data = [1_u8, 0, 2, 3];
    println!("non-zero bytes: {}", count_nonzero_bytes(&data));

    let text = " Hello World ";
    println!("normalize: {}", normalize(text));

    let fib = algo::slow_fib(20);
    println!("fib(20): {}", fib);

    let uniq = algo::slow_dedup(&[1, 2, 2, 3, 1, 4, 4]);
    println!("dedup: {:?}", uniq);
}
