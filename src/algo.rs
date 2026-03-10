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

fn fib_internal(n: u64, cache: &mut std::collections::HashMap<u64, u64>) -> u64 {
    if let Some(&val) = cache.get(&n) {
        return val;
    }
    let val = fib_internal(n - 1, cache) + fib_internal(n - 2, cache);
    cache.insert(n, val);
    val
}
