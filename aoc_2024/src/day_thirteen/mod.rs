pub mod part_one;
pub mod part_two;
pub fn extended_euclid(point_a: i64, point_b: i64) -> (i64, i64, i64) {
    if point_b == 0 {
        (point_a, 1, 0)
    } else {
        let (d, x, y) = extended_euclid(point_b, point_a % point_b);
        (d, y, x - (point_a / point_b) * y)
    }
}

pub fn star_algo_parser(input: &str, flag_two: bool) -> i64 {
    let press_limit = if flag_two { i64::MAX } else { 100 };
    let offset = if flag_two { 10000000000000 } else { 0 };

    regex::Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)",
    )
    .unwrap()
    .captures_iter(&input)
    .map(|cap| {
        let mut nums = cap
            .iter()
            .skip(1)
            .map(|x| x.unwrap().as_str().parse::<i64>().unwrap());
        let (abutton_x, abutton_y, bbutton_x, bbutton_y, prize_x, prize_y) = (
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap(),
            nums.next().unwrap() + offset,
            nums.next().unwrap() + offset,
        );

        let denominator = abutton_x * bbutton_y - abutton_y * bbutton_x;

        if denominator != 0 {
            let numerator_a = bbutton_y * prize_x - bbutton_x * prize_y;
            let numerator_b = abutton_x * prize_y - abutton_y * prize_x;
            let presses_a = numerator_a / denominator;
            let presses_b = numerator_b / denominator;

            if numerator_a % denominator == 0
                && numerator_b % denominator == 0
                && presses_a >= 0
                && presses_b >= 0
                && presses_a <= press_limit
                && presses_b <= press_limit
            {
                presses_a * 3 + presses_b
            } else {
                0
            }
        } else if abutton_x * prize_y != abutton_y * prize_x {
            0
        } else {
            let (a_x, b_x, p_x) = if prize_x != 0 {
                (abutton_x, bbutton_x, prize_x)
            } else {
                (abutton_y, bbutton_y, prize_y)
            };

            let (gcd, a_0, b_0) = extended_euclid(a_x, b_x);
            if (p_x % gcd) != 0 {
                return 0;
            }
            let a_x_simplified = a_x / gcd;
            let b_x_simplified = b_x / gcd;
            let p_x_simplified = p_x / gcd;
            let a_p = a_0 * p_x_simplified;
            let b_p = b_0 * p_x_simplified;

            let max_presses_a = std::cmp::min(p_x / a_x, press_limit);
            let max_presses_b = std::cmp::min(p_x / b_x, press_limit);
            let min_k = -a_p.div_euclid(b_x_simplified);
            let max_k = b_p.div_euclid(a_x_simplified);
            let min_presses_a = a_p + b_x_simplified * min_k;
            let min_presses_b = b_p - a_x_simplified * max_k;
            let max_k_2 = std::cmp::min(
                min_k + (max_presses_a - min_presses_a).div_euclid(b_x_simplified),
                max_k,
            );
            let min_k_2 = std::cmp::max(
                max_k - (max_presses_b - min_presses_b).div_euclid(a_x_simplified),
                min_k,
            );
            if min_k_2 > max_k_2 {
                return 0;
            }

            let k = if a_x > b_x * 3 { max_k_2 } else { min_k_2 };

            let presses_a = a_p + b_x_simplified * k;
            let presses_b = b_p - a_x_simplified * k;

            presses_a * 3 + presses_b
        }
    })
    .sum()
}
