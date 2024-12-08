pub mod part_one;
pub mod part_two;
pub fn validate(current: usize, numbers: &Vec<usize>, length: usize, part_two: bool) -> bool {
    if length == 0 {
        return current == numbers[0];
    }
    if part_two && current > numbers[length] {
        let digits = 10usize.pow(numbers[length].checked_ilog10().unwrap_or(0) + 1);
        if (current - numbers[length]) % digits == 0
            && validate(current / digits, numbers, length - 1, part_two)
        {
            return true;
        }
    }
    if current % numbers[length] == 0
        && validate(current / numbers[length], numbers, length - 1, part_two)
    {
        return true;
    }
    if current >= numbers[length]
        && validate(current - numbers[length], numbers, length - 1, part_two)
    {
        return true;
    }
    false
}
