use anyhow::Context;

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
    let ranges: Vec<(usize, usize)> = process_input(input.next())?
        .into_iter()
        .map(|item| {
            Ok::<Vec<(usize, usize)>, anyhow::Error>(
                item.split("-")
                    .map(|value| {
                        value
                            .parse::<usize>()
                            .with_context(|| format!("Conversion failed"))
                    })
                    .collect::<Result<Vec<_>, _>>()?
                    .chunks(2)
                    .map(|chunk| (chunk[0], chunk[1]))
                    .collect::<Vec<(usize, usize)>>(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .flatten()
        .collect();
    let items = process_input(input.next())?;
    dbg!(ranges, items);
    todo!()
}
