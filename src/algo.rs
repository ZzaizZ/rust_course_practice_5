/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut set = std::collections::BTreeSet::<u64>::new();
    for v in values {
        set.insert(*v);
    }
    set.into_iter().collect()
}

/// Рекурсивная реализация с мемоизацией
pub fn slow_fib(n: u64) -> u64 {
    let mut cache = std::collections::HashMap::<u64, u64>::with_capacity(n as usize + 1);
    cache.insert(0, 0);
    cache.insert(1, 1);
    fib_internal(n, &mut cache)
}

fn fib_internal(n: u64, cache: &mut std::collections::HashMap<u64, u64>) -> u64 {
    if let Some(&val) = cache.get(&n) {
        return val;
    }
    let val = fib_internal(n - 1, cache) + fib_internal(n - 2, cache);
    cache.insert(n, val);
    val
}
