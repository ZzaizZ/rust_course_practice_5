pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
pub fn sum_even(values: &[i64]) -> i64 {
    values.iter().filter(|&&v| v % 2 == 0).sum()
}

/// Подсчёт ненулевых байтов.
pub fn count_nonzero_bytes(input: &[u8]) -> usize {
    input.iter().filter(|&&b| b != 0_u8).count()
}

/// Нормализация строки: удаляем все пробельные символы (пробелы, табуляции и т.д.)
/// и приводим к нижнему регистру.
pub fn normalize(input: &str) -> String {
    input.split_whitespace().collect::<String>().to_lowercase()
}

/// Усреднение только положительных чисел. Если нет положительных, возвращаем 0.ы
pub fn average_positive(values: &[i64]) -> f64 {
    let mut count_positive = 0;
    let sum: i64 = values
        .iter()
        .filter(|&&v| {
            if v > 0 {
                count_positive += 1;
                true
            } else {
                false
            }
        })
        .sum();
    if count_positive == 0 {
        return 0.0;
    }
    sum as f64 / count_positive as f64
}

/// Use-after-free: возвращает значение после освобождения бокса.
/// UB, проявится под ASan/Miri.
pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let raw = Box::into_raw(b);
    let val = *raw;
    drop(Box::from_raw(raw));
    val + *raw
}
