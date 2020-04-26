#[inline]
pub fn as_recursive(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        n => as_recursive(n-1) + as_recursive(n-2),
    }
}

#[inline]
pub fn as_loop(n: u64) -> u64 {
    let mut a = 0;
    let mut b = 1;

    match n {
        0 => b,
        _ => {
            for _ in 0..n {
                let c = a + b;
                a = b;
                b = c;
            }
            b
        }
    }
}
