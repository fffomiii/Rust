// opredelyaem dlinu posledovatel'nosti Kollatca dlya chisla n
fn collatz_length(mut n: i32) -> u32 {
    
    if n == 1{
        return 1;
    }
    n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
    return 1+collatz_length(n);
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
    // macros, kotorii proveryaet ravenstvo dvyh vyrazhenii
}

fn main() {
    println!("Длина: {}", collatz_length(11));
}
