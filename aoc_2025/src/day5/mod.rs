fn process_input<T>(item: Option<T>) -> Result<Vec<String>, anyhow::Error>
where
    T: AsRef<str>,
{
    let item = item.ok_or_else(|| anyhow::anyhow!("Parsing failed"))?;
    Ok(item
        .as_ref()
        .split_whitespace()
        .map(|s| s.to_string())
        .collect())
}

pub fn dayfive_partone(input: &str) -> Result<usize, anyhow::Error> {
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let mut input = input.iter();
    let ranges = process_input(input.next())?;
    let items = process_input(input.next())?;
    dbg!(ranges, items);
    todo!()
}
