use std::collections::HashMap;

pub mod part_one;
pub mod part_two;

pub fn blink_counter(num: usize, count: usize, seen: &mut HashMap<(usize, usize), usize>) -> usize {
    if count == 0 {
        return 1;
    }
    if let Some(&value) = seen.get(&(num, count)) {
        return value;
    }
    let result = match num {
        0 => blink_counter(1, count - 1, seen),
        _ => {
            let ndigits = num.ilog10() + 1;
            if ndigits % 2 == 0 {
                let mask = 10usize.pow(ndigits / 2);
                let left = num / mask;
                let right = num % mask;
                blink_counter(left, count - 1, seen) + blink_counter(right, count - 1, seen)
            } else {
                blink_counter(num * 2024, count - 1, seen)
            }
        }
    };
    seen.insert((num, count), result);
    result
}
