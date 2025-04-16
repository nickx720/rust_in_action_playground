pub fn md5() {
    let s: Vec<u32> = vec![
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5,
        9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10,
        15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    let mut k: Vec<u32> = Vec::new();
    //https://en.wikipedia.org/wiki/MD5?utm_source=substack&utm_medium=email#Algorithm
    for i in 0..=63usize {
        let sin = (i + 1) as f32;
        let abs = sin.sin().abs();
        let floored = (2f32.powf(32f32) * abs).floor();
        k.push(floored as u32)
    }
    dbg!(k[1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_md5() {
        md5();
        assert_eq!(1, 1);
    }
}
