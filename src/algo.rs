/// Убираем дубликаты с сортировкой.
/// Тот же O(n log n), только оптимальнее за счёт кэш-локальности.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut result = values.to_vec();
    result.sort_unstable();
    result.dedup();
    result
}

/// Линейная реализация с мемоизацией
pub fn slow_fib(n: u64) -> u64 {
    match n {
        x if x <= 1 => x,
        _ => {
            let mut a = 0;
            let mut b = 1;
            for i in 2..=n {
                let next = a + b;
                a = b;
                b = next;
            }
            b
        }
    }
}
